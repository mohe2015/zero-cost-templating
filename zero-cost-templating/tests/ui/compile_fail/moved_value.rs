#![feature(coroutines)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating_macros::template_stream;

#[template_stream("../../../../zero-cost-templating/test.html.hbs")]
pub async fn test() {
    let template = initial0!();
    let _ = template0!(template);
    let template = template0!(template);
    let page_title = Cow::from("thetitle");
    let template = page_title1!(template, page_title);
    let csrf_token = Cow::from("thetoken");
    let template = csrf_token2!(template, csrf_token);
    let template = template6!(template);
    copyright_year7!(template, Cow::from("2023".to_string()));
}

pub fn main() {}
