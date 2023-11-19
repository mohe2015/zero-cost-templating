use core::fmt::{Display, Write};

use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use proc_macro2::TokenTree;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{visit_mut, Expr, Macro, Stmt, Token};

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

pub struct InnerMacroReplace {
    pub graph: StableGraph<(), IntermediateAstElement>,
    pub first: NodeIndex,
    pub last: NodeIndex,
}

impl InnerMacroReplace {
    #[expect(clippy::too_many_lines, reason = "tmp")]
    fn magic(&self, input: &Macro, semicolon: Option<Token![;]>) -> Option<syn::Expr> {
        let ident = input.path.require_ident().unwrap();
        let template = input.tokens.clone();
        let mut template = template.into_iter();
        let first_parameter = template.take_while_ref(
            |elem| !matches!(elem, TokenTree::Punct(punct) if punct.as_char() == ','),
        );
        let first_parameter = first_parameter.collect::<proc_macro2::TokenStream>();
        let comma = template.next();
        let span = input.span();
        comma.map_or_else(
            || {
                let first_index = self.first.index();
                let initial_ident = format_ident!("initial{}", first_index);
                if &initial_ident == ident {
                    if !first_parameter.is_empty() {
                        // fall back to compiler macro error
                        return None;
                    }
                    let template_struct = format_ident!("Template{}", self.first.index());
                    return Some(Expr::Verbatim(quote_spanned! {span=>
                        {
                            #template_struct
                        } #semicolon
                    }));
                }

                let edge = self.graph.edge_references().find(|edge| {
                    let expected_ident = format_ident!("{}{}", "template", edge.id().index());
                    ident == &expected_ident
                });
                edge.and_then(|edge| {
                    if first_parameter.is_empty() {
                        // fall back to compiler macro error
                        return None;
                    }

                    let text = &edge.weight().text;
                    let template_struct = format_ident!(
                        "Template{}",
                        edge.source().index(),
                        span = input.path.span()
                    ); // good span for mismatched type error
                    let next_template_struct = if edge.target() == self.last {
                        quote_spanned! {span=>
                            ()
                        }
                    } else {
                        let ident = format_ident!("Template{}", edge.target().index(), span = span);
                        quote! {
                            #ident
                        }
                    };

                    let tmp = quote! {
                        let magic_expression_result: #template_struct = #first_parameter;
                    };
                    Some(Expr::Verbatim(quote! {
                        {
                            {
                                #tmp
                                drop(magic_expression_result);
                            }
                            yield Cow::from(#text);
                            #next_template_struct
                        } #semicolon
                    }))
                })
            },
            |_comma| {
                let edge = self.graph.edge_references().find(|edge| {
                    edge.weight().variable.as_ref().map_or(false, |variable| {
                        let expected_ident = format_ident!("{}{}", variable, edge.id().index());
                        ident == &expected_ident
                    })
                });
                edge.and_then(|edge| {
                    let second_parameter = template.collect::<proc_macro2::TokenStream>();
                    if first_parameter.is_empty() || second_parameter.is_empty() {
                        // fall back to compiler macro error
                        return None;
                    }

                    let text = &edge.weight().text;
                    let _second_parameter_span = second_parameter.span();

                    let template_struct = format_ident!(
                        "Template{}",
                        edge.source().index(),
                        span = input.path.span()
                    ); // good span for mismatched type error
                    let next_template_struct = if edge.target() == self.last {
                        quote_spanned! {span=>
                            ()
                        }
                    } else {
                        let ident = format_ident!("Template{}", edge.target().index(), span = span);
                        quote! {
                            #ident
                        }
                    };

                    let tmp = quote! {
                        let magic_expression_result: #template_struct = #first_parameter;
                    };
                    let escaped_value = match edge.weight().escaping_fun {
                        EscapingFunction::NoVariableStart => quote! {
                            unreachable();
                        },
                        EscapingFunction::HtmlAttribute | EscapingFunction::HtmlElementInner => {
                            quote! {
                                yield zero_cost_templating::encode_safe(#second_parameter);
                            }
                        }
                    };
                    Some(Expr::Verbatim(quote! {
                        {
                            {
                                #tmp
                                drop(magic_expression_result);
                            }
                            #escaped_value
                            yield Cow::from(#text);
                            #next_template_struct
                        } #semicolon
                    }))
                })
            },
        )
    }
}

impl VisitMut for InnerMacroReplace {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        if let Expr::Macro(expr_macro) = node {
            if let Some(result) = self.magic(&expr_macro.mac, None) {
                *node = result;
            }
        } else {
            visit_mut::visit_expr_mut(self, node);
        }
    }

    fn visit_stmt_mut(&mut self, node: &mut syn::Stmt) {
        if let Stmt::Macro(stmt_macro) = node {
            if let Some(result) = self.magic(&stmt_macro.mac, stmt_macro.semi_token) {
                *node = Stmt::Expr(result, None);
            }
        } else {
            visit_mut::visit_stmt_mut(self, node);
        }
    }
}

#[must_use]
pub fn codegen(
    graph: &StableGraph<(), IntermediateAstElement>,
    first: NodeIndex,
    last: NodeIndex,
) -> proc_macro2::TokenStream {
    let instructions = graph.node_references().map(|(node_index, _node)| {
        let template_struct = format_ident!("Template{}", node_index.index());

        quote! {
            #[must_use]
            pub struct #template_struct;
        }
    });
    let edges = graph.edge_references().map(|edge| {
        edge.weight().variable.as_ref().map_or_else(
            || {
                let variable_name = format_ident!("{}{}", "template", edge.id().index());
                let next_template_struct = if edge.target() == last {
                    quote! {
                        ()
                    }
                } else {
                    let ident = format_ident!("Template{}", edge.target().index());
                    quote! {
                        #ident
                    }
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr) => { unreachable!(); #next_template_struct }
                    }
                }
            },
            |variable| {
                let variable_name = format_ident!("{}{}", variable, edge.id().index());
                let next_template_struct = if edge.target() == last {
                    quote! {
                        ()
                    }
                } else {
                    let ident = format_ident!("Template{}", edge.target().index());
                    quote! {
                        #ident
                    }
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr, $value: expr) => { unreachable!(); #next_template_struct }
                    }
                }
            },
        )
    });
    let first_index = first.index();
    let ident = format_ident!("initial{}", first_index);
    let template_struct = format_ident!("Template{}", first_index);
    let other = quote! {
        #[allow(unused)]
        macro_rules! #ident {
            () => { unreachable!(); #template_struct }
        }
    };

    let result = quote! {
        #(#instructions)*

        #(#edges)*

        #other
    };
    result
}
