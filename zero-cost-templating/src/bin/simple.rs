#![feature(coroutines)]

extern crate alloc;

use std::borrow::Cow;

use zero_cost_templating::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
// cargo run --release --bin simple

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin simple > type-sizes.txt
// search for
// `{static coroutine@

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = test_initial0();
    let template = template.test_template0();
    let page_title = Cow::from("thetitle");
    let template = template.test_page_title1(page_title);
    let csrf_token = Cow::from("thetoken");
    let template = template.test_csrf_token2(csrf_token);
    let template = template.test_template6();
    template.test_copyright_year7(Cow::from("2023".to_string()));
}

pub fn main() {}
