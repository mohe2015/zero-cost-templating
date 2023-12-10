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
        let template_struct = node_type_to_create_type_with_span(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            template_codegen.first,
            Span::call_site(),
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
            quote_spanned! {span=> _magic_expression_result.after }
        } else {
            node_type_to_create_type_with_span(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote_spanned! {span=> _magic_expression_result.partial },
                &quote_spanned! {span=> _magic_expression_result.after },
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
            quote_spanned! {span=> _magic_expression_result.after }
        } else {
            node_type_to_create_type_with_span(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote_spanned! {span=> _magic_expression_result.partial },
                &quote_spanned! {span=> _magic_expression_result.after },
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
            // TODO FIXME
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
                Template::<
                    #name,
                    Template::<#partial, (), Template::<#after, (), ()>>,
                    Template::<#after, (), ()>
                >
            }
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            // TODO FIXME
            quote_spanned! {span=>
                Template::<#ident, _, _>
            }
        }
    }
}

fn node_type_to_create_type_with_span(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &TokenStream,
    after: &TokenStream,
) -> TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock { after: inner_after } => {
            let inner_after = format_ident!("{}", inner_after, span = span);
            quote_spanned! {span=>
                // TODO FIXME map_partial and map_after
                #partial.map_inner((), Template { r#type: #inner_after, partial: (), after: #after })
            }
        }
        NodeType::InnerTemplate {
            name,
            partial: inner_partial,
            after: inner_after,
        } => {
            let name = format_ident!("{}", name, span = span);
            let inner_partial = format_ident!("{}", inner_partial, span = span);
            let inner_after = format_ident!("{}", inner_after, span = span);
            quote_spanned! {span=>
                Template::<
                    #name,
                    Template::<#inner_partial, (), Template::<#inner_after, (), ()>>,
                    Template::<#inner_after, (), ()>
                > {
                    r#type: #name,
                    partial: Template::<#inner_partial, (), Template::<#inner_after, (), ()>> {
                        r#type: #inner_partial,
                        partial: (),
                        after: Template::<#inner_after, (), ()> { r#type: #inner_after, partial: (), after: () }
                    },
                    after: Template::<#inner_after, (), ()> { r#type: #inner_after, partial: (), after: () }
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
            // TODO FIXME
            quote_spanned! {span=>
                Template::<#ident, _, _> { r#type: #ident, partial: #partial, after: #after }
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
                pub struct #template_struct;

                impl TemplateTypy for #template_struct {}
            }
        })
}

pub fn calculate_edges(
    template_codegen: &'_ TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    template_codegen.graph.edge_references().map(|edge| {
        edge.weight().variable.as_ref().map_or_else(
            || {
                // no variable
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
                let return_type = if last_node {
                    // TODO FIXME?
                    quote! { After }
                } else {
                    // TODO FIXME extract
                    let span = Span::call_site();
                    match &template_codegen.graph[edge.target()] {
                        NodeType::PartialBlock { after } => {
                            let after = format_ident!("{}", after, span = span);
                            // TODO FIXME REALLY HERE
                            quote_spanned! {span=>
                                Template::<PartialType, (), Template::<#after, (), After>>
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
                                // TODO FIXME
                                Template::<
                                    #name,
                                    Template::<#partial, (), Template::<#after, (), ()>>,
                                    Template::<#after, (), ()>
                                >
                            }
                        }
                        NodeType::Other => {
                            let ident = format_ident!(
                                "{}Template{}",
                                template_codegen.template_name.to_upper_camel_case(),
                                edge.target().index().to_string(),
                                span = span
                            );
                            quote_spanned! {span=>
                                Template::<#ident, Partial, After>
                            }
                        }
                    }
                };
                let impl_func = match &template_codegen.graph[edge.source()] {
                    NodeType::InnerTemplate { .. } | NodeType::PartialBlock { .. } => None,
                    NodeType::Other => Some({
                        let impl_template_name = format_ident!(
                            "{}Template{}",
                            template_codegen.template_name.to_upper_camel_case(),
                            edge.source().index().to_string(),
                        );
                        match &template_codegen.graph[edge.target()] {
                            // TODO FIXME merge this with above
                            NodeType::PartialBlock { .. } => {
                                quote! {
                                    impl<PartialType: TemplateTypy, PartialPartial: Templaty, PartialAfter: Templaty, After: Templaty> Template<#impl_template_name, Template<PartialType, PartialPartial, PartialAfter>, After> {
                                        pub fn #variable_name(self) -> #return_type {
                                            todo!()
                                        }
                                    }
                                }
                            }
                            _ => {
                                quote! {
                                    impl<Partial: Templaty, After: Templaty> Template<#impl_template_name, Partial, After> {
                                        pub fn #variable_name(self) -> #return_type {
                                            todo!()
                                        }
                                    }
                                }
                            }
                        }
                        
                    }),
                };
                let next_template_struct = if last_node {
                    quote! { _magic_expression_result.after }
                } else {
                    node_type_to_create_type_with_span(
                        &template_codegen.template_name,
                        &template_codegen.graph,
                        edge.target(),
                        Span::call_site(),
                        &quote! { $template.partial },
                        &quote! { $template.after },
                    )
                };
                quote! {
                    #[allow(unused)]
                    macro_rules! #variable_name {
                        ($template: expr) => { unreachable!(); #next_template_struct }
                    }

                    #impl_func
                }
            },
            |variable| {
                // with variable
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
                    quote! { _magic_expression_result.after }
                } else {
                    node_type_to_create_type_with_span(
                        &template_codegen.template_name,
                        &template_codegen.graph,
                        edge.target(),
                        Span::call_site(),
                        &quote! { $template.partial },
                        &quote! { $template.after },
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
        let template_struct = node_type_to_create_type_with_span(
            &template_codegen.template_name,
            &template_codegen.graph,
            template_codegen.first,
            Span::call_site(),
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
        pub trait Templaty {}

        pub trait TemplateTypy {}

        #[must_use]
        pub struct Template<Type: TemplateTypy, Partial: Templaty, After: Templaty> {
            r#type: Type,
            partial: Partial,
            after: After,
        }

        impl Templaty for () {}
        impl<Type: TemplateTypy, Partial: Templaty, After: Templaty> Templaty for Template<Type, Partial, After> {}

        impl<Type: TemplateTypy, Partial: Templaty, After: Templaty> Template<Type, Partial, After> {
            pub fn map_inner<NewPartial: Templaty, NewAfter: Templaty>(
                self,
                new_partial: NewPartial,
                new_after: NewAfter,
            ) -> Template<Type, NewPartial, NewAfter> {
                Template {
                    r#type: self.r#type,
                    partial: new_partial,
                    after: new_after,
                }
            }
        }

        #(#code)*
    };
    result
}
