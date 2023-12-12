use std::path::PathBuf;

use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{visit_mut, Expr, ExprMethodCall, Macro, Stmt, Token};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType};

pub struct InnerMacroReplace(pub Vec<TemplateCodegen>);

fn handle_macro_call_zero_or_one_parameter(
    path_span: Span,
    ident: &Ident,
    span: Span,
    first_parameter: &TokenStream,
    semicolon: Option<Token![;]>,
    template_codegen: &TemplateCodegen,
) -> Option<Expr> {
    let first_index = template_codegen.first.index();
    let initial_ident = format_ident!("{}_initial{}", template_codegen.template_name, first_index,);
    if &initial_ident == ident {
        if !first_parameter.is_empty() {
            // one parameter
            // fall back to compiler macro error
            return None;
        }
        let template_struct = node_type(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            template_codegen.first,
            Span::call_site(),
            &quote! { () },
            &quote! { () },
            &quote! { _ },
            &quote! { _ },
            true,
        );
        return Some(Expr::Verbatim(quote! {
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
        let template_struct = node_type(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            edge.source(),
            path_span,
            &quote! { () },
            &quote! { () },
            &quote! { _ },
            &quote! { _ },
            false,
        ); // good span for mismatched type error
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote! { _magic_expression_result.after }
        } else {
            node_type(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote! { _magic_expression_result.partial },
                &quote! { _magic_expression_result.after },
                &quote! { _ },
                &quote! { _ },
                true,
            )
        };

        Some(Expr::Verbatim(quote! {
            {
                let _magic_expression_result: #template_struct = #first_parameter;
                yield ::alloc::borrow::Cow::from(#text);
                #next_template_struct
            } #semicolon
        }))
    })
}

fn handle_macro_call_two_parameters(
    path_span: Span,
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

        let template_struct = node_type(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            edge.source(),
            path_span,
            &quote! { () },
            &quote! { () },
            &quote! { _ },
            &quote! { _ },
            false,
        ); // good span for mismatched type error
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote! { _magic_expression_result.after }
        } else {
            node_type(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                span,
                &quote! { _magic_expression_result.partial },
                &quote! { _magic_expression_result.after },
                &quote! { _ },
                &quote! { _ },
                true,
            )
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
                let _magic_expression_result: #template_struct = #first_parameter;
                #escaped_value
                yield ::alloc::borrow::Cow::from(#text);
                #next_template_struct
            } #semicolon
        }))
    })
}

