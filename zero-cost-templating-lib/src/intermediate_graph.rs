use core::fmt::Display;
use std::collections::HashMap;

use petgraph::stable_graph::{NodeIndex, StableGraph};

use crate::html_recursive_descent::{AttributeValuePart, Child, Element};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum EscapingFunction {
    HtmlAttribute,
    HtmlElementInner,
}

impl Display for EscapingFunction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::HtmlAttribute => write!(formatter, "attr"),
            Self::HtmlElementInner => write!(formatter, "element"),
        }
    }
}

// first variable then text, so we can print as much as possible
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum IntermediateAstElement {
    Variable(String, EscapingFunction),
    Text(String),
    Noop,
    /// The part we want to render when a partial block occurs.
    PartialBlockPartial,
    /// The inner template that we want to render
    InnerTemplate,
}

impl IntermediateAstElement {
    #[must_use]
    pub const fn variable(&self) -> Option<(&String, &EscapingFunction)> {
        if let Self::Variable(name, escaping_fun) = self {
            Some((name, escaping_fun))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn variable_name(&self) -> Option<&String> {
        if let Self::Variable(name, _) = self {
            Some(name)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn text(&self) -> Option<&String> {
        if let Self::Text(string) = self {
            Some(string)
        } else {
            None
        }
    }
}

impl Display for IntermediateAstElement {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IntermediateAstElement::Variable(variable, escaping_fun) => {
                write!(formatter, "{{{{{variable}:{escaping_fun}}}}}")?;
            }
            IntermediateAstElement::Text(text) => {
                write!(formatter, "{text}")?;
            }
            IntermediateAstElement::Noop => {
                write!(formatter, "noop")?;
            }
            IntermediateAstElement::PartialBlockPartial => write!(formatter, "partial")?,
            IntermediateAstElement::InnerTemplate => write!(formatter, "inner_template")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    PartialBlock,
    InnerTemplate,
    Other,
}

#[derive(Debug, Clone)]
pub struct TemplateNode {
    pub template_name: String,
    pub node_type: NodeType,
}

pub fn add_node_with_edge(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    last: NodeIndex,
    node: TemplateNode,
    edge_type: IntermediateAstElement,
) -> NodeIndex {
    let current = graph.add_node(node);
    graph.add_edge(last, current, edge_type);
    current
}

#[must_use]
pub fn children_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: String,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut last: NodeIndex,
    input: Vec<Child>,
    parent: &str,
) -> NodeIndex {
    for child in input {
        match child {
            Child::Variable(next_variable) => {
                // https://html.spec.whatwg.org/dev/syntax.html
                // https://github.com/cure53/DOMPurify/blob/main/src/tags.js
                let escaping_fun = match parent {
                    "h1" | "li" | "span" | "title" | "main" => EscapingFunction::HtmlElementInner,
                    other => panic!("unknown escaping rules for element {other}"),
                };
                last = add_node_with_edge(
                    graph,
                    last,
                    TemplateNode {
                        template_name,
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Variable(next_variable, escaping_fun),
                );
            }
            Child::Literal(string) => {
                last = add_node_with_edge(
                    graph,
                    last,
                    TemplateNode {
                        template_name,
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Text(string),
                );
            }
            Child::Element(element) => {
                assert!(
                    !(parent == "script" || parent == "style"),
                    "children are unsafe in <script> and <style>"
                );
                last = element_to_ast(first_nodes, template_name, graph, last, element);
            }
            Child::Each(_identifier, children) => {
                let loop_start = last;
                last = children_to_ast(first_nodes, template_name, graph, last, children, parent);
                graph.add_edge(last, loop_start, IntermediateAstElement::Noop);
                last = loop_start;
            }
            Child::PartialBlock(name, children) => {
                let partial_block_partial = graph.add_node(TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                });
                let _partial_block_partial_end = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    partial_block_partial,
                    children,
                    parent,
                );

                let inner_template = graph.add_node(TemplateNode {
                    template_name,
                    node_type: NodeType::InnerTemplate,
                });

                graph.add_edge(last, inner_template, IntermediateAstElement::Noop);

                graph.add_edge(
                    inner_template,
                    partial_block_partial,
                    IntermediateAstElement::PartialBlockPartial,
                );

                let inner_template_target = *first_nodes.get(&name).unwrap();

                graph.add_edge(
                    inner_template,
                    inner_template_target,
                    IntermediateAstElement::InnerTemplate,
                );

                last = inner_template;

                // This is needed so e.g. branching doesn't break the guarantee that
                // there is exactly one successor node after InnerTemplate
                last = add_node_with_edge(
                    graph,
                    last,
                    TemplateNode {
                        template_name,
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Noop,
                );
            }
            Child::PartialBlockPartial => {
                last = add_node_with_edge(
                    graph,
                    last,
                    TemplateNode {
                        template_name,
                        node_type: NodeType::PartialBlock,
                    },
                    IntermediateAstElement::Noop,
                );

                // This is needed so e.g. branching doesn't break the guarantee that
                // there is exactly one successor node after PartialBlock
                last = add_node_with_edge(
                    graph,
                    last,
                    TemplateNode {
                        template_name,
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Noop,
                );
            }
            Child::If(_variable, if_children, else_children) => {
                let if_last =
                    children_to_ast(first_nodes, template_name, graph, last, if_children, parent);

                let else_last = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    last,
                    else_children,
                    parent,
                );

                last = graph.add_node(TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                });

                graph.add_edge(if_last, last, IntermediateAstElement::Noop);
                graph.add_edge(else_last, last, IntermediateAstElement::Noop);
            }
        }
    }
    last
}

#[must_use]
pub fn element_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: String,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut last: NodeIndex,
    input: Element,
) -> NodeIndex {
    let name = input.name;
    last = add_node_with_edge(
        graph,
        last,
        TemplateNode {
            template_name,
            node_type: NodeType::Other,
        },
        IntermediateAstElement::Text(format!("<{name}")),
    );
    for attribute in input.attributes {
        if let Some(value) = attribute.value {
            last = add_node_with_edge(
                graph,
                last,
                TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(format!(r#" {}=""#, attribute.key)),
            );
            for value_part in value {
                match value_part {
                    AttributeValuePart::Variable(next_variable) => {
                        // https://html.spec.whatwg.org/dev/syntax.html
                        // https://github.com/cure53/DOMPurify/blob/main/src/attrs.js
                        let escaping_fun = match (name.as_str(), attribute.key.as_str()) {
                            (_, "value" | "class") => EscapingFunction::HtmlAttribute,
                            (name, attr) => panic!(
                                "in element {name}, unknown escaping rules for attribute name \
                                 {attr}"
                            ),
                        };
                        last = add_node_with_edge(
                            graph,
                            last,
                            TemplateNode {
                                template_name,
                                node_type: NodeType::Other,
                            },
                            IntermediateAstElement::Variable(next_variable, escaping_fun),
                        );
                    }
                    AttributeValuePart::Literal(string) => {
                        last = add_node_with_edge(
                            graph,
                            last,
                            TemplateNode {
                                template_name,
                                node_type: NodeType::Other,
                            },
                            IntermediateAstElement::Text(string),
                        );
                    }
                }
            }
            last = add_node_with_edge(
                graph,
                last,
                TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(r#"""#.to_owned()),
            );
        } else {
            last = add_node_with_edge(
                graph,
                last,
                TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(format!(r#" {}"#, attribute.key)),
            );
        }
    }
    last = add_node_with_edge(
        graph,
        last,
        TemplateNode {
            template_name,
            node_type: NodeType::Other,
        },
        IntermediateAstElement::Text(">".to_owned()),
    );
    last = children_to_ast(
        first_nodes,
        template_name,
        graph,
        last,
        input.children,
        &name,
    );
    // https://html.spec.whatwg.org/dev/syntax.html#void-elements
    match name.to_ascii_lowercase().as_str() {
        "!doctype" | "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
        | "meta" | "source" | "track" | "wbr" => {}
        _ => {
            last = add_node_with_edge(
                graph,
                last,
                TemplateNode {
                    template_name,
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(format!("</{name}>")),
            );
        }
    }
    last
}
