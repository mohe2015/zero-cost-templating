use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use proc_macro2::{Span, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{visit_mut, Expr, Macro, Stmt, Token};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType};

pub struct InnerMacroReplace(pub Vec<TemplateCodegen>);

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
              self.0.iter().find_map(|template_codegen| {
                // macro call without zero or one parameters
                let first_index = template_codegen.first.index();
                let initial_ident = format_ident!("{}_initial{}", template_codegen.template_name, first_index);
                if &initial_ident == ident {
                    if !first_parameter.is_empty() {
                        // one parameter
                        // fall back to compiler macro error
                        return None;
                    }
                    let template_struct = node_type_to_create_type(template_codegen.template_name.as_str(), &template_codegen.graph, template_codegen.first);
                    return Some(Expr::Verbatim(quote_spanned! {span=>
                        {
                            #template_struct
                        } #semicolon
                    }));
                }

                let edge = template_codegen.graph.edge_references().find(|edge| {
                    let expected_ident = format_ident!("{}_template{}", template_codegen.template_name, edge.id().index());
                    ident == &expected_ident
                });
                edge.and_then(|edge| {
                    if first_parameter.is_empty() {
                        // no parameters
                        // fall back to compiler macro error
                        return None;
                    }

                    let text = &edge.weight().text;
                    let template_struct = node_type_to_type_with_span(template_codegen.template_name.as_str(), &template_codegen.graph, edge.source(), input.path.span()); // good span for mismatched type error
                    let next_template_struct = if edge.target() == template_codegen.last {
                        quote_spanned! {span=>
                            ()
                        }
                    } else {
                        // here?
                        
                        node_type_to_create_type_with_span(template_codegen.template_name.as_str(), &template_codegen.graph, edge.target(), span)
                    };

                    Some(Expr::Verbatim(quote! {
                        {
                            {
                                let magic_expression_result: #template_struct = #first_parameter;
                                drop(magic_expression_result);
                            }
                            yield ::alloc::borrow::Cow::from(#text);
                            #next_template_struct
                        } #semicolon
                    }))
                })
            })
            },
            |_comma| {
                let second_parameter = template.collect::<proc_macro2::TokenStream>();

                self.0.iter().find_map(|template_codegen| {

                // macro call with two parameters
                let edge = template_codegen.graph.edge_references().find(|edge| {
                    edge.weight().variable.as_ref().map_or(false, |variable| {
                        let expected_ident = format_ident!("{}_{}{}", template_codegen.template_name, variable, edge.id().index());
                        ident == &expected_ident
                    })
                });
                edge.and_then(|edge| {
                    if first_parameter.is_empty() || second_parameter.is_empty() {
                        // one of the parameters is empty
                        // fall back to compiler macro error
                        return None;
                    }

                    let text = &edge.weight().text;
                    let _second_parameter_span = second_parameter.span();

                    let template_struct = node_type_to_type_with_span(template_codegen.template_name.as_str(), &template_codegen.graph, edge.source(), input.path.span()); // good span for mismatched type error
                    let next_template_struct = if edge.target() == template_codegen.last {
                        quote_spanned! {span=>
                            ()
                        }
                    } else {
                        node_type_to_create_type_with_span(template_codegen.template_name.as_str(), &template_codegen.graph, edge.target(), span)
                    };

                    let tmp = quote! {
                        let magic_expression_result: #template_struct = #first_parameter;
                    };
                    let escaped_value = match edge.weight().escaping_fun {
                        EscapingFunction::NoVariableStart => quote! {
                            unreachable();
                        },
                        EscapingFunction::HtmlAttribute => {
                            quote! {
                                yield zero_cost_templating::encode_double_quoted_attribute(#second_parameter);
                            }
                        }
                        EscapingFunction::HtmlElementInner => {
                            quote! {
                                yield zero_cost_templating::encode_element_text(#second_parameter);
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
                            yield ::alloc::borrow::Cow::from(#text);
                            #next_template_struct
                        } #semicolon
                    }))
                })
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

fn node_type_to_type(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
) -> proc_macro2::TokenStream {
    node_type_to_type_with_span(template_name, graph, node_index, Span::call_site())
}

fn node_type_to_type_with_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
) -> proc_macro2::TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock => {
            let ident = format_ident!("PartialType");
            quote! {
                #ident
            }
        }
        NodeType::InnerTemplate { name, partial } => {
            todo!()
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            quote! {
                #ident
            }
        }
    }
}

fn node_type_to_create_type(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
) -> proc_macro2::TokenStream {
    node_type_to_create_type_with_span(template_name, graph, node_index, Span::call_site())
}

fn node_type_to_create_type_with_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
) -> proc_macro2::TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock => {
            let ident = format_ident!("PartialType");
            quote! {
                #ident
            }
        }
        NodeType::InnerTemplate { name, partial } => {
            todo!()
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            quote! {
                #ident::<(), ()> { partial_type: ::core::marker::PhantomData, end_type: ::core::marker::PhantomData }
            }
        }
    }
}


