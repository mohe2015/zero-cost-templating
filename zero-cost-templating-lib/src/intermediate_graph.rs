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
            Self::Variable(variable, escaping_fun) => {
                write!(formatter, "{{{{{variable}:{escaping_fun}}}}}")
            }
            Self::Text(text) => {
                write!(formatter, "{text}")
            }
            Self::Noop => {
                write!(formatter, "noop")
            }
            Self::PartialBlockPartial => write!(formatter, "partial"),
            Self::InnerTemplate => write!(formatter, "template"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    PartialBlock,
    InnerTemplate,
    Other,
}

impl Display for NodeType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PartialBlock => write!(formatter, "partial"),
            Self::InnerTemplate => write!(formatter, "inner"),
            Self::Other => write!(formatter, "other"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateNode {
    pub template_name: String,
    pub node_type: NodeType,
}

impl Display for TemplateNode {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{} {}", self.template_name, self.node_type)
    }
}

pub fn add_node_with_edge(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    last: NodeIndex,
    current: IntermediateAstElement,
    node: TemplateNode,
    edge_type: IntermediateAstElement,
) -> (NodeIndex, IntermediateAstElement) {
    match (&node.node_type, current, edge_type) {
        (NodeType::Other, IntermediateAstElement::Text(old), IntermediateAstElement::Text(new)) => {
            (last, IntermediateAstElement::Text(old + &new))
        }
        (NodeType::Other, IntermediateAstElement::Noop, edge_type) => (last, edge_type),
        (_, current, edge_type) => {
            let current_node = graph.add_node(node);
            graph.add_edge(last, current_node, current);
            (current_node, edge_type)
        }
    }
}

pub fn flush_pending_edge(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    last: NodeIndex,
    current: IntermediateAstElement,
    node: TemplateNode,
) -> (NodeIndex, IntermediateAstElement) {
    match current {
        IntermediateAstElement::Noop => (last, IntermediateAstElement::Noop),
        current @ (IntermediateAstElement::Variable(..)
        | IntermediateAstElement::Text(_)
        | IntermediateAstElement::PartialBlockPartial
        | IntermediateAstElement::InnerTemplate) => {
            let current_node = graph.add_node(node);
            graph.add_edge(last, current_node, current);
            (current_node, IntermediateAstElement::Noop)
        }
    }
}

