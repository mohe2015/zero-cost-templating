#![deny(warnings)]
#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::{borrow::Cow, pin::pin};

use zero_cost_templating::{async_iterator_extension::AsyncIterExt, yields};
use zero_cost_templating_macros::template_stream;

#[template_stream("only_template.html.hbs", "partial_block_partial.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(only_template_initial0());
    let template = yields!(template.only_template_template0());
    let template = yields!(template.partial_block_partial_template4());
    let template = yields!(template.up());
}

#[tokio::main]
pub async fn main() {
    let mut actual = String::new();
    let stream = test();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        actual.push_str(&value);
    }
    assert_eq!("<p></p>", actual);
}
