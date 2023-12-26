use core::fmt::Display;
use std::collections::HashMap;

use petgraph::{
    data::Build,
    stable_graph::{NodeIndex, StableGraph},
};

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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct IntermediateAstElement {
    /// The tag to distinguish multiple outgoing nodes. E.g. `true` and `false` for an if.
    tag: String,
    inner: IntermediateAstElementInner,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum IntermediateAstElementInner {
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
        if let Self {
            inner:
                IntermediateAstElementInner::Variable {
                    variable_name,
                    escaping_fun,
                    ..
                },
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
        if let Self {
            inner: IntermediateAstElementInner::Variable { variable_name, .. },
            ..
        } = self
        {
            Some(variable_name)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn text(&self) -> Option<&String> {
        if let Self {
            inner: IntermediateAstElementInner::Text(string),
            ..
        } = self
        {
            Some(string)
        } else {
            None
        }
    }
}

impl Display for IntermediateAstElement {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self {
                inner:
                    IntermediateAstElementInner::Variable {
                        before,
                        variable_name,
                        escaping_fun,
                        after,
                    },
                ..
            } => {
                write!(
                    formatter,
                    "{before}{{{{{variable_name}:{escaping_fun}}}}}{after}"
                )
            }
            Self {
                inner: IntermediateAstElementInner::Text(text),
                ..
            } => {
                write!(formatter, "{text}")
            }
            Self {
                inner: IntermediateAstElementInner::PartialBlockPartial,
                ..
            } => write!(formatter, "partial"),
            Self {
                inner: IntermediateAstElementInner::InnerTemplate,
                ..
            } => write!(formatter, "template"),
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

// Normal use of the library should create few nodes and few necessary calls.
// But edge cases should not all be optimized if it makes the code ugly etc.
// Maybe in some future version add the full path to the graph including inner template stuff?
// maybe for inner templates add an edge layer or
// something like that for edges that don't exist for all users
// probably not a good idea because of generic programming
// the end of the partial could point to the after partial node
// (no doesn't work if partial is used multiple times)

/// Adds the node in all cases if it is not NodeType::Other.
/// If it is NodeType::Other only adds it if there are pending outgoing edges
/// (even not added if current node type is not NodeType::Other).
// Two partials after each other...
pub fn flush_with_node(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    tmp: Vec<(NodeIndex, Option<IntermediateAstElement>)>,
    node: TemplateNode,
) -> NodeIndex {
    assert!(!tmp.is_empty());
    if tmp.len() == 1 && tmp[0].1.is_none() && graph[tmp[0].0].node_type == node.node_type {
        return tmp[0].0;
    }
    let to = graph.add_node(node);
    for (from, edge) in tmp {
        // TODO FIXME maybe just unwrap?
        graph.add_edge(
            from,
            to,
            edge.unwrap_or_else(|| IntermediateAstElement {
                tag: String::new(),
                inner: IntermediateAstElementInner::Text(String::new()),
            }),
        );
    }
    to
}

pub fn connect_edges_to_node(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    tmp: Vec<(NodeIndex, Option<IntermediateAstElement>)>,
    to: NodeIndex,
) {
    for (from, edge) in tmp {
        // TODO FIXME maybe just unwrap?
        graph.add_edge(
            from,
            to,
            edge.unwrap_or_else(|| IntermediateAstElement {
                tag: String::new(),
                inner: IntermediateAstElementInner::Text(String::new()),
            }),
        );
    }
}

/// Adds the edge in all cases.
/// If adding the edge requires a new node, it adds the node of the specified type.
pub fn add_edge_maybe_with_node(
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut tmp: Vec<(NodeIndex, Option<IntermediateAstElement>)>,
    edge_type: IntermediateAstElement,
    node: TemplateNode,
) -> Vec<(NodeIndex, Option<IntermediateAstElement>)> {
    //let new_node = None;
    for (from, edge) in tmp.iter_mut() {
        todo!();
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
            (NodeType::Other, IntermediateAstElement::Text(old),
            IntermediateAstElement::Text(new)) => {
                (last, IntermediateAstElement::Text(old + &new))
            }
            (NodeType::Other, IntermediateAstElement::Noop, edge_type) => (last, edge_type),
            (_, current, edge_type) => {
                let current_node = graph.add_node(node);
                graph.add_edge(last, current_node, current);
                (current_node, edge_type)
            }
        }*/
    }
    tmp
}

#[must_use]
#[expect(clippy::too_many_lines, reason = "tmp")]
pub fn children_to_ast(
    first_nodes: &HashMap<String, NodeIndex>,
    template_name: &str,
    graph: &mut StableGraph<TemplateNode, IntermediateAstElement>,
    mut tmp: Vec<(NodeIndex, Option<IntermediateAstElement>)>,
    input: Vec<Child>,
    parent: &str,
) -> Vec<(NodeIndex, Option<IntermediateAstElement>)> {
    for child in input {
        match child {
            Child::Variable(next_variable) => {
                // https://html.spec.whatwg.org/dev/syntax.html
                // https://github.com/cure53/DOMPurify/blob/main/src/tags.js
                let escaping_fun = match parent {
                    "h1" | "li" | "span" | "title" | "main" => EscapingFunction::HtmlElementInner,
                    other => panic!("unknown escaping rules for element {other}"),
                };
                tmp = add_edge_maybe_with_node(
                    graph,
                    tmp,
                    IntermediateAstElement {
                        tag: String::new(),
                        inner: IntermediateAstElementInner::Variable {
                            before: String::new(),
                            variable_name: next_variable,
                            escaping_fun,
                            after: String::new(),
                        },
                    },
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
            }
            Child::Literal(string) => {
                tmp = add_edge_maybe_with_node(
                    graph,
                    tmp,
                    IntermediateAstElement {
                        tag: String::new(),
                        inner: IntermediateAstElementInner::Text(string),
                    },
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
                if !children.is_empty() {
                    let loop_start = flush_with_node(
                        graph,
                        tmp,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::Other,
                        },
                    );
                    let loop_end = children_to_ast(
                        first_nodes,
                        template_name,
                        graph,
                        vec![(loop_start, None)],
                        children,
                        parent,
                    );

                    connect_edges_to_node(graph, loop_end, loop_start);

                    tmp = vec![(loop_start, None)];
                }
            }
            Child::PartialBlock(name, children) => {
                let inner_template_tmp = flush_with_node(
                    graph,
                    tmp,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::InnerTemplate,
                    },
                );

                // this part needs to be fully disjunct from the rest
                // TODO create an add_edge function that enforces that a new node is not needed.
                let mut partial_block_partial_tmp = add_edge_maybe_with_node(
                    graph,
                    vec![(inner_template_tmp, None)],
                    IntermediateAstElement {
                        tag: String::new(),
                        inner: IntermediateAstElementInner::PartialBlockPartial,
                    },
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );
                partial_block_partial_tmp = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    partial_block_partial_tmp,
                    children,
                    parent,
                );
                flush_with_node(
                    graph,
                    partial_block_partial_tmp,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );

                let inner_template_target = *first_nodes
                    .get(&name)
                    .unwrap_or_else(|| panic!("unknown inner template {name}"));

                let inner_template_template_tmp = add_edge_maybe_with_node(
                    graph,
                    vec![(inner_template_tmp, None)],
                    IntermediateAstElement {
                        tag: String::new(),
                        inner: IntermediateAstElementInner::InnerTemplate,
                    },
                    TemplateNode {
                        template_name: name,
                        node_type: NodeType::Other,
                    },
                );

                connect_edges_to_node(graph, inner_template_template_tmp, inner_template_target);

                tmp = vec![(inner_template_tmp, None)];
            }
            Child::PartialBlockPartial => {
                tmp = vec![(
                    flush_with_node(
                        graph,
                        tmp,
                        TemplateNode {
                            template_name: template_name.to_owned(),
                            node_type: NodeType::PartialBlock,
                        },
                    ),
                    None,
                )];
            }
            Child::If(_variable, if_children, else_children) => {
                let if_start = flush_with_node(
                    graph,
                    tmp,
                    TemplateNode {
                        template_name: template_name.to_owned(),
                        node_type: NodeType::Other,
                    },
                );

                let true_tmp = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    vec![(if_start, None)],
                    if_children,
                    parent,
                );

                let mut false_tmp = children_to_ast(
                    first_nodes,
                    template_name,
                    graph,
                    vec![(if_start, None)],
                    else_children,
                    parent,
                );

                tmp = true_tmp;
                tmp.append(&mut false_tmp);
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
    mut tmp: Vec<(NodeIndex, Option<IntermediateAstElement>)>,
    input: Element,
) -> Vec<(NodeIndex, Option<IntermediateAstElement>)> {
    let name = input.name;
    tmp = add_edge_maybe_with_node(
        graph,
        tmp,
        IntermediateAstElement {
            tag: String::new(),
            inner: IntermediateAstElementInner::Text(format!("<{name}")),
        },
        TemplateNode {
            template_name: template_name.to_owned(),
            node_type: NodeType::Other,
        },
    );
    for attribute in input.attributes {
        if let Some(value) = attribute.value {
            tmp = add_edge_maybe_with_node(
                graph,
                tmp,
                IntermediateAstElement {
                    tag: String::new(),
                    inner: IntermediateAstElementInner::Text(format!(r#" {}=""#, attribute.key)),
                },
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
                        tmp = add_edge_maybe_with_node(
                            graph,
                            tmp,
                            IntermediateAstElement {
                                tag: String::new(),
                                inner: IntermediateAstElementInner::Variable {
                                    before: String::new(),
                                    variable_name: next_variable,
                                    escaping_fun,
                                    after: String::new(),
                                },
                            },
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                        );
                    }
                    AttributeValuePart::Literal(string) => {
                        tmp = add_edge_maybe_with_node(
                            graph,
                            tmp,
                            IntermediateAstElement {
                                tag: String::new(),
                                inner: IntermediateAstElementInner::Text(string),
                            },
                            TemplateNode {
                                template_name: template_name.to_owned(),
                                node_type: NodeType::Other,
                            },
                        );
                    }
                }
            }
            tmp = add_edge_maybe_with_node(
                graph,
                tmp,
                IntermediateAstElement {
                    tag: String::new(),
                    inner: IntermediateAstElementInner::Text(r#"""#.to_owned()),
                },
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        } else {
            tmp = add_edge_maybe_with_node(
                graph,
                tmp,
                IntermediateAstElement {
                    tag: String::new(),
                    inner: IntermediateAstElementInner::Text(format!(r#" {}"#, attribute.key)),
                },
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        }
    }
    tmp = add_edge_maybe_with_node(
        graph,
        tmp,
        IntermediateAstElement {
            tag: String::new(),
            inner: IntermediateAstElementInner::Text(">".to_owned()),
        },
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
            tmp = add_edge_maybe_with_node(
                graph,
                tmp,
                IntermediateAstElement {
                    tag: String::new(),
                    inner: IntermediateAstElementInner::Text(format!("</{name}>")),
                },
                TemplateNode {
                    template_name: template_name.to_owned(),
                    node_type: NodeType::Other,
                },
            );
        }
    }
    tmp
}
