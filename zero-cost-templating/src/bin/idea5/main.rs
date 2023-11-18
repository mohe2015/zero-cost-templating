#![feature(coroutines)]

extern crate alloc;

use alloc::borrow::Cow;
use std::pin::pin;

use futures::StreamExt;
use futures_async_stream::stream;
use tokio::io::{stdout, AsyncWriteExt};
use zero_cost_templating_macros::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin idea5
// cargo run --release --bin idea5

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin idea5 > type-sizes.txt
// search for
// `{static coroutine@

#[inline(always)]
#[stream(item = Cow<'static, str>)]
pub async fn get_articles_stream() {
    yield Cow::from("hi");
    yield Cow::from("jo");
}

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = initial0!();
    let template = template0!(template);
    let page_title = Cow::from("thetitle");
    let template = page_title1!(template, page_title);
    let csrf_token = Cow::from("thetoken");
    let mut template = csrf_token2!(template, csrf_token);
    let articles = get_articles_stream();
    #[for_await]
    for article in articles {
        let inner_template = template3!(template);
        let inner_template = title4!(inner_template, article);
        template = text5!(inner_template, Cow::from("a"));
    }
    let template = template6!(template);
    copyright_year7!(template, Cow::from("2023".to_string()));
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    println!("cow size {}", std::mem::size_of::<Cow<'static, str>>()); // 24
    let mut stdout = stdout();
    let stream = test();
    println!("size: {}", std::mem::size_of_val(&stream));
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        stdout.write_all(value.as_bytes()).await?;
    }
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}
