use std::path::PathBuf;

use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType, TemplateNode};

#[expect(clippy::too_many_lines, reason = "tmp")]
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
                    after: Template {
                        r#type: #inner_after_create,
                        partial: (),
                        after: #after_create
                    }
                }
            };

            (
                common.clone(),
                quote_spanned! {span=>
                    #common #create
                },
            )
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

            // TODO FIXME implement empty partial or prevent empty partial

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
            let common = quote_spanned! {span=>
                Template::<#ident, #partial_type, #after_type>
            };
            let create = quote_spanned! {span=>
                { r#type: #ident, partial: #partial_create, after: #after_create }
            };
            (
                common.clone(),
                quote_spanned! {span=>
                    #common #create
                },
            )
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

                impl<Partial, After>
                    Template<#template_struct, Partial, After> {
                    pub fn up(self) -> (After,
                            impl ::std::async_iter::AsyncIterator<Item =
                                ::alloc::borrow::Cow<'static, str>>) {
                        (self.after, async gen {})
                    }
                }
            }
        })
}

#[must_use]
pub fn element_to_yield(
    intermediate_ast_element: &IntermediateAstElement,
) -> proc_macro2::TokenStream {
    match intermediate_ast_element {
        IntermediateAstElement::Variable(variable_name, EscapingFunction::HtmlAttribute) => {
            let variable_name = format_ident!("{}", variable_name);
            quote! {
                yield zero_cost_templating::encode_double_quoted_attribute(#variable_name);
            }
        }
        IntermediateAstElement::Variable(variable_name, EscapingFunction::HtmlElementInner) => {
            let variable_name = format_ident!("{}", variable_name);
            quote! {
                yield zero_cost_templating::encode_element_text(#variable_name);
            }
        }
        IntermediateAstElement::Text(text) => {
            quote! {
                yield ::alloc::borrow::Cow::from(#text);
            }
        }
        IntermediateAstElement::Noop => {
            quote! {}
        }
        v => unreachable!("unexpected value to yield {:?}", v),
    }
}

#[must_use]
pub fn calculate_edge(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &TemplateCodegen,
    edge: petgraph::stable_graph::EdgeReference<'_, IntermediateAstElement>,
) -> proc_macro2::TokenStream {
    let r#return = node_type(
        graph,
        edge.target(),
        &(quote! { Partial }, quote! { self.partial }),
        &(quote! { After }, quote! { self.after }),
        Span::call_site(),
    );
    let return_type = r#return.0;
    let return_create = r#return.1;
    let function_name = edge.weight().variable_name().as_ref().map_or_else(
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
    let variable_name = edge
        .weight()
        .variable_name()
        .as_ref()
        .map(|variable| format_ident!("{}", variable));
    let parameter = variable_name.as_ref().map(|variable| {
        quote! {
            , #variable: impl Into<::alloc::borrow::Cow<'static, str>>
        }
    });
    let impl_func = match (
        &graph[edge.source()].node_type,
        &graph[edge.target()].node_type,
    ) {
        (NodeType::InnerTemplate | NodeType::PartialBlock, _) => None,
        (NodeType::Other, NodeType::PartialBlock) => Some({
            let impl_template_name = format_ident!(
                "{}Template{}",
                template_codegen.template_name.to_upper_camel_case(),
                edge.source().index().to_string(),
            );

            let to_yield = element_to_yield(edge.weight());

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
                    pub fn #function_name(self #parameter) -> (#return_type,
                            impl ::std::async_iter::AsyncIterator<Item =
                                ::alloc::borrow::Cow<'static, str>>) {
                        (#return_create, async gen {
                            #to_yield
                        })
                    }
                }

                // empty partial
                impl<After> Template<#impl_template_name, (), After> {
                    pub fn #function_name(self #parameter) -> (After,
                            impl ::std::async_iter::AsyncIterator<Item =
                                ::alloc::borrow::Cow<'static, str>>) {
                        (self.after, async gen {
                            #to_yield
                        })
                    }
                }
            }
        }),
        (NodeType::Other, NodeType::InnerTemplate | NodeType::Other) => Some({
            let impl_template_name = format_ident!(
                "{}Template{}",
                template_codegen.template_name.to_upper_camel_case(),
                edge.source().index().to_string(),
            );

            let to_yield = element_to_yield(edge.weight());

            quote! {
                impl<Partial, After>
                    Template<#impl_template_name, Partial, After> {
                    pub fn #function_name(self #parameter) -> (#return_type,
                            impl ::std::async_iter::AsyncIterator<Item =
                                ::alloc::borrow::Cow<'static, str>>) {
                        (#return_create, async gen {
                            #to_yield
                        })
                    }
                }
            }
        }),
    };
    quote! {
        #impl_func
    }
}

pub fn calculate_edges<'a>(
    graph: &'a StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &'a TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph
        .edge_references()
        .map(|edge| calculate_edge(graph, template_codegen, edge))
}

#[must_use]
pub fn codegen_template_codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &TemplateCodegen,
) -> proc_macro2::TokenStream {
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
    );
    let template_struct_type = template_struct.0;
    let template_struct_create = template_struct.1;
    let recompile_ident = format_ident!("_{}_FORCE_RECOMPILE", template_codegen.template_name);
    let path = template_codegen.path.to_string_lossy();
    quote! {

        #(#instructions)*

        #(#edges)*

        #[allow(unused)]
        /// Start
        pub fn #ident() -> (#template_struct_type,
                impl ::std::async_iter::AsyncIterator<Item = ::alloc::borrow::Cow<'static, str>>) {
            (#template_struct_create, async gen {})
        }

        const #recompile_ident: &'static str = include_str!(#path);
    }
}

#[must_use]
pub fn codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    templates: &[TemplateCodegen],
) -> proc_macro2::TokenStream {
    let code = templates
        .iter()
        .map(|template_codegen| codegen_template_codegen(graph, template_codegen));

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
