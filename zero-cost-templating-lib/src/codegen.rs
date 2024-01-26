use std::path::PathBuf;

use heck::ToUpperCamelCase;
use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};

use crate::intermediate_graph::{
    EscapingFunction, IntermediateAstElement, IntermediateAstElementInner, NodeType, TemplateNode,
};

/// design: all nodes have a struct type so you can go too all nodes
/// partial block works by going inside on the incoming edge and
/// the outgoing edge goes back to the partial node.
/// this makes it possible to directly use an if
/// after the partial block or another partial block without special casing

/// multiple partial blocks after each other should work

/// partial is only the type of the node itself
/// (maybe we could change this to be a full node type (with Template::?))
fn node_partial_block_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
    partial_template_name: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    assert_eq!(
        graph[node_index].node_type,
        NodeType::PartialBlock,
        "must be NodeType::PartialBlock"
    );

    let partial_template_name_type = &partial_template_name.0;
    let partial_template_name_create = &partial_template_name.1;

    let partial_after = node_raw_type(node_index, span, partial, after);
    let partial_after_type = partial_after.0;
    let partial_after_create = partial_after.1;

    let common = quote_spanned! {span=>
        Tp::<#partial_template_name_type, (), #partial_after_type>
    };
    let create = quote_spanned! {span=>
    {
        r#type: #partial_template_name_create,
        partial: (),
        after: #partial_after_create
    }
    };

    (
        common.clone(),
        quote_spanned! {span=>
            #common #create
        },
    )
}

/// the same design for inner template:
/// the node itself will be visited when going out of the inner template

fn node_inner_template_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    assert_eq!(
        graph[node_index].node_type,
        NodeType::InnerTemplate,
        "must be NodeType::InnerTemplate"
    );

    let inner_after = node_raw_type(
        node_index,
        span,
        partial,
        &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
    );

    let inner_partial = graph
        .edges_directed(node_index, Direction::Outgoing)
        .filter(|edge| edge.weight().inner == IntermediateAstElementInner::PartialBlockPartial)
        .exactly_one()
        .unwrap();

    let _inner_partial_empty = graph
        .edges_directed(inner_partial.target(), Direction::Outgoing)
        .next()
        .is_none();

    // maybe don't do this?
    //let inner_partial = if inner_partial_empty {
    //    (quote_spanned! {span=> () }, quote_spanned! {span=> () })
    //} else {
    let inner_partial = node_type(
        graph,
        inner_partial.target(),
        span,
        &(quote_spanned! {span=> () }, quote_spanned! {span=> () }),
        &inner_after,
    );
    // };

    let inner_template = graph
        .edges_directed(node_index, Direction::Outgoing)
        .filter(|edge| edge.weight().inner == IntermediateAstElementInner::InnerTemplate)
        .exactly_one()
        .unwrap();

    node_type(
        graph,
        inner_template.target(),
        span,
        &inner_partial,
        &inner_after,
    )
}

fn node_other_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    assert_eq!(
        graph[node_index].node_type,
        NodeType::Other,
        "must be NodeType::Other"
    );

    let last_node = graph
        .edges_directed(node_index, Direction::Outgoing)
        .next()
        .is_none();
    if last_node {
        return after.clone();
    }

    node_raw_type(node_index, span, partial, after)
}

