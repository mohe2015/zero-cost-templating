#![deny(warnings)]
#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use std::pin::pin;

use alloc::borrow::Cow;

use zero_cost_templating::{async_iterator_extension::AsyncIterExt, yields};
use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub gen fn test() -> Cow<'static, str> {
    let template = yields!(test0());
    let page_title = Cow::from("thetitle");
    let template = yields!(template.page_title0(page_title));
    let csrf_token = Cow::from("thetoken");
    let template = yields!(template.csrf_token1(csrf_token));
    let template = yields!(template.copyright_year4(Cow::from("2023".to_string())));
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
