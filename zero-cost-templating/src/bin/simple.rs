#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

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

macro_rules! yields {
    ($e: expr) => {{
        let expr = $e;
        let ret = expr.0;
        let mut iter = std::pin::pin!(expr.1);
        while let Some(v) = ::zero_cost_templating::async_iterator_extension::AsyncIterExt::next(&mut iter).await {
            yield v;
        }
        ret
    }};
}

#[template_stream("if_else.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(if_else_initial0());
    let template = template.if_else_template0();
}

pub fn main() {}