impl InnerMacroReplace {
    fn magic_macro(&self, input: &Macro, semicolon: Option<Token![;]>) -> Option<syn::Expr> {
        let ident = input.path.require_ident().unwrap();
        let template = input.tokens.clone();
        let mut template = template.into_iter();
        let first_parameter = template.take_while_ref(
            |elem| !matches!(elem, TokenTree::Punct(punct) if punct.as_char() == ','),
        );
        let first_parameter = first_parameter.collect::<proc_macro2::TokenStream>();
        let comma = template.next();
        comma.map_or_else(
            || {
                // macro call without zero or one parameters
                self.0.iter().find_map(|template_codegen| {
                    handle_macro_call_zero_or_one_parameter(
                        input.path.span(),
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
                        input.path.span(),
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

    fn magic_method_call(
        &self,
        input: &ExprMethodCall,
        semicolon: Option<Token![;]>,
    ) -> Option<syn::Expr> {
        let ident = &input.method;
        match input.args.len() {
            0 => {
                // macro call without zero or one parameters
                self.0.iter().find_map(|template_codegen| {
                    handle_macro_call_zero_or_one_parameter(
                        input.method.span(),
                        ident,
                        span,
                        &input.receiver.to_token_stream(),
                        semicolon,
                        template_codegen,
                    )
                })
            }
            1 => self.0.iter().find_map(|template_codegen| {
                handle_macro_call_two_parameters(
                    input.method.span(),
                    ident,
                    span,
                    &input.receiver.to_token_stream(),
                    &input.args.first().unwrap().into_token_stream(),
                    semicolon,
                    template_codegen,
                )
            }),
            _ => panic!(),
        }
    }
}

impl VisitMut for InnerMacroReplace {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        match node {
            Expr::Macro(expr_macro) => {
                if let Some(result) = self.magic_macro(&expr_macro.mac, None) {
                    *node = result;
                }
            }
            Expr::MethodCall(expr_method_call) => {
                if let Some(result) = self.magic_method_call(&expr_method_call, None) {
                    *node = result;
                }
            }
            _ => {
                visit_mut::visit_expr_mut(self, node);
            }
        }
    }

    fn visit_stmt_mut(&mut self, node: &mut syn::Stmt) {
        match node {
            Stmt::Macro(stmt_macro) => {
                if let Some(result) = self.magic_macro(&stmt_macro.mac, stmt_macro.semi_token) {
                    *node = Stmt::Expr(result, None);
                }
            }
            _ => {
                visit_mut::visit_stmt_mut(self, node);
            }
        }
    }
}

fn node_type(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &TokenStream,
    after: &TokenStream,
    partial_type: &TokenStream,
    after_type: &TokenStream,
    create: bool,
) -> TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock { after: inner_after } => {
            let inner_after = format_ident!("{}", inner_after);
            let create = create.then(|| {
                Some(quote! {
                    {
                        r#type: #partial.r#type,
                        partial: (),
                        after: Template { r#type: #inner_after, partial: (), after: #after }
                    }
                })
            });
            quote! {
                Template::<#partial_type, (), Template::<#inner_after, (), #after_type>> #create
            }
        }
        NodeType::InnerTemplate {
            name,
            partial: inner_partial,
            after: inner_after,
        } => {
            let name = format_ident!("{}", name);
            let inner_partial = format_ident!("{}", inner_partial);
            let inner_after = format_ident!("{}", inner_after);
            let create = create.then(|| {
                Some(quote! {
                    {
                        r#type: #name,
                        partial: Template::<#inner_partial, (), Template::<#inner_after, (), ()>> {
                            r#type: #inner_partial,
                            partial: (),
                            after: Template::<#inner_after, (), ()> {
                                r#type: #inner_after,
                                partial: (),
                                after: ()
                            }
                        },
                        after: Template::<#inner_after, (), ()> {
                            r#type: #inner_after,
                            partial: (),
                            after: ()
                        }
                    }
                })
            });
            quote! {
                Template::<
                    #name,
                    Template::<#inner_partial, (), Template::<#inner_after, (), ()>>,
                    Template::<#inner_after, (), ()>
                > #create
            }
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                template_name.to_upper_camel_case(),
                node_index.index().to_string(),
            );
            let create = create.then(|| {
                Some(quote! {
                    { r#type: #ident, partial: #partial, after: #after }
                })
            });
            quote! {
                Template::<#ident, #partial_type, #after_type> #create
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
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote! { _magic_expression_result.after }
        } else {
            node_type(
                &template_codegen.template_name,
                &template_codegen.graph,
                edge.target(),
                Span::call_site(),
                &quote! { $template.partial },
                &quote! { $template.after },
                &quote! { _ },
                &quote! { _ },
                true,
            )
        };
        let return_type = if last_node {
            quote! { After }
        } else {
            node_type(
                &template_codegen.template_name,
                &template_codegen.graph,
                edge.target(),
                Span::call_site(),
                &quote! { $template.partial },
                &quote! { $template.after },
                //
                &quote! { Partial },
                &quote! { After },
                false,
            )
        };
        let variable_name = edge.weight().variable.as_ref().map_or_else(
            || {
                format_ident!(
                    "{}_template{}",
                    template_codegen.template_name,
                    edge.id().index()
                )
            },
            |variable| {
                format_ident!(
                    "{}_{}{}",
                    template_codegen.template_name,
                    variable,
                    edge.id().index()
                )
            },
        );
        let parameter = edge
            .weight()
            .variable
            .as_ref()
            .map(|variable| format_ident!("{}", variable))
            .map(|variable| {
                // TODO FIXME
                // <'a, I: Into<Cow<'a, str>>>(input: I) -> Cow<'a, str>
                quote! {
                    , #variable: impl Into<::alloc::borrow::Cow<'static, str>>
                }
            });
        let macro_parameter = edge.weight().variable.as_ref().map(|_| {
            quote! {
                , $value: expr
            }
        });
        let impl_func = match &template_codegen.graph[edge.source()] {
            NodeType::InnerTemplate { .. } | NodeType::PartialBlock { .. } => None,
            NodeType::Other => Some({
                let impl_template_name = format_ident!(
                    "{}Template{}",
                    template_codegen.template_name.to_upper_camel_case(),
                    edge.source().index().to_string(),
                );
                match &template_codegen.graph[edge.target()] {
                    NodeType::PartialBlock { .. } => {
                        quote! {
                            impl<Partial: TemplateTypy,
                                PartialPartial: Templaty,
                                PartialAfter: Templaty,
                                After: Templaty
                                >
                                Template<
                                        #impl_template_name,
                                        Template<Partial, PartialPartial, PartialAfter>,
                                        After
                                        > {
                                pub fn #variable_name(self #parameter) -> #return_type {
                                    todo!()
                                }
                            }
                        }
                    }
                    NodeType::InnerTemplate { .. } | NodeType::Other => {
                        quote! {
                            impl<Partial: Templaty, After: Templaty>
                                Template<#impl_template_name, Partial, After> {
                                pub fn #variable_name(self #parameter) -> #return_type {
                                    todo!()
                                }
                            }
                        }
                    }
                }
            }),
        };
        quote! {
            #[allow(unused)]
            macro_rules! #variable_name {
                ($template: expr #macro_parameter) => { unreachable!(); #next_template_struct }
            }

            #impl_func
        }
    })
}

#[must_use]
pub fn codegen(templates: &[TemplateCodegen]) -> proc_macro2::TokenStream {
    let code = templates.iter().map(|template_codegen| {
        let instructions = calculate_nodes(template_codegen);
        let edges = calculate_edges(template_codegen);
        let ident = format_ident!(
            "{}_initial{}",
            template_codegen.template_name,
            template_codegen.first.index()
        );
        let template_struct = node_type(
            &template_codegen.template_name,
            &template_codegen.graph,
            template_codegen.first,
            Span::call_site(),
            &quote! { () },
            &quote! { () },
            &quote! { _ },
            &quote! { _ },
            true,
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
        impl<Type: TemplateTypy, Partial: Templaty, After: Templaty>
            Templaty for Template<Type, Partial, After> {}

        #(#code)*
    };
    result
}
