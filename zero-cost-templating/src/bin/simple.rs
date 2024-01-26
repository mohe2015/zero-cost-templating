#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::borrow::Cow;

use zero_cost_templating::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
/*

echo '#![feature(print_internals)] #![feature(unsafe_pin_internals)]' > zero-cost-templating/src/bin/test.rs \
&& cargo expand --package zero-cost-templating --bin simple >> zero-cost-templating/src/bin/test.rs \
&& RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin test > type-sizes.txt

search for
`{gen fn body@
`{gen block@
`{async gen fn body@
`{async gen block@
`{static coroutine@
*/

#[template_stream("templates")]
pub async fn test() -> Cow<'static, str> {
    let template = g_partial_block().await;
    let template = template.next().await;
    let template = template.next().await;
    let template = template.before(b"before").await;
    let template = template.next().await;
    let template = template.next().await;
    let template = template.test(b"test").await;
    let template = template.next().await;
    let template = template.next().await;
    let template = template.next().await;
    let template = template.test(b"test").await;
    let template = template.next().await;
    let template = template.next().await;
    let template = template.after(b"after").await;
    let template = template.next().await;
    template.next().await;
}

#[tokio::main]
pub async fn main() {
    let async_iterator = test();
    println!("size of &str: {}", std::mem::size_of::<&str>());
    println!("size of Cow: {}", std::mem::size_of::<Cow<'static, str>>());
    println!("size of String: {}", std::mem::size_of::<String>());
    println!(
        "size of iterator: {}",
        std::mem::size_of_val(&async_iterator)
    );
    let mut output = String::new();
    for value in async_iterator {
        output.push_str(&value);
    }
    print!("{}", output);
}
