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

#[template_stream("only_template.html.hbs", "partial_block_partial.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(only_template0());
    let template = yields!(template.start0("start"));
    let template = yields!(template.before5("before"));
}

pub fn main() {}