fn node_type_to_create_type_with_target_and_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    target: NodeIndex,
    span: Span,
) -> proc_macro2::TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock => {
            let ident = format_ident!("PartialType");
            quote! {
                #ident
            }
        }
        NodeType::InnerTemplate { name, partial } => {
            let name = format_ident!("{}", name);
            let partial = format_ident!("{}", partial);
            let end = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                target.index().to_string(),
                span = span
            );
            quote! {
                #name::<#partial, #end<(), ()>> { partial_type: ::core::marker::PhantomData, end_type: ::core::marker::PhantomData }
            }
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            quote! {
                #ident::<(), ()> { partial_type: ::core::marker::PhantomData, end_type: ::core::marker::PhantomData }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateCodegen {
    pub template_name: String,
    pub graph: StableGraph<NodeType, IntermediateAstElement>,
    pub first: NodeIndex,
    pub last: NodeIndex,
}

#[must_use]
pub fn codegen(templates: &[TemplateCodegen]) -> proc_macro2::TokenStream {
    let code = templates.iter().map(|template| {
        let instructions = template
            .graph
            .node_references()
            .filter_map(|(node_index, node)| match node {
                NodeType::InnerTemplate { .. } | NodeType::PartialBlock => None,
                NodeType::Other => Some(node_index),
            })
            .map(|node_index| {
                let template_struct =
                    node_type_to_type(&template.template_name, &template.graph, node_index);

                quote! {
                    #[must_use]
                    pub struct #template_struct<PartialType, EndType> {
                        partial_type: ::core::marker::PhantomData<PartialType>,
                        end_type: ::core::marker::PhantomData<EndType>,
                    }
                }
            });
        let edges = template.graph.edge_references().map(|edge| {
            edge.weight().variable.as_ref().map_or_else(
            || {
                let variable_name = format_ident!("{}_template{}", template.template_name, edge.id().index());
                let next_template_struct = if edge.target() == template.last {
                    quote! {
                        ()
                    }
                } else {
                    // TODO FIXME
                    node_type_to_create_type(&template.template_name, &template.graph, edge.target())
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr) => { unreachable!(); #next_template_struct }
                    }
                }
            },
            |variable| {
                let variable_name = format_ident!("{}_{}{}", template.template_name, variable, edge.id().index());
                let next_template_struct = if edge.target() == template.last {
                    quote! {
                        ()
                    }
                } else {
                    node_type_to_create_type(&template.template_name, &template.graph, edge.target())
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
        let ident = format_ident!("{}_initial{}", template.template_name, template.first.index());
        let template_struct =
            node_type_to_create_type(&template.template_name, &template.graph, template.first);
        let other = quote! {
            #[allow(unused)]
            macro_rules! #ident {
                () => { unreachable!(); #template_struct }
            }
        };
        let recompile_ident = format_ident!("_{}_FORCE_RECOMPILE", template.template_name);
        let input = format!("{}.html.hbs", template.template_name);
        quote! {
            #(#instructions)*

            #(#edges)*

            #other

            const #recompile_ident: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #input));
        }
    });

    let result = quote! {
        #(#code)*
    };
    result
}
