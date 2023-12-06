#![feature(coroutines)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = test_initial0!();
    let _ = test_template0!(template);
    let template = test_template0!(template);
    let page_title = Cow::from("thetitle");
    let template = test_page_title1!(template, page_title);
    let csrf_token = Cow::from("thetoken");
    let template = test_csrf_token2!(template, csrf_token);
    let template = test_template6!(template);
    test_copyright_year7!(template, Cow::from("2023".to_string()));
}

pub fn main() {}
