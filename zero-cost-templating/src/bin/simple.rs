#![feature(coroutines)]

extern crate alloc;

use zero_cost_templating::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
// cargo run --release --bin simple

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin simple > type-sizes.txt
// search for
// `{static coroutine@

#[template_stream(
    "test.html.hbs",
    "partial_block.html.hbs",
    "partial_block_partial.html.hbs"
)]
pub async fn test() {}

pub fn main() {}
