extern crate alloc;

use futures_util::StreamExt;
use std::{borrow::Cow, io::Write, pin::pin};
use zero_cost_templating::{template_stream, TheStream};

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin simple
/*

echo '#![feature(print_internals)] #![feature(unsafe_pin_internals)]' > zero-cost-templating/src/bin/test.rs \
&& cargo expand --package zero-cost-templating --bin simple >> zero-cost-templating/src/bin/test.rs \
&& RUSTFLAGS="-Zprint-type-sizes" cargo +nightly run --release --bin test > type-sizes.txt

search for
`{gen fn body@
`{gen block@
`{async gen fn body@
`{async gen block@
`{static coroutine@
*/

// maybe add long hand type aliases for templates
pub async fn inner<After>(template: Tp<Tpg_partial_block2, (), After>) -> After {
    template.test("inner").await
}

#[template_stream("templates")]
pub async fn test(stream: ::zero_cost_templating::FutureToStream) {
    let template = g_partial_block(stream);
    let template = template.next().await;
    let template = template.before("before").await;
    let template = inner(template).await;
    let template = template.next().await;
    let template = inner(template).await;
    let template = template.after("after").await;
    template.next().await;
}

#[tokio::main]
pub async fn main() {
    let stream = TheStream::new(test);
    println!("size of &str: {}", std::mem::size_of::<&str>());
    println!("size of Cow: {}", std::mem::size_of::<Cow<'static, str>>());
    println!("size of String: {}", std::mem::size_of::<String>());
    println!("size of stream: {}", std::mem::size_of_val(&stream));
    let mut stream = pin!(stream);
    let mut stdout = std::io::stdout().lock();
    while let Some(element) = stream.next().await {
        stdout.write_all(&element).unwrap();
    }
    stdout.write_all(b"\n").unwrap();
}
