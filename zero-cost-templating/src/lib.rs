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

extern crate alloc;

use alloc::borrow::Cow;
use std::sync::OnceLock;

pub use futures::stream::iter;
pub use futures::Stream;
use regex::Captures;
//pub use html_escape::{encode_double_quoted_attribute, encode_safe};

pub fn encode_safe<'a, I: Into<Cow<'a, str>>>(input: I) -> Cow<'a, str> {
    static REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| regex::Regex::new("[&<>\"]").unwrap());

    let input = input.into();
    match regex.replace_all(&input, |captures: &Captures| {
        match captures.get(0).unwrap().as_str() {
            "&" => "&amp;",
            "<" => "&lt;",
            ">" => "&gt;",
            "\"" => "&quot;",
            _ => unreachable!(),
        }
    }) {
        Cow::Borrowed(_) => input,
        Cow::Owned(owned) => Cow::Owned(owned),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let test_cases = trybuild::TestCases::new();
        test_cases.compile_fail("tests/ui/compile_fail/*.rs");
        test_cases.pass("tests/ui/pass/*.rs");
    }
}
