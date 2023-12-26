#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::borrow::Cow;

use zero_cost_templating::{template_stream, yields};

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
// cargo expand --package zero-cost-templating --bin simple > zero-cost-templating/src/bin/test.rs
// cargo run --release --bin simple

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin simple > type-sizes.txt
// search for
// `{static coroutine@

#[template_stream("templates")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(g_partial_block());
    let template = yields!(template.next());
    let template = yields!(template.before("test"));
    let template = yields!(template.test("test"));
}

pub fn main() {}
