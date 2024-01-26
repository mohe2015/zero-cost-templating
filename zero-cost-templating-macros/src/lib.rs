use std::collections::{BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use itertools::peek_nth;
use petgraph::dot::{Config, Dot};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{EdgeRef, NodeRef};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Item, LitStr, Token};
use zero_cost_templating_lib::codegen::{codegen, TemplateCodegen};
use zero_cost_templating_lib::html_recursive_descent::parse_children;
use zero_cost_templating_lib::intermediate_graph::{
    children_to_ast, flush_with_node, IntermediateAstElementInner, NodeType, TemplateNode,
};

// https://veykril.github.io/posts/ide-proc-macros/
// https://github.com/rust-lang/rust-analyzer/pull/11444
// https://github.com/rust-lang/rust-analyzer/issues/11014
// https://github.com/intellij-rust/intellij-rust/pull/9711
// https://github.com/yewstack/yew/pull/2972

#[proc_macro_attribute]
#[allow(clippy::too_many_lines)]
pub fn template_stream(
    attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_directories = parse_macro_input!(attributes
        with Punctuated::<LitStr, Token![,]>::parse_separated_nonempty);
    // https://github.com/dtolnay/trybuild/issues/202
    let cargo_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR_OVERRIDE")
        .or_else(|| std::env::var_os("CARGO_MANIFEST_DIR"))
        .unwrap();

    let root = PathBuf::from(&cargo_manifest_dir);

    let inputs: Vec<_> = input_directories
        .iter()
        .flat_map(|directory| {
            let path = root.join(directory.value());
            fs::read_dir(path)
                .unwrap()
                .map(core::result::Result::unwrap)
        })
        .map(|file| {
            let path = file.path();
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();

            let template_name = file_name.trim_end_matches(".html.hbs");

            (path, template_name.to_owned())
        })
        .collect();

    let mut graph = StableGraph::new();
    let graph = &mut graph;
    let first_nodes: HashMap<_, _> = inputs
        .iter()
        .map(|(_path, template_name)| {
            let first = graph.add_node(TemplateNode {
                template_name: template_name.to_owned(),
                node_type: NodeType::Other,
            });

            (template_name.clone(), first)
        })
        .collect();

    let inputs: Vec<_> = inputs
        .iter()
        .map(|(path, template_name)| {
            // TODO FIXME error if end doesn't match

            let input = std::fs::read_to_string(path).unwrap_or_else(|err| {
                // TODO FIXME don't panic
                panic!("failed to read file at path: {} {}", path.display(), err)
            });

            let mut input = peek_nth(input.chars());
            let dom = match parse_children(&mut input) {
                Ok(element) => {
                    let remaining_input: String = input.collect();
                    assert_eq!(
                        remaining_input,
                        "",
                        "File: {}\n{element:?}\nremaining input: {remaining_input}",
                        path.display()
                    );
                    element
                }
                Err(error) => {
                    let remaining_input: String = input.collect();
                    panic!(
                        "File: {}\n{error}\nremaining input: {remaining_input}",
                        path.display()
                    );
                }
            };
            let first = first_nodes.get(template_name).unwrap();
            let tmp = children_to_ast(
                &first_nodes,
                template_name,
                graph,
                BTreeSet::from([(*first, None)]),
                dom,
                "root",
            );
            let last = flush_with_node(
                graph,
                tmp,
                TemplateNode {
                    template_name: template_name.clone(),
                    node_type: NodeType::Other,
                },
            );

            TemplateCodegen {
                template_name: template_name.to_owned(),
                path: path.to_owned(),
                first: *first,
                last,
            }
        })
        .collect();

    let mut file = File::create("template_graph.dot").unwrap();
    let immut_graph = &*graph;
    file.write_all(
        format!(
            "{:?}",
            Dot::with_attr_getters(
                immut_graph,
                &[Config::NodeNoLabel, Config::EdgeNoLabel],
                &|_, er| match er.weight().inner {
                    IntermediateAstElementInner::InnerTemplate => {
                        format!(
                            "label = \"{}: {}\" style = dashed color = blue",
                            er.id().index(),
                            er.weight().to_string().replace('\"', "\\\"")
                        )
                    }
                    IntermediateAstElementInner::PartialBlockPartial => {
                        format!(
                            "label = \"{}: {}\" style = dashed color = orange",
                            er.id().index(),
                            er.weight().to_string().replace('\"', "\\\"")
                        )
                    }
                    _ => {
                        format!(
                            "label = \"{}: {}\" color = red",
                            er.id().index(),
                            er.weight().to_string().replace('\"', "\\\"")
                        )
                    }
                },
                &|_, nr| match nr.weight().node_type {
                    NodeType::PartialBlock => format!(
                        "label = \"{}: {}\" color = orange",
                        nr.id().index(),
                        nr.weight().to_string().replace('\"', "\\\"")
                    ),
                    NodeType::InnerTemplate => format!(
                        "label = \"{}: {}\" color = blue",
                        nr.id().index(),
                        nr.weight().to_string().replace('\"', "\\\"")
                    ),
                    NodeType::Other => format!(
                        "label = \"{}: {}\" color = red",
                        nr.id().index(),
                        nr.weight().to_string().replace('\"', "\\\"")
                    ),
                },
            )
        )
        .as_bytes(),
    )
    .unwrap();

    let code = codegen(graph, &inputs);

    let item = parse_macro_input!(item as Item);

    let expanded = quote! {

        #code

        #item
    };

    // TODO FIXME remove for production
    if let Err(error) = syn::parse2::<syn::File>(expanded.clone()) {
        panic!("{error}\n{expanded}")
    }

    proc_macro::TokenStream::from(expanded)
}
