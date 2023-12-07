use std::path::PathBuf;

use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{visit_mut, Expr, Macro, Stmt, Token};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType};

pub struct InnerMacroReplace(pub Vec<TemplateCodegen>);

fn handle_macro_call_zero_or_one_parameter(
    input: &Macro,
    ident: &Ident,
    span: Span,
    first_parameter: &TokenStream,
    semicolon: Option<Token![;]>,
    template_codegen: &TemplateCodegen,
) -> Option<Expr> {
    let first_index = template_codegen.first.index();
    let initial_ident = format_ident!(
        "{}_initial{}",
        template_codegen.template_name,
        first_index,
        span = span
    );
    if &initial_ident == ident {
        if !first_parameter.is_empty() {
            // one parameter
            // fall back to compiler macro error
            return None;
        }
        let template_struct = node_type_to_create_type(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            template_codegen.first,
            &quote_spanned! {span=> () },
            &quote_spanned! {span=> () },
        );
        return Some(Expr::Verbatim(quote_spanned! {span=>
            {
                #template_struct
            } #semicolon
        }));
    }

    let edge = template_codegen.graph.edge_references().find(|edge| {
        let expected_ident = format_ident!(
            "{}_template{}",
            template_codegen.template_name,
            edge.id().index(),
            span = span
        );
        ident == &expected_ident
    });
    edge.and_then(|edge| {
        if first_parameter.is_empty() {
            // no parameters
            // fall back to compiler macro error
            return None;
        }

        let text = &edge.weight().text;
        let template_struct = node_type_to_type_with_span(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            edge.source(),
            input.path.span(),
        ); // good span for mismatched type error
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote_spanned! {span=> _magic_expression_result.end_type }
        } else {
            node_type_to_create_type_with_span(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote_spanned! {span=> _magic_expression_result.partial_type },
                &quote_spanned! {span=> _magic_expression_result.end_type },
            )
        };

        Some(Expr::Verbatim(quote_spanned! {span=>
            {
                let _magic_expression_result: #template_struct = #first_parameter;
                yield ::alloc::borrow::Cow::from(#text);
                #next_template_struct
            } #semicolon
        }))
    })
}

fn handle_macro_call_two_parameters(
    input: &Macro,
    ident: &Ident,
    span: Span,
    first_parameter: &TokenStream,
    second_parameter: &TokenStream,
    semicolon: Option<Token![;]>,
    template_codegen: &TemplateCodegen,
) -> Option<Expr> {
    // macro call with two parameters
    let edge = template_codegen.graph.edge_references().find(|edge| {
        edge.weight().variable.as_ref().map_or(false, |variable| {
            let expected_ident = format_ident!(
                "{}_{}{}",
                template_codegen.template_name,
                variable,
                edge.id().index(),
                span = span
            );
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

        let template_struct = node_type_to_type_with_span(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            edge.source(),
            input.path.span(),
        ); // good span for mismatched type error
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote_spanned! {span=> _magic_expression_result.end_type }
        } else {
            node_type_to_create_type_with_span(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote_spanned! {span=> _magic_expression_result.partial_type },
                &quote_spanned! {span=> _magic_expression_result.end_type },
            )
        };

        let escaped_value = match edge.weight().escaping_fun {
            EscapingFunction::NoVariableStart => quote_spanned! {span=>
                unreachable();
            },
            EscapingFunction::HtmlAttribute => {
                quote_spanned! {span=>
                    yield zero_cost_templating::encode_double_quoted_attribute(#second_parameter);
                }
            }
            EscapingFunction::HtmlElementInner => {
                quote_spanned! {span=>
                    yield zero_cost_templating::encode_element_text(#second_parameter);
                }
            }
        };
        Some(Expr::Verbatim(quote_spanned! {span=>
            {
                let _magic_expression_result: #template_struct = #first_parameter;
                #escaped_value
                yield ::alloc::borrow::Cow::from(#text);
                #next_template_struct
            } #semicolon
        }))
    })
}

impl InnerMacroReplace {
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
                // macro call without zero or one parameters
                self.0.iter().find_map(|template_codegen| {
                    handle_macro_call_zero_or_one_parameter(
                        input,
                        ident,
                        span,
                        &first_parameter,
                        semicolon,
                        template_codegen,
                    )
                })
            },
            |_comma| {
                let second_parameter = template.collect::<proc_macro2::TokenStream>();

                self.0.iter().find_map(|template_codegen| {
                    handle_macro_call_two_parameters(
                        input,
                        ident,
                        span,
                        &first_parameter,
                        &second_parameter,
                        semicolon,
                        template_codegen,
                    )
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

fn node_type_to_type_with_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
) -> proc_macro2::TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock { .. } => {
            quote_spanned! {span=>
                _
            }
        }
        NodeType::InnerTemplate {
            name,
            partial,
            after,
        } => {
            let name = format_ident!("{}", name, span = span);
            let partial = format_ident!("{}", partial, span = span);
            let after = format_ident!("{}", after, span = span);
            quote_spanned! {span=>
                #name<#partial<(), #after<(), ()>>, #after<(), ()>>
            }
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            quote_spanned! {span=>
                #ident<_, _>
            }
        }
    }
}

fn node_type_to_create_type(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    partial_type: &TokenStream,
    end_type: &TokenStream,
) -> TokenStream {
    node_type_to_create_type_with_span(
        template_name,
        graph,
        node_index,
        Span::call_site(),
        partial_type,
        end_type,
    )
}

fn node_type_to_create_type_with_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial_type: &TokenStream,
    end_type: &TokenStream,
) -> TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock { after } => {
            let after = format_ident!("{}", after, span = span);
            quote_spanned! {span=>
                // TODO FIXME map_partial_type and map_end_type
                #partial_type.map_inner((), #after { partial_type: (), end_type: #end_type })
            }
        }
        NodeType::InnerTemplate {
            name,
            partial,
            after,
        } => {
            let name = format_ident!("{}", name, span = span);
            let partial = format_ident!("{}", partial, span = span);
            let after = format_ident!("{}", after, span = span);
            quote_spanned! {span=>
                #name::<#partial::<(), #after<(), ()>>, #after::<(), ()>> {
                    partial_type: #partial::<(), #after<(), ()>> {
                        partial_type: (),
                        end_type: #after::<(), ()> { partial_type: (), end_type: () }
                    },
                    end_type: #after::<(), ()> { partial_type: (), end_type: () }
                }
            }
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            quote_spanned! {span=>
                #ident::<_, _> { partial_type: #partial_type, end_type: #end_type }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateCodegen {
    pub path: PathBuf,
    pub template_name: String,
    pub graph: StableGraph<NodeType, IntermediateAstElement>,
    pub first: NodeIndex,
    pub last: NodeIndex,
}

pub fn calculate_nodes(
    template_codegen: &'_ TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    template_codegen
        .graph
        .node_references()
        .filter_map(|(node_index, node)| match node {
            NodeType::InnerTemplate { .. } | NodeType::PartialBlock { .. } => None,
            NodeType::Other => Some(format_ident!(
                "{}Template{}",
                template_codegen.template_name.to_upper_camel_case(),
                node_index.index().to_string(),
            )),
        })
        .map(|template_struct| {
            quote! {
                #[must_use]
                pub struct #template_struct<PartialType, EndType> {
                    partial_type: PartialType,
                    end_type: EndType,
                }

                impl<PartialType, EndType> #template_struct<PartialType, EndType> {
                    pub fn map_inner<NewPartialType, NewEndType>(
                                self,
                                new_partial_type: NewPartialType,
                                new_end_type: NewEndType)
                            -> #template_struct<NewPartialType, NewEndType> {
                        #template_struct {
                            partial_type: new_partial_type,
                            end_type: new_end_type,
                        }
                    }
                }
            }
        })
}