#[must_use]
#[expect(clippy::too_many_lines, reason = "tmp")]
pub fn children_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: &str,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut last: NodeIndex,
    mut current: IntermediateAstElement,
    input: Vec<Child>,
    parent: &str,
) -> (NodeIndex, IntermediateAstElement) {
    for child in input {
        match child {
            Child::Variable(next_variable) => {
                // https://html.spec.whatwg.org/dev/syntax.html
                // https://github.com/cure53/DOMPurify/blob/main/src/tags.js
                let escaping_fun = match parent {
                    "h1" | "li" | "span" | "title" | "main" => EscapingFunction::HtmlElementInner,
                    other => panic!("unknown escaping rules for element {other}"),
                };
                (last, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Variable(next_variable, escaping_fun),
                );
            }
            Child::Literal(string) => {
                (last, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
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
                (last, current) =
                    element_to_ast(first_nodes, template_name, graph, last, current, element);
            }
            Child::Each(_identifier, children) => {
                (last, current) = flush_pending_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
                let loop_start = last;
                (last, current) = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    last,
                    current,
                    children,
                    parent,
                );

                graph.add_edge(last, loop_start, current);
                current = IntermediateAstElement::Noop;

                last = loop_start;
            }
            Child::PartialBlock(name, children) => {
                let partial_block_partial = {
                    // this part needs to be fully disjunct from the rest
                    let partial_block_partial = graph.add_node(TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    });
                    let (inner_last, inner_current) = children_to_ast(
                        first_nodes,
                        template_name,
                        graph,
                        partial_block_partial,
                        IntermediateAstElement::Noop,
                        children,
                        parent,
                    );
                    flush_pending_edge(
                        graph,
                        inner_last,
                        inner_current,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::Other,
                        },
                    );
                    partial_block_partial
                };

                let inner_template;
                (inner_template, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::InnerTemplate,
                    },
                    IntermediateAstElement::Noop,
                );

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
                // that guarantee is needed to tell the template what the after node is
                // TODO FIXME maybe add a special current == NoopForceFlush
                // It should be a Vec<(last, current)> because then a simple if else could be optimized two two nodes with a double edge.
                (last, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Noop,
                );
            }
            Child::PartialBlockPartial => {
                (last, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::PartialBlock,
                    },
                    IntermediateAstElement::Noop,
                );

                // This is needed so e.g. branching doesn't break the guarantee that
                // there is exactly one successor node after InnerTemplate
                // that guarantee is needed to tell the template what the after node is
                // TODO FIXME maybe add a special current == NoopForceFlush
                (last, current) = add_node_with_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                    IntermediateAstElement::Noop,
                );
            }
            Child::If(_variable, if_children, else_children) => {
                (last, current) = flush_pending_edge(
                    graph,
                    last,
                    current,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );

                let if_last = {
                    let (mut if_last, if_current) = children_to_ast(
                        first_nodes,
                        template_name,
                        graph,
                        last,
                        IntermediateAstElement::Noop,
                        if_children,
                        parent,
                    );
                    (if_last, _) = flush_pending_edge(
                        graph,
                        if_last,
                        if_current,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::Other,
                        },
                    );
                    if_last
                };

                let else_last = {
                    let (mut else_last, else_current) = children_to_ast(
                        first_nodes,
                        template_name,
                        graph,
                        last,
                        IntermediateAstElement::Noop,
                        else_children,
                        parent,
                    );
                    (else_last, _) = flush_pending_edge(
                        graph,
                        else_last,
                        else_current,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::Other,
                        },
                    );
                    else_last
                };

                // TODO FIXME if last would be a vec, then we probably wouldn't need this
                last = graph.add_node(TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                });

                graph.add_edge(if_last, last, IntermediateAstElement::Noop);
                graph.add_edge(else_last, last, IntermediateAstElement::Noop);
            }
        }
    }
    (last, current)
}

#[must_use]
#[expect(clippy::too_many_lines, reason = "tmp")]
pub fn element_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: &str,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut last: NodeIndex,
    mut current: IntermediateAstElement,
    input: Element,
) -> (NodeIndex, IntermediateAstElement) {
    let name = input.name;
    (last, current) = add_node_with_edge(
        graph,
        last,
        current,
        TemplateNode {
            template_name: template_name.to_owned(),
            node_type: NodeType::Other,
        },
        IntermediateAstElement::Text(format!("<{name}")),
    );
    for attribute in input.attributes {
        if let Some(value) = attribute.value {
            (last, current) = add_node_with_edge(
                graph,
                last,
                current,
                TemplateNode {
                    template_name: template_name.to_owned(),
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
                        (last, current) = add_node_with_edge(
                            graph,
                            last,
                            current,
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                            IntermediateAstElement::Variable(next_variable, escaping_fun),
                        );
                    }
                    AttributeValuePart::Literal(string) => {
                        (last, current) = add_node_with_edge(
                            graph,
                            last,
                            current,
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                            IntermediateAstElement::Text(string),
                        );
                    }
                }
            }
            (last, current) = add_node_with_edge(
                graph,
                last,
                current,
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(r#"""#.to_owned()),
            );
        } else {
            (last, current) = add_node_with_edge(
                graph,
                last,
                current,
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(format!(r#" {}"#, attribute.key)),
            );
        }
    }
    (last, current) = add_node_with_edge(
        graph,
        last,
        current,
        TemplateNode {
            template_name: template_name.to_owned(),
            node_type: NodeType::Other,
        },
        IntermediateAstElement::Text(">".to_owned()),
    );
    (last, current) = children_to_ast(
        first_nodes,
        template_name,
        graph,
        last,
        current,
        input.children,
        &name,
    );
    // https://html.spec.whatwg.org/dev/syntax.html#void-elements
    match name.to_ascii_lowercase().as_str() {
        "!doctype" | "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
        | "meta" | "source" | "track" | "wbr" => {}
        _ => {
            (last, current) = add_node_with_edge(
                graph,
                last,
                current,
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
                IntermediateAstElement::Text(format!("</{name}>")),
            );
        }
    }
    (last, current)
}
