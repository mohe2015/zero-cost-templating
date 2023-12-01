use core::fmt::{Display, Write};

use petgraph::stable_graph::{NodeIndex, StableGraph};

use crate::html_recursive_descent::{AttributeValuePart, Child, Element};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum NodeType {
    PartialBlock,
    InnerTemplate { name: String, partial: String },
    Other,
}

// what about creating a graph with also nodes that just print text and then merge in a postpass?
// I think with branching etc it is pretty hard to merge them to only have nodes with variables

// returns first node, graph, last node
// first node always has no variable
// must return at least one node
#[must_use]
pub fn children_to_ast(
    template_name: &str,
    graph: &mut StableGraph<NodeType, IntermediateAstElement>,
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
                let previous = last;
                last = graph.add_node(NodeType::Other);
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: Some(next_variable),
                    escaping_fun,
                    text: String::new(),
                };
            }
            Child::Literal(string) => {
                write!(&mut current.text, "{string}").unwrap();
            }
            Child::Element(element) => {
                assert!(
                    !(parent == "script" || parent == "style"),
                    "children are unsafe in <script> and <style>"
                );
                (last, current) = element_to_ast(template_name, graph, last, current, element);
            }
            Child::Each(_identifier, children) => {
                let previous = last;
                last = graph.add_node(NodeType::Other);
                let loop_start = last;
                graph.add_edge(previous, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                (last, current) =
                    children_to_ast(template_name, graph, last, current, children, parent);
                graph.add_edge(last, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                last = loop_start;
            }
            Child::PartialBlock(name, children) => {
                let first = last;
                let inner_template = graph.add_node(NodeType::Other);
                last = inner_template;
                graph.add_edge(first, last, current);

                let inner_template_start = graph.add_node(NodeType::Other);
                last = inner_template_start;
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };

                (last, current) =
                    children_to_ast(template_name, graph, last, current, children, parent);

                let previous = last;
                let after_all = graph.add_node(NodeType::Other);
                last = after_all;
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };

                last = inner_template;

                graph[inner_template] = NodeType::InnerTemplate {
                    name: format!("{name}Template0"), // Start
                    partial: format!("{name}Template{}", inner_template_start.index()),
                };
            }
            Child::PartialBlockPartial => {
                let previous = last;
                last = graph.add_node(NodeType::PartialBlock);
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
            }
        }
    }
    (last, current)
}

#[must_use]
pub fn element_to_ast(
    template_name: &str,
    graph: &mut StableGraph<NodeType, IntermediateAstElement>,
    mut last: NodeIndex,
    mut current: IntermediateAstElement,
    input: Element,
) -> (NodeIndex, IntermediateAstElement) {
    let name = input.name;
    write!(&mut current.text, "<{name}").unwrap();
    for attribute in input.attributes {
        write!(&mut current.text, r#" {}"#, attribute.key).unwrap();
        if let Some(value) = attribute.value {
            write!(&mut current.text, r#"=""#).unwrap();
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
                        let previous = last;
                        last = graph.add_node(NodeType::Other);
                        graph.add_edge(previous, last, current);
                        current = IntermediateAstElement {
                            variable: Some(next_variable),
                            escaping_fun,
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
    }
    write!(&mut current.text, ">").unwrap();
    (last, current) = children_to_ast(template_name, graph, last, current, input.children, &name);
    // https://html.spec.whatwg.org/dev/syntax.html#void-elements
    match name.as_str() {
        "!doctype" | "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
        | "meta" | "source" | "track" | "wbr" => {}
        _ => {
            write!(&mut current.text, "</{name}>").unwrap();
        }
    }
    (last, current)
}
