#![deny(warnings)]
#![feature(coroutines)]

extern crate alloc;

use std::pin::pin;

use alloc::borrow::Cow;

use futures::StreamExt;
use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = test_initial0();
    let template = template.test_template0();
    let page_title = Cow::from("thetitle");
    let template = template.test_page_title1(page_title);
    let csrf_token = Cow::from("thetoken");
    let template = template.test_csrf_token2(csrf_token);
    let template = template.test_template6();
    template.test_copyright_year7(Cow::from("2023".to_string()));
}

#[tokio::main]
pub async fn main() {
    if option_env!("ZERO_COST_TEMPLATING_NO_EXPAND").is_some() {
        return;
    }
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