fn node_raw_type(
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    let partial_type = &partial.0;
    let partial_create = &partial.1;

    let after_type = &after.0;
    let after_create = &after.1;

    let ident = format_ident!("Tp{}", node_index.index().to_string(), span = span);
    let common = quote_spanned! {span=>
        Tp::<#ident, #partial_type, #after_type>
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

/// return.0 is type and return.1 is create expression
/// This method's only responsibility is to convert the node to a type and creation ``TokenStream``.
fn node_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    let node = &graph[node_index];
    match node.node_type {
        NodeType::PartialBlock => panic!(),
        NodeType::InnerTemplate => node_inner_template_type(graph, node_index, span, partial),
        NodeType::Other => node_other_type(graph, node_index, span, partial, after),
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
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph.node_references().map(|(node_index, node)| {
        let template_struct = format_ident!("Tp{}", node_index.index().to_string(),);
        let name = node.template_name.to_upper_camel_case();
        quote! {
            #[must_use]
            #[derive(Clone, Copy)]
            #[doc = #name]
            pub struct #template_struct;
        }
    })
}

#[must_use]
pub fn element_to_yield(
    intermediate_ast_element: &IntermediateAstElement,
) -> proc_macro2::TokenStream {
    // TODO FIXME check for empty string yielding
    match &intermediate_ast_element.inner {
        IntermediateAstElementInner::Variable {
            before,
            variable_name,
            escaping_fun: EscapingFunction::HtmlAttribute,
            after,
        } => {
            let variable_name = format_ident!("{}", variable_name);

            quote! {
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#before.as_bytes())).await;
                ::zero_cost_templating::FutureToStream(())._yield(zero_cost_templating::encode_double_quoted_attribute(#variable_name)).await;
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#after.as_bytes())).await;
            }
        }
        IntermediateAstElementInner::Variable {
            before,
            variable_name,
            escaping_fun: EscapingFunction::HtmlElementInner,
            after,
        } => {
            let variable_name = format_ident!("{}", variable_name);

            quote! {
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#before.as_bytes())).await;
                ::zero_cost_templating::FutureToStream(())._yield(zero_cost_templating::encode_element_text(#variable_name)).await;
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#after.as_bytes())).await;
            }
        }
        IntermediateAstElementInner::Variable {
            before,
            variable_name,
            escaping_fun: EscapingFunction::Unsafe,
            after,
        } => {
            let variable_name = format_ident!("{}", variable_name);

            quote! {
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#before.as_bytes())).await;
                ::zero_cost_templating::FutureToStream(())._yield(#variable_name.get_unsafe_input().into()).await;
                ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#after.as_bytes())).await;
            }
        }
        IntermediateAstElementInner::Text(text) => quote! {
            ::zero_cost_templating::FutureToStream(())._yield(::bytes::Bytes::from_static(#text.as_bytes())).await;
        },
        IntermediateAstElementInner::InnerTemplate
        | IntermediateAstElementInner::PartialBlockPartial => quote! {},
    }
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn calculate_edge(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    edge: petgraph::stable_graph::EdgeReference<'_, IntermediateAstElement>,
) -> proc_macro2::TokenStream {
    // TODO FIXME only add number when multiple outgoing edges
    // (add the number to documentation to aid in debugging)

    let function_header = match &edge.weight().inner {
        IntermediateAstElementInner::Variable {
            variable_name,
            escaping_fun,
            ..
        } => {
            let function_name = format_ident!(
                "{}{}{}",
                variable_name,
                if edge.weight().tag.is_empty() {
                    String::new()
                } else {
                    "_".to_owned() + &edge.weight().tag
                },
                if matches!(escaping_fun, EscapingFunction::Unsafe) {
                    "_unsafe".to_owned()
                } else {
                    String::new()
                }
            );
            let variable_name = format_ident!("{}", variable_name);
            let variable_type = if matches!(escaping_fun, EscapingFunction::Unsafe) {
                quote! { ::zero_cost_templating::Unsafe<impl Into<::alloc::borrow::Cow<'static, str>>> }
            } else {
                quote! { impl Into<::alloc::borrow::Cow<'static, str>> }
            };
            quote! { #function_name(self, #variable_name: #variable_type) }
        }
        IntermediateAstElementInner::Text(_)
        | IntermediateAstElementInner::PartialBlockPartial
        | IntermediateAstElementInner::InnerTemplate => {
            let function_name = format_ident!(
                "next{}",
                if edge.weight().tag.is_empty() {
                    String::new()
                } else {
                    "_".to_owned() + &edge.weight().tag
                }
            );
            quote! { #function_name(self) }
        }
    };

    let impl_template_name = format_ident!("Tp{}", edge.source().index().to_string(),);
    let yield_value = element_to_yield(edge.weight());
    // TODO FIXME the ` makes doc test parsing fail
    let _documentation = format!(
        "Transition from `{}: {}` to `{}: {}` using `{}: {}`",
        edge.source().index(),
        graph[edge.source()],
        edge.target().index(),
        graph[edge.target()],
        edge.id().index(),
        edge.weight()
    );
    let impl_func = match (
        &graph[edge.source()].node_type,
        &graph[edge.target()].node_type,
    ) {
        (_, NodeType::PartialBlock) => Some({
            let r#return = node_partial_block_type(
                graph,
                edge.target(),
                Span::call_site(),
                &(
                    quote! { Tp<PartialName, PartialPartial, PartialAfter> },
                    quote! { self.partial },
                ),
                &(quote! { PartialName }, quote! { self.partial.r#type }),
                &(quote! { After }, quote! { self.after }),
            );
            let return_type = r#return.0;
            let return_create = r#return.1;
            quote! {
                impl<PartialName: Copy,
                    PartialPartial,
                    PartialAfter,
                    After
                    >
                    Tp<
                            #impl_template_name,
                            Tp<PartialName, PartialPartial, PartialAfter>,
                            After
                            > {
                    pub async fn #function_header -> #return_type {
                        #yield_value
                        #return_create
                    }
                }
            }
        }),
        (_, NodeType::InnerTemplate | NodeType::Other) => {
            let r#return = node_type(
                graph,
                edge.target(),
                Span::call_site(),
                &(quote! { Partial }, quote! { self.partial }),
                &(quote! { After }, quote! { self.after }),
            );
            let return_type = r#return.0;
            let return_create = r#return.1;
            Some({
                quote! {
                    impl<Partial: Copy, After>
                    Tp<#impl_template_name, Partial, After> {

                        pub async fn #function_header -> #return_type {
                            #yield_value
                            #return_create
                        }
                    }
                }
            })
        }
    };
    quote! {
        #impl_func
    }
}

pub fn calculate_edges<'a>(
    graph: &'a StableGraph<TemplateNode, IntermediateAstElement>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph
        .edge_references()
        .filter(|edge| {
            edge.weight().inner != IntermediateAstElementInner::PartialBlockPartial
                && edge.weight().inner != IntermediateAstElementInner::InnerTemplate
        })
        .map(|edge| calculate_edge(graph, edge))
}

#[must_use]
pub fn codegen_template_codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &TemplateCodegen,
) -> proc_macro2::TokenStream {
    let ident = format_ident!("{}", template_codegen.template_name,);
    let template_struct = node_type(
        graph,
        template_codegen.first,
        Span::call_site(),
        &(quote! { () }, quote! { () }),
        &(quote! { () }, quote! { () }),
    );
    let template_struct_type = template_struct.0;
    let template_struct_create = template_struct.1;
    let recompile_ident = format_ident!("_{}_FORCE_RECOMPILE", template_codegen.template_name);
    let path = template_codegen.path.to_string_lossy();
    quote! {


        #[allow(unused)]
        /// Start
        pub fn #ident(stream: ::zero_cost_templating::FutureToStream) -> #template_struct_type {
            #template_struct_create
        }

        const #recompile_ident: &'static str = include_str!(#path);
    }
}

#[must_use]
pub fn codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    templates: &[TemplateCodegen],
) -> proc_macro2::TokenStream {
    let instructions = calculate_nodes(graph);
    let edges = calculate_edges(graph);

    let code = templates
        .iter()
        .map(|template_codegen| codegen_template_codegen(graph, template_codegen));

    let result = quote! {
        #[must_use]
        #[derive(Clone, Copy)]
        pub struct Tp<Type, Partial, After> {
            r#type: Type,
            partial: Partial,
            after: After,
        }

        #(#instructions)*

        #(#edges)*

        #(#code)*
    };
    result
}
