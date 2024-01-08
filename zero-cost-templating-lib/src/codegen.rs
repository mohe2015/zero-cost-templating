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

    let partial_after = node_raw_type(graph, node_index, span, partial, after);
    let partial_after_type = partial_after.0;
    let partial_after_create = partial_after.1;

    let common = quote_spanned! {span=>
        Template::<#partial_template_name_type, (), #partial_after_type>
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
        graph,
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

    node_raw_type(graph, node_index, span, partial, after)
}

fn node_raw_type(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    node_index: NodeIndex,
    span: Span,
    partial: &(TokenStream, TokenStream),
    after: &(TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    let partial_type = &partial.0;
    let partial_create = &partial.1;

    let after_type = &after.0;
    let after_create = &after.1;

    let ident = format_ident!(
        "{}Template{}",
        graph[node_index].template_name.to_upper_camel_case(),
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
    template_codegen: &'a TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph.node_references().map(|(node_index, _)| {
        let template_struct = format_ident!(
            "{}Template{}",
            template_codegen.template_name.to_upper_camel_case(),
            node_index.index().to_string(),
        );
        quote! {
            #[must_use]
            #[derive(Clone, Copy)]
            pub struct #template_struct;
        }
    })
}

#[must_use]
pub fn element_to_yield(
    intermediate_ast_element: &IntermediateAstElement,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    // TODO FIXME check for empty string yielding in production
    match &intermediate_ast_element.inner {
        IntermediateAstElementInner::Variable {
            variable_name,
            escaping_fun: EscapingFunction::HtmlAttribute,
        } => {
            let variable_name = format_ident!("{}", variable_name);
            (
                quote! { ::alloc::borrow::Cow<'static, str> },
                quote! {
                    zero_cost_templating::encode_double_quoted_attribute(#variable_name)
                },
            )
        }
        IntermediateAstElementInner::Variable {
            variable_name,
            escaping_fun: EscapingFunction::HtmlElementInner,
        } => {
            let variable_name = format_ident!("{}", variable_name);
            (
                quote! { ::alloc::borrow::Cow<'static, str> },
                quote! {
                    zero_cost_templating::encode_element_text(#variable_name)
                },
            )
        }
        IntermediateAstElementInner::Text(text) => (
            quote! { impl ::std::iter::Iterator<Item = ::alloc::borrow::Cow<'static, str>> },
            quote! {
                gen {
                    yield ::alloc::borrow::Cow::from(#text);
                }
            },
        ),
        IntermediateAstElementInner::InnerTemplate
        | IntermediateAstElementInner::PartialBlockPartial => (
            quote! { impl ::std::iter::Iterator<Item = ::alloc::borrow::Cow<'static, str>> },
            quote! {
                gen {

                }
            },
        ),
    }
}

#[expect(clippy::too_many_lines, reason = "tmp")]
#[must_use]
pub fn calculate_edge(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &TemplateCodegen,
    edge: petgraph::stable_graph::EdgeReference<'_, IntermediateAstElement>,
) -> proc_macro2::TokenStream {
    // TODO FIXME only add number when multiple outgoing edges
    // (add the number to documentation to aid in debugging)

    let function_name = edge.weight().variable_name().as_ref().map_or_else(
        || {
            format_ident!(
                "next{}",
                if edge.weight().tag.is_empty() {
                    String::new()
                } else {
                    "_".to_owned() + &edge.weight().tag
                }
            )
        },
        |variable| {
            format_ident!(
                "{}{}",
                variable,
                if edge.weight().tag.is_empty() {
                    String::new()
                } else {
                    "_".to_owned() + &edge.weight().tag
                }
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

    let impl_template_name = format_ident!(
        "{}Template{}",
        template_codegen.template_name.to_upper_camel_case(),
        edge.source().index().to_string(),
    );
    let (yield_return_type, yield_value) = element_to_yield(edge.weight());
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
                    quote! { Template<PartialName, PartialPartial, PartialAfter> },
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
                    Template<
                            #impl_template_name,
                            Template<PartialName, PartialPartial, PartialAfter>,
                            After
                            > {
                    pub fn #function_name(self #parameter) -> (#return_type,
                            #yield_return_type) {
                        (#return_create, #yield_value)
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
                        Template<#impl_template_name, Partial, After> {

                        pub fn #function_name(self #parameter) -> (#return_type,
                                #yield_return_type) {
                            (#return_create, #yield_value)
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
    template_codegen: &'a TemplateCodegen,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    graph
        .edge_references()
        .filter(|edge| {
            edge.weight().inner != IntermediateAstElementInner::PartialBlockPartial
                && edge.weight().inner != IntermediateAstElementInner::InnerTemplate
        })
        .map(|edge| calculate_edge(graph, template_codegen, edge))
}

#[must_use]
pub fn codegen_template_codegen(
    graph: &StableGraph<TemplateNode, IntermediateAstElement>,
    template_codegen: &TemplateCodegen,
) -> proc_macro2::TokenStream {
    let instructions = calculate_nodes(graph, template_codegen);
    let edges = calculate_edges(graph, template_codegen);
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

        #(#instructions)*

        #(#edges)*

        #[allow(unused)]
        /// Start
        pub fn #ident() -> (#template_struct_type,
                impl ::std::iter::Iterator<Item = ::alloc::borrow::Cow<'static, str>>) {
            (#template_struct_create, gen {})
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
        #[derive(Clone, Copy)]
        pub struct Template<Type, Partial, After> {
            r#type: Type,
            partial: Partial,
            after: After,
        }

        #(#code)*
    };
    result
}
