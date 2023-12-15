use std::path::PathBuf;

use heck::ToUpperCamelCase;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences};
use petgraph::Direction;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};

use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{parse_quote_spanned, visit_mut, Expr, ExprCall, ExprMethodCall, ExprPath, Token};

use crate::intermediate_graph::{EscapingFunction, IntermediateAstElement, NodeType};

pub struct InnerReplace(pub Vec<TemplateCodegen>);

fn handle_call(
    ident: &Ident,
    template_variable: &syn::Expr,
    parameter: Option<&syn::Expr>,
    semicolon: Option<Token![;]>,
    template_codegen: &TemplateCodegen,
    span: Span,
) -> Option<Expr> {
    let edge = template_codegen.graph.edge_references().find(|edge| {
        let expected_ident = format_ident!(
            "{}_{}{}",
            template_codegen.template_name,
            edge.weight()
                .variable
                .as_ref()
                .unwrap_or(&"template".to_owned()),
            edge.id().index(),
            span = span
        );
        edge.weight().variable.is_some() == parameter.is_some() && ident == &expected_ident
    });
    edge.map(|edge| {
        let text = &edge.weight().text;

        let template_struct = node_type(
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            edge.source(),
            &quote_spanned! {span=> () },
            &quote_spanned! {span=> () },
            &quote_spanned! {span=> _ },
            &quote_spanned! {span=> _ },
            false,
            span,
        ); // good span for mismatched type error
        let last_node = template_codegen
            .graph
            .edges_directed(edge.target(), Direction::Outgoing)
            .next()
            .is_none();
        let next_template_struct = if last_node {
            quote_spanned! {span=> _magic_expression_result.after }
        } else {
            node_type(
                template_codegen.template_name.as_str(),
                &template_codegen.graph,
                edge.target(),
                &quote_spanned! {span=> _magic_expression_result.partial },
                &quote_spanned! {span=> _magic_expression_result.after },
                &quote_spanned! {span=> _ },
                &quote_spanned! {span=> _ },
                true,
                span,
            )
        };

        let escaped_value = parameter.map(|parameter| match edge.weight().escaping_fun {
            EscapingFunction::NoVariableStart => quote_spanned! {span=>
                unreachable("NoVariableStart");
            },
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
        Expr::Verbatim(quote_spanned! {span=>
            let _magic_expression_result: #template_struct = #template_variable;
            #escaped_value
            yield ::alloc::borrow::Cow::from(#text);
            #next_template_struct #semicolon
        })
    })
}

impl InnerReplace {
    fn magic_method_call(
        &self,
        input: &ExprMethodCall,
        semicolon: Option<Token![;]>,
    ) -> Option<syn::Expr> {
        let ident = &input.method;
        match input.args.len() {
            0 | 1 => self.0.iter().find_map(|template_codegen| {
                handle_call(
                    ident,
                    &input.receiver,
                    input.args.first(),
                    semicolon,
                    template_codegen,
                    input.span(),
                )
            }),
            _ => None,
        }
    }
}

impl VisitMut for InnerReplace {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        let span = node.span();
        match node {
            Expr::MethodCall(expr_method_call) => {
                if let Some(result) = self.magic_method_call(expr_method_call, None) {
                    *node = parse_quote_spanned! {span=>
                        if false {
                            #expr_method_call
                        } else {
                            #result
                        }
                    };
                }
            }
            Expr::Call(expr_call @ ExprCall { .. }) => {
                let ident = match &expr_call.func {
                    box Expr::Path(ExprPath { path, .. }) => path.get_ident(),
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
                                template_codegen.template_name.as_str(),
                                &template_codegen.graph,
                                template_codegen.first,
                                &quote_spanned! {span=> () },
                                &quote_spanned! {span=> () },
                                &quote_spanned! {span=> _ },
                                &quote_spanned! {span=> _ },
                                true,
                                span,
                            );
                            Expr::Verbatim(quote_spanned! {span=>
                                #template_struct
                            })
                        })
                    });
                    if let Some(result) = result {
                        *node = parse_quote_spanned! {span=>
                            if false {
                                #expr_call
                            } else {
                                #result
                            }
                        };
                    }
                }
            }
            _ => {
                visit_mut::visit_expr_mut(self, node);
            }
        }
    }
}

#[expect(clippy::too_many_arguments, reason = "tmp")]
fn node_type(
    template_name: &str,
    graph: &StableGraph<NodeType, IntermediateAstElement>,
    node_index: NodeIndex,
    partial: &TokenStream,
    after: &TokenStream,
    partial_type: &TokenStream,
    after_type: &TokenStream,
    create: bool,
    span: Span,
) -> TokenStream {
    match &graph[node_index] {
        NodeType::PartialBlock { after: inner_after } => {
            let inner_after = format_ident!("{}", inner_after, span = span);
            let create = create.then(|| {
                Some(quote_spanned! {span=>
                    {
                        r#type: #partial.r#type,
                        partial: (),
                        after: Template { r#type: #inner_after, partial: (), after: #after }
                    }
                })
            });
            quote_spanned! {span=>
                Template::<#partial_type, (), Template::<#inner_after, (), #after_type>> #create
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
            let create = create.then(|| {
                Some(quote_spanned! {span=>
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
            quote_spanned! {span=>
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
                span = span
            );
            let create = create.then(|| {
                Some(quote_spanned! {span=>
                    { r#type: #ident, partial: #partial, after: #after }
                })
            });
            quote_spanned! {span=>
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
        let return_type = if last_node {
            quote! { After }
        } else {
            node_type(
                &template_codegen.template_name,
                &template_codegen.graph,
                edge.target(),
                &quote! { $template.partial },
                &quote! { $template.after },
                //
                &quote! { Partial },
                &quote! { After },
                false,
                Span::call_site(),
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
            template_codegen.template_name.as_str(),
            &template_codegen.graph,
            template_codegen.first,
            &quote! { () },
            &quote! { () },
            &quote! { () },
            &quote! { () },
            false,
            Span::call_site(),
        );
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
