#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::borrow::Cow;

use zero_cost_templating::{template_stream, yields};

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
// cargo run --release --bin simple

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin simple > type-sizes.txt
// search for
// `{static coroutine@

// it is really important to have IDE support so we should probably temporarily
// switch back to the async-stream crate or roll our own.

#[template_stream("if_else.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(if_else0());
    let template = yields!(template.next0());
    yields!(template.next2());
}

pub fn main() {}
