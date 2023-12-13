#![feature(coroutines)]

extern crate alloc;

use std::pin::pin;

use futures::StreamExt;
use tokio::io::{stdout, AsyncWriteExt};
use zero_cost_templating::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin idea5
// cargo run --release --bin idea5

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin idea5 > type-sizes.txt
// search for
// `{static coroutine@

#[template_stream("partial_block.html.hbs", "partial_block_partial.html.hbs")]
pub async fn partial_block() {
    // is it important that this possibly stays composable?
    let template = partial_block_initial0();
    let template = template.partial_block_template0();
    let template = template.partial_block_partial_template0();
    let template = template.partial_block_template1();
    let template = template.partial_block_test2("Hi");
    let template = template.partial_block_partial_template2();
    template.partial_block_template4();
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    let stream = partial_block();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        stdout.write_all(value.as_bytes()).await?;
    }
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}
