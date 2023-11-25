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

// what about creating a graph with also nodes that just print text and then merge in a postpass?
// I think with branching etc it is pretty hard to merge them to only have nodes with variables

// returns first node, graph, last node
// first node always has no variable
// must return at least one node
#[must_use]
pub fn children_to_ast(
    graph: &mut StableGraph<Option<String>, IntermediateAstElement>,
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
                last = graph.add_node(None);
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
                (last, current) = element_to_ast(graph, last, current, element);
            }
            Child::Each(_identifier, children) => {
                let previous = last;
                last = graph.add_node(None);
                let loop_start = last;
                graph.add_edge(previous, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                (last, current) = children_to_ast(graph, last, current, children, parent);
                graph.add_edge(last, loop_start, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                last = loop_start;
            }
            Child::PartialBlock(name, children) => {
                let previous = last;
                last = graph.add_node(None);
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };

                let before_children = last;
                let children_last;
                (children_last, current) = children_to_ast(graph, last, current, children, parent);
                let inner_template = graph.add_node(Some(format!(
                    "{name}Template<Template{}>",
                    before_children.index(),
                )));
                last = inner_template;

                let previous = last;
                last = graph.add_node(None);
                graph.add_edge(previous, last, current);
                current = IntermediateAstElement {
                    variable: None,
                    escaping_fun: EscapingFunction::NoVariableStart,
                    text: String::new(),
                };
                graph.add_edge(
                    before_children,
                    inner_template,
                    IntermediateAstElement {
                        variable: None,
                        escaping_fun: EscapingFunction::NoVariableStart,
                        text: String::new(),
                    },
                );
            }
            Child::PartialBlockPartial => {
                let previous = last;
                last = graph.add_node(Some("Generic".to_owned()));
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
    graph: &mut StableGraph<Option<String>, IntermediateAstElement>,
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
                        last = graph.add_node(None);
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
    (last, current) = children_to_ast(graph, last, current, input.children, &name);
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
