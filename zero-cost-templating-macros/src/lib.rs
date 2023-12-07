#![forbid(unsafe_code)]
#![warn(
    future_incompatible,
    let_underscore,
    nonstandard_style,
    unused,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::deref_by_slicing,
    clippy::disallowed_script_idents,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::mixed_read_write_in_expression,
    clippy::mod_module_files,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::needless_raw_strings,
    //clippy::panic,
    //clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_type_annotations,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::shadow_unrelated,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::try_err,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    //clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    reason = "not yet ready for that"
)]
#![allow(clippy::shadow_unrelated, reason = "likely useful for templates")]
#![allow(
    clippy::unwrap_used,
    clippy::cargo,
    clippy::unreachable,
    clippy::pattern_type_mismatch,
    clippy::print_stdout,
    clippy::use_debug,
    reason = "development"
)]
#![feature(coroutines)]
#![feature(lint_reasons)]
#![feature(proc_macro_span)]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use itertools::peek_nth;
use petgraph::dot::Dot;
use petgraph::stable_graph::StableGraph;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Item, LitStr, Token};
use zero_cost_templating_lib::codegen::{codegen, InnerMacroReplace, TemplateCodegen};
use zero_cost_templating_lib::html_recursive_descent::parse_children;
use zero_cost_templating_lib::intermediate_graph::{
    children_to_ast, EscapingFunction, IntermediateAstElement, NodeType,
};

// https://veykril.github.io/posts/ide-proc-macros/
// https://github.com/rust-lang/rust-analyzer/pull/11444
// https://github.com/rust-lang/rust-analyzer/issues/11014
// https://github.com/intellij-rust/intellij-rust/pull/9711
// https://github.com/yewstack/yew/pull/2972

#[proc_macro_attribute]
pub fn template_stream(
    attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_paths = parse_macro_input!(attributes with Punctuated::<LitStr, Token![,]>::parse_separated_nonempty);
    // https://github.com/dtolnay/trybuild/issues/202
    let cargo_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR_OVERRIDE")
        .or_else(|| std::env::var_os("CARGO_MANIFEST_DIR"))
        .unwrap();

    let root = PathBuf::from(&cargo_manifest_dir);

    let inputs: Vec<_> = input_paths
        .iter()
        .map(|file| {
            let path = root.join(file.value());

            let file_name = path.file_name().unwrap().to_string_lossy();
            let template_name = file_name.trim_end_matches(".html.hbs");

            let input = std::fs::read_to_string(&path).unwrap_or_else(|err| {
                panic!("failed to read file at path: {} {}", path.display(), err)
            });

            let mut input = peek_nth(input.chars());
            let dom = match parse_children(&mut input) {
                Ok(element) => {
                    let remaining_input: String = input.collect();
                    assert_eq!(
                        remaining_input, "",
                        "{element:?}\nremaining input: {remaining_input}"
                    );
                    element
                }
                Err(error) => {
                    let remaining_input: String = input.collect();
                    panic!("{error}\nremaining input: {remaining_input}");
                }
            };

            let mut graph = StableGraph::new();
            let first = graph.add_node(NodeType::Other);
            let mut last = first;
            let mut current = IntermediateAstElement {
                variable: None,
                escaping_fun: EscapingFunction::NoVariableStart,
                text: String::new(),
            };
            (last, current) =
                children_to_ast(template_name, &mut graph, last, current, dom, "root");
            let previous = last;
            last = graph.add_node(NodeType::Other);
            graph.add_edge(previous, last, current);

            let mut file = File::create(path.with_extension("dot")).unwrap();
            file.write_all(
                format!(
                    "{}",
                    Dot::new(&graph.map(
                        |node_idx, node| format!("{}: {:?}", node_idx.index(), node),
                        |edge_idx, edge| format!("{}: {}", edge_idx.index(), edge)
                    ))
                )
                .as_bytes(),
            )
            .unwrap();
            TemplateCodegen {
                template_name: template_name.to_owned(),
                path,
                graph,
                first,
                last,
            }
        })
        .collect();

    let code = codegen(&inputs);

    let mut item = parse_macro_input!(item as Item);

    InnerMacroReplace(inputs).visit_item_mut(&mut item);

    let expanded = quote! {

        #code

        #[::futures_async_stream::stream(item = alloc::borrow::Cow<'static, str>)]
        #item
    };

    proc_macro::TokenStream::from(expanded)
}
