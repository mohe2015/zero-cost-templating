use core::fmt::{Display, Write};

use petgraph::stable_graph::{NodeIndex, StableGraph};

use crate::html_recursive_descent::{AttributeValuePart, Child, Element};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum EscapingFunction {
    NoVariableStart,
    HtmlAttribute,
    HtmlElementInner,
}

impl Display for EscapingFunction {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoVariableStart => write!(formatter, "plain"),
            Self::HtmlAttribute => write!(formatter, "attr"),
            Self::HtmlElementInner => write!(formatter, "element"),
        }
    }
}

// first variable then text, so we can print as much as possible
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct IntermediateAstElement {
    pub variable: Option<String>, // TODO FIXME add escaping function in this optional
    pub escaping_fun: EscapingFunction,
    pub text: String,
}

impl Display for IntermediateAstElement {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(variable) = &self.variable {
            write!(
                formatter,
                "{{{{{variable}:{}}}}}{}",
                self.escaping_fun, self.text
            )?;
        } else {
            write!(formatter, "{}", self.text)?;
        }
        Ok(())
    }
}

// what about creating a graph with also nodes that just print text and then merge in a postpass?
// I think with branching etc it is pretty hard to merge them to only have nodes with variables

// returns first node, graph, last node
// first node always has no variable
// must return at least one node
#[must_use]
pub fn children_to_ast(
    graph: &mut StableGraph<(), IntermediateAstElement>,
    mut last: NodeIndex,
    mut current: IntermediateAstElement,
    input: Vec<Child>,
) -> (NodeIndex, IntermediateAstElement) {
    for child in input {
        match child {
            Child::Variable(next_variable) => {
                let previous = last;
                last = graph.add_node(());
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: Some(next_variable),
                    escaping_fun: EscapingFunction::HtmlElementInner,
                    text: String::new(),
                };
            }
            Child::Literal(string) => {
                write!(&mut current.text, "{string}").unwrap();
            }
            Child::Element(element) => {
                (last, current) = element_to_ast(graph, last, current, element);
            }
            Child::Each(_identifier, children) => {
                let previous = last;
                last = graph.add_node(());
                let loop_start = last;
                graph.add_edge(previous, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                (last, current) = children_to_ast(graph, last, current, children);
                graph.add_edge(last, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                last = loop_start;
            }
        }
    }
    (last, current)
}

#[must_use]
pub fn element_to_ast(
    graph: &mut StableGraph<(), IntermediateAstElement>,
    mut last: NodeIndex,
    mut current: IntermediateAstElement,
    input: Element,
) -> (NodeIndex, IntermediateAstElement) {
    let name = input.name;
    write!(&mut current.text, "<{name}").unwrap();
    for attribute in input.attributes {
        write!(&mut current.text, r#" {}=""#, attribute.key).unwrap();
        for value_part in attribute.value {
            match value_part {
                AttributeValuePart::Variable(next_variable) => {
                    let previous = last;
                    last = graph.add_node(());
                    graph.add_edge(previous, last, current);
                    current = IntermediateAstElement {
                        variable: Some(next_variable),
                        escaping_fun: EscapingFunction::HtmlElementInner,
                        text: String::new(),
                    };
                }
                AttributeValuePart::Literal(string) => {
                    write!(&mut current.text, "{string}").unwrap();
                }
            }
        }
        write!(&mut current.text, r#"""#).unwrap();
    }
    write!(&mut current.text, ">").unwrap();
    (last, current) = children_to_ast(graph, last, current, input.children);
    write!(&mut current.text, "</{name}>").unwrap();
    (last, current)
}
