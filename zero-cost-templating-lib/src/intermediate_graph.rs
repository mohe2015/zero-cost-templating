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
    Variable {
        before: String,
        variable_name: String,
        escaping_fun: EscapingFunction,
        after: String,
    },
    Text(String),
    /// The part we want to render when a partial block occurs.
    PartialBlockPartial,
    /// The inner template that we want to render
    InnerTemplate,
}

impl IntermediateAstElement {
    #[must_use]
    pub const fn variable(&self) -> Option<(&String, &EscapingFunction)> {
        if let Self::Variable {
            variable_name,
            escaping_fun,
            ..
        } = self
        {
            Some((variable_name, escaping_fun))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn variable_name(&self) -> Option<&String> {
        if let Self::Variable { variable_name, .. } = self {
            Some(variable_name)
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
            Self::Variable {
                before,
                variable_name,
                escaping_fun,
                after,
            } => {
                write!(
                    formatter,
                    "{before}{{{{{variable_name}:{escaping_fun}}}}}{after}"
                )
            }
            Self::Text(text) => {
                write!(formatter, "{text}")
            }
            Self::PartialBlockPartial => write!(formatter, "partial"),
            Self::InnerTemplate => write!(formatter, "template"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    tmp: Vec<(NodeIndex, IntermediateAstElement)>,
    edge_type: IntermediateAstElement,
    node: TemplateNode,
) -> Vec<(NodeIndex, IntermediateAstElement)> {
    todo!()
    /*
    match (&node.node_type, current, edge_type) {
        (
            NodeType::Other,
            IntermediateAstElement::Text(old),
            IntermediateAstElement::Variable {
                before,
                variable_name,
                escaping_fun,
                after,
            },
        ) => (
            last,
            IntermediateAstElement::Variable {
                before: old + &before,
                variable_name,
                escaping_fun,
                after,
            },
        ),
        (
            NodeType::Other,
            IntermediateAstElement::Variable {
                before,
                variable_name,
                escaping_fun,
                after,
            },
            IntermediateAstElement::Text(new),
        ) => (
            last,
            IntermediateAstElement::Variable {
                before,
                variable_name,
                escaping_fun,
                after: after + &new,
            },
        ),
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
    */
}

pub fn flush_pending_edge(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    tmp: Vec<(NodeIndex, IntermediateAstElement)>,
    node: TemplateNode,
) -> Vec<(NodeIndex, IntermediateAstElement)> {
    todo!()
    /*
    match current {
        IntermediateAstElement::Noop => (last, IntermediateAstElement::Noop),
        current @ (IntermediateAstElement::Variable { .. }
        | IntermediateAstElement::Text(_)
        | IntermediateAstElement::PartialBlockPartial
        | IntermediateAstElement::InnerTemplate) => {
            let current_node = graph.add_node(node);
            graph.add_edge(last, current_node, current);
            (current_node, IntermediateAstElement::Noop)
        }
    }
    */
}

#[must_use]
#[expect(clippy::too_many_lines, reason = "tmp")]
pub fn children_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: &str,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut tmp: Vec<(NodeIndex, IntermediateAstElement)>,
    input: Vec<Child>,
    parent: &str,
) -> Vec<(NodeIndex, IntermediateAstElement)> {
    for child in input {
        match child {
            Child::Variable(next_variable) => {
                // https://html.spec.whatwg.org/dev/syntax.html
                // https://github.com/cure53/DOMPurify/blob/main/src/tags.js
                let escaping_fun = match parent {
                    "h1" | "li" | "span" | "title" | "main" => EscapingFunction::HtmlElementInner,
                    other => panic!("unknown escaping rules for element {other}"),
                };
                tmp = add_node_with_edge(
                    graph,
                    tmp,
                    IntermediateAstElement::Variable {
                        before: String::new(),
                        variable_name: next_variable,
                        escaping_fun,
                        after: String::new(),
                    },
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
            }
            Child::Literal(string) => {
                tmp = add_node_with_edge(
                    graph,
                    tmp,
                    IntermediateAstElement::Text(string),
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
            }
            Child::Element(element) => {
                assert!(
                    !(parent == "script" || parent == "style"),
                    "children are unsafe in <script> and <style>"
                );
                tmp = element_to_ast(first_nodes, template_name, graph, tmp, element);
            }
            Child::Each(_identifier, children) => {
                tmp = flush_pending_edge(
                    graph,
                    tmp,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
                let loop_start = tmp;
                tmp = children_to_ast(first_nodes, template_name, graph, tmp, children, parent);

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
                    let inner_tmp = children_to_ast(
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
                        inner_tmp,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::Other,
                        },
                    );
                    partial_block_partial
                };

                let inner_template;
                tmp = add_node_with_edge(
                    graph,
                    tmp,
                    IntermediateAstElement::Noop,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::InnerTemplate,
                    },
                );

                graph.add_edge(
                    inner_template,
                    partial_block_partial,
                    IntermediateAstElement::PartialBlockPartial,
                );

                let inner_template_target = *first_nodes
                    .get(&name)
                    .unwrap_or_else(|| panic!("unknown inner template {name}"));

                graph.add_edge(
                    inner_template,
                    inner_template_target,
                    IntermediateAstElement::InnerTemplate,
                );

                last = inner_template;

                // It should be a Vec<(last, current)> because
                // then a simple if else could be optimized too two nodes with a double edge.

                let current_node = graph.add_node(TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                });
                graph.add_edge(last, current_node, current);
                current = IntermediateAstElement::Noop;
                last = current_node;
            }
            Child::PartialBlockPartial => {
                tmp = add_node_with_edge(
                    graph,
                    tmp,
                    IntermediateAstElement::Noop,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::PartialBlock,
                    },
                );

                tmp = add_node_with_edge(
                    graph,
                    tmp,
                    IntermediateAstElement::Noop,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
            }
            Child::If(_variable, if_children, else_children) => {
                tmp = flush_pending_edge(
                    graph,
                    tmp,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );

                let if_tmp =
                    children_to_ast(first_nodes, template_name, graph, tmp, if_children, parent);

                let else_tmp = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    tmp,
                    else_children,
                    parent,
                );

                tmp = if_tmp;
                tmp.append(&mut else_tmp);
            }
        }
    }
    tmp
}

#[must_use]
#[expect(clippy::too_many_lines, reason = "tmp")]
pub fn element_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: &str,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut tmp: Vec<(NodeIndex, IntermediateAstElement)>,
    input: Element,
) -> Vec<(NodeIndex, IntermediateAstElement)> {
    let name = input.name;
    tmp = add_node_with_edge(
        graph,
        tmp,
        IntermediateAstElement::Text(format!("<{name}")),
        TemplateNode {
            template_name: template_name.to_owned(),
            node_type: NodeType::Other,
        },
    );
    for attribute in input.attributes {
        if let Some(value) = attribute.value {
            tmp = add_node_with_edge(
                graph,
                tmp,
                IntermediateAstElement::Text(format!(r#" {}=""#, attribute.key)),
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
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
                        tmp = add_node_with_edge(
                            graph,
                            tmp,
                            IntermediateAstElement::Variable {
                                before: String::new(),
                                variable_name: next_variable,
                                escaping_fun,
                                after: String::new(),
                            },
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                        );
                    }
                    AttributeValuePart::Literal(string) => {
                        tmp = add_node_with_edge(
                            graph,
                            tmp,
                            IntermediateAstElement::Text(string),
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                        );
                    }
                }
            }
            tmp = add_node_with_edge(
                graph,
                tmp,
                IntermediateAstElement::Text(r#"""#.to_owned()),
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        } else {
            tmp = add_node_with_edge(
                graph,
                tmp,
                IntermediateAstElement::Text(format!(r#" {}"#, attribute.key)),
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        }
    }
    tmp = add_node_with_edge(
        graph,
        tmp,
        IntermediateAstElement::Text(">".to_owned()),
        TemplateNode {
            template_name: template_name.to_owned(),
            node_type: NodeType::Other,
        },
    );
    tmp = children_to_ast(
        first_nodes,
        template_name,
        graph,
        tmp,
        input.children,
        &name,
    );
    // https://html.spec.whatwg.org/dev/syntax.html#void-elements
    match name.to_ascii_lowercase().as_str() {
        "!doctype" | "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
        | "meta" | "source" | "track" | "wbr" => {}
        _ => {
            tmp = add_node_with_edge(
                graph,
                tmp,
                IntermediateAstElement::Text(format!("</{name}>")),
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        }
    }
    tmp
}
