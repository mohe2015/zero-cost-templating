#![deny(warnings)]
#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::pin::pin;

use futures::StreamExt;
use zero_cost_templating_macros::template_stream;

#[template_stream("only_template.html.hbs", "partial_block_partial.html.hbs")]
pub async gen fn test() {
    let template = only_template_initial0();
    let template = template.only_template_template0();
    template.partial_block_partial_template4();
}

#[tokio::main]
pub async fn main() {
    let mut actual = String::new();
    let stream = test();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        actual.push_str(&value);
    }
    assert_eq!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    \
    <title>title</title>\n    <link rel=\"stylesheet\" href=\"style.css\">\n    \
    <script src=\"script.js\"></script>\n  </head>\n  <body>\n    <h1>thetitle</h1>\n    \
    <input type=\"hidden\" value=\"thetoken\">\n    <ul>\n    \n    </ul>\n    \
    <span>2023</span>\n  </body>\n</html>",
        actual
    );
}
