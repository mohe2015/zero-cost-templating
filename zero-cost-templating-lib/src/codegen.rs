use std::path::PathBuf;

use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};

use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{visit_mut, Expr, ExprMethodCall, ExprPath};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType, TemplateNode};

pub struct InnerReplace<'a>(
    pub Vec<TemplateCodegen>,
    pub &'a StableGraph<TemplateNode, IntermediateAstElement>,
);

fn handle_call(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    ident: &Ident,
    template_variable: &syn::Expr,
    parameter: Option<&syn::Expr>,
    template_codegen: &TemplateCodegen,
    span: Span,
) -> Option<Expr> {
    let edge = graph.edge_references().find(|edge| {
        let expected_ident = format_ident!(
            "{}_{}{}",
            template_codegen.template_name,
            edge.weight()
                .variable_name()
                .unwrap_or(&"template".to_owned()),
            edge.id().index(),
            span = span
        );
        edge.weight().variable_name().is_some() == parameter.is_some() && ident == &expected_ident
    });
    edge.map(|edge| {
        let template_struct = node_type(
            graph,
            edge.source(),
            &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
            &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
            span,
        ).0; // good span for mismatched type error
        let last_node = graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote_spanned! {span=> _magic_expression_result.after }
        } else {
            node_type(
                graph,
                edge.target(),
                &(quote_spanned! {span=> _ }, quote_spanned! {span=> _magic_expression_result.partial }),
                &(quote_spanned! {span=> _ }, quote_spanned! {span=> _magic_expression_result.after }),
                span,
            ).1
        };

        // TODO FIXME fix unwrap by better matching here in general
        let escaped_value = parameter.map(|parameter| match edge.weight().variable().unwrap().1 {
            EscapingFunction::HtmlAttribute => {
                quote_spanned! {span=>
                    yield zero_cost_templating::encode_double_quoted_attribute(#parameter);
                }
            }
            EscapingFunction::HtmlElementInner => {
                quote_spanned! {span=>
                    yield zero_cost_templating::encode_element_text(#parameter);
                }
            }
        });

        let text = &edge.weight().text().map(|text| {
            quote_spanned! {span=>
                yield ::alloc::borrow::Cow::from(#text);
            }
        });

        Expr::Verbatim(quote_spanned! {span=>
            {
                let _magic_expression_result: #template_struct = #template_variable;
                #escaped_value
                #text
                #next_template_struct
            }
        })
    })
}

impl<'a> InnerReplace<'a> {
    fn magic_method_call(&self, input: &ExprMethodCall) -> Option<syn::Expr> {
        let ident = &input.method;
        match input.args.len() {
            0 | 1 => self.0.iter().find_map(|template_codegen| {
                handle_call(
                    self.1,
                    ident,
                    &input.receiver,
                    input.args.first(),
                    template_codegen,
                    input.span(),
                )
            }),
            _ => None,
        }
    }
}

impl<'a> VisitMut for InnerReplace<'a> {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        let span = node.span();
        match node {
            Expr::MethodCall(expr_method_call) => {
                if let Some(result) = self.magic_method_call(expr_method_call) {
                    *node = result;
                }
            }
            Expr::Call(expr_call) => {
                let ident = match &expr_call.func {
                    box Expr::Path(ExprPath { path, .. }) if expr_call.args.is_empty() => {
                        path.get_ident()
                    }
                    _ => None,
                };
                if let Some(ident) = ident {
                    let result = self.0.iter().find_map(|template_codegen| {
                        let first_index = template_codegen.first.index();
                        let initial_ident = format_ident!(
                            "{}_initial{}",
                            template_codegen.template_name,
                            first_index,
                            span = span,
                        );
                        (&initial_ident == ident).then(|| {
                            let template_struct = node_type(
                                self.1,
                                template_codegen.first,
                                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                                span,
                            ).1;
                            Expr::Verbatim(quote_spanned! {span=>
                                #template_struct
                            })
                        })
                    });
                    if let Some(result) = result {
                        *node = result;
                    }
                }
            }
            _ => {
                visit_mut::visit_expr_mut(self, node);
            }
        }
    }
}

#[expect(clippy::too_many_lines, reason = "tmp")]
#[expect(clippy::too_many_arguments, reason = "tmp")]
/// return.0 is type and return.1 is create expression
fn node_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    partial: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
    span: Span,
) -> (TokenStream, TokenStream) {
    let partial_type = &partial.0;
    let partial_create = &partial.1;

    let after_type = &after.0;
    let after_create = &after.1;

    let node = &graph[node_index];
    match node.node_type {
        NodeType::PartialBlock => {
            let inner_after = graph
                .edges_directed(node_index, Direction::Outgoing)
                .exactly_one()
                .unwrap();
            let inner_after = node_type(
                graph,
                inner_after.target(),
                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                span,
            );
            let inner_after_type = inner_after.0;
            let inner_after_create = inner_after.1;
          
            let common = quote_spanned! {span=>
                Template::<#partial_type, (), Template::<#inner_after_type, (), #after_type>>
            };
            let create = quote_spanned! {span=>
                {
                    r#type: #partial_create.r#type,
                    partial: (),
                    after: Template { r#type: #inner_after_create, partial: (), after: #after_create }
                }
            };

            (common.clone(), quote_spanned! {span=>
                #common #create
            })
        }
        NodeType::InnerTemplate => {
            let inner_after = graph
                .edges_directed(node_index, Direction::Outgoing)
                .filter(|edge| {
                    *edge.weight() != IntermediateAstElement::InnerTemplate
                        && *edge.weight() != IntermediateAstElement::PartialBlockPartial
                })
                .exactly_one()
                .unwrap();
            let inner_after = node_type(
                graph,
                inner_after.target(),
                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                span,
            );

            let inner_partial = graph
                .edges_directed(node_index, Direction::Outgoing)
                .filter(|edge| *edge.weight() == IntermediateAstElement::PartialBlockPartial)
                .exactly_one()
                .unwrap();
            let inner_partial = node_type(
                graph,
                inner_partial.target(),
                &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
                &inner_after,
                span,
            );

            let inner_template = graph
                .edges_directed(node_index, Direction::Outgoing)
                .filter(|edge| *edge.weight() == IntermediateAstElement::InnerTemplate)
                .exactly_one()
                .unwrap();
            node_type(
                graph,
                inner_template.target(),
                &inner_partial,
                &inner_after,
                span,
            )
        }
        NodeType::Other => {
            let ident = format_ident!(
                "{}Template{}",
                node.template_name.to_upper_camel_case(),
                node_index.index().to_string(),
                span = span
            );
            let common =  quote_spanned! {span=>
                Template::<#ident, #partial_type, #after_type>
            };
            let create = quote_spanned! {span=>
                { r#type: #ident, partial: #partial_create, after: #after_create }
            };
            (common.clone(), quote_spanned! {span=>
                #common #create
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateCodegen {
    pub path: PathBuf,
    pub template_name: String,
    pub first: NodeIndex,
    pub last: NodeIndex,
}

pub fn calculate_nodes<'a>(
    graph: &'a StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &'a TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph
        .node_references()
        .filter_map(|(node_index, node)| match node.node_type {
            NodeType::InnerTemplate | NodeType::PartialBlock => None,
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
            }
        })
}

pub fn calculate_edges<'a>(
    graph: &'a StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &'a TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph.edge_references().map(|edge| {
        let last_node = graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let return_type = if last_node {
            quote! { After }
        } else {
            node_type(
                graph,
                edge.target(),
                &(quote! { Partial }, quote! { template.partial }),
                &(quote! { After }, quote! { template.after }),
                Span::call_site(),
            ).0
        };
        let variable_name = edge.weight().variable_name().as_ref().map_or_else(
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
            .variable_name()
            .as_ref()
            .map(|variable| format_ident!("{}", variable))
            .map(|variable| {
                quote! {
                    , #variable: impl Into<::alloc::borrow::Cow<'static, str>>
                }
            });
        let impl_func = match &graph[edge.source()].node_type {
            NodeType::InnerTemplate | NodeType::PartialBlock => None,
            NodeType::Other => Some({
                let impl_template_name = format_ident!(
                    "{}Template{}",
                    template_codegen.template_name.to_upper_camel_case(),
                    edge.source().index().to_string(),
                );
                match &graph[edge.target()].node_type {
                    NodeType::PartialBlock => {
                        quote! {
                            impl<Partial,
                                PartialPartial,
                                PartialAfter,
                                After
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
                    NodeType::InnerTemplate | NodeType::Other => {
                        quote! {
                            impl<Partial, After>
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
            #impl_func
        }
    })
}

#[must_use]
pub fn codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    templates: &[TemplateCodegen],
) -> proc_macro2::TokenStream {
    let code = templates.iter().map(|template_codegen| {
        let instructions = calculate_nodes(graph, template_codegen);
        let edges = calculate_edges(graph, template_codegen);
        let ident = format_ident!(
            "{}_initial{}",
            template_codegen.template_name,
            template_codegen.first.index()
        );
        let template_struct = node_type(
            graph,
            template_codegen.first,
            &(quote! { () }, quote! { () }),
            &(quote! { () }, quote! { () }),
            Span::call_site(),
        ).0;
        let other = quote! {
            #[allow(unused)]
            /// Start
            pub fn #ident() -> #template_struct {
                unreachable!("Start")
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
        #[must_use]
        pub struct Template<Type, Partial, After> {
            r#type: Type,
            partial: Partial,
            after: After,
        }

        #(#code)*
    };
    result
}
