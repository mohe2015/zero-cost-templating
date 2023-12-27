#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::{borrow::Cow, pin::pin};

use zero_cost_templating::{async_iterator_extension::AsyncIterExt, template_stream, yields};

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
// echo "#![feature(print_internals)] #![feature(unsafe_pin_internals)] " > zero-cost-templating/src/bin/test.rs
// cargo expand --package zero-cost-templating --bin simple >> zero-cost-templating/src/bin/test.rs
// cargo run --release --bin simple

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin simple > type-sizes.txt
// search for
// `{async gen block@
// `{async gen fn body@
// `{static coroutine@

// Don't use Cow because it is so big?
#[template_stream("templates")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(g_partial_block());
    let template = yields!(template.next());
    let template = yields!(template.before("before"));
    let template = yields!(template.test("test"));
    let template = yields!(template.next());
    let template = yields!(template.test("test"));
    let template = yields!(template.after("after"));
    yields!(template.next());
}

#[tokio::main]
pub async fn main() {
    let mut async_iterator = test();
    println!("size: {}", std::mem::size_of_val(&async_iterator)); // 264
    let async_iterator = pin!(async_iterator);
    let mut async_iterator = async_iterator.next();
    let mut output = String::new();
    while let Some(value) = (&mut async_iterator).await {
        output.push_str(&value);
    }
    print!("{}", output);
}