pub fn calculate_edges(
    template_codegen: &'_ TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    template_codegen.graph.edge_references().map(|edge| {
        edge.weight().variable.as_ref().map_or_else(
            || {
                let variable_name = format_ident!(
                    "{}_template{}",
                    template_codegen.template_name,
                    edge.id().index()
                );
                let last_node = template_codegen
                    .graph
                    .edges_directed(edge.target(), Direction::Outgoing)
                    .next()
                    .is_none();
                let next_template_struct = if last_node {
                    quote! { _magic_expression_result.end_type }
                } else {
                    node_type_to_create_type(
                        &template_codegen.template_name,
                        &template_codegen.graph,
                        edge.target(),
                        &quote! { $template.partial_type },
                        &quote! { $template.end_type },
                    )
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr) => { unreachable!(); #next_template_struct }
                    }
                }
            },
            |variable| {
                let variable_name = format_ident!(
                    "{}_{}{}",
                    template_codegen.template_name,
                    variable,
                    edge.id().index()
                );
                let last_node = template_codegen
                    .graph
                    .edges_directed(edge.target(), Direction::Outgoing)
                    .next()
                    .is_none();
                let next_template_struct = if last_node {
                    quote! { _magic_expression_result.end_type }
                } else {
                    node_type_to_create_type(
                        &template_codegen.template_name,
                        &template_codegen.graph,
                        edge.target(),
                        &quote! { $template.partial_type },
                        &quote! { $template.end_type },
                    )
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr, $value: expr) => { unreachable!(); #next_template_struct }
                    }
                }
            },
        )
    })
}

pub fn codegen(templates: &[TemplateCodegen]) -> proc_macro2::TokenStream {
    // TODO FIXME spans
    let code = templates.iter().map(|template_codegen| {
        let instructions = calculate_nodes(template_codegen);
        let edges = calculate_edges(template_codegen);
        let ident = format_ident!(
            "{}_initial{}",
            template_codegen.template_name,
            template_codegen.first.index()
        );
        let template_struct = node_type_to_create_type(
            &template_codegen.template_name,
            &template_codegen.graph,
            template_codegen.first,
            &quote! { () },
            &quote! { () },
        );
        let other = quote! {
            #[allow(unused)]
            macro_rules! #ident {
                () => { unreachable!(); #template_struct }
            }
        };
        let recompile_ident = format_ident!("_{}_FORCE_RECOMPILE", template_codegen.template_name);
        let path = template_codegen.path.to_string_lossy();
        quote! {
            #(#instructions)*

            #(#edges)*

            #other

            const #recompile_ident: &'static str = include_str!(#path);
        }
    });

    let result = quote! {
        #(#code)*
    };
    result
}
