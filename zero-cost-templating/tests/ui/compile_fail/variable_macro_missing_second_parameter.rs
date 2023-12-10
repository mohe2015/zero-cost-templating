#![feature(gen_blocks, async_iterator)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
fn test() -> std::borrow::Cow<'static, str> {
    let template = test_initial0!();
    let template = test_template0!(template);
    let page_title = Cow::from("thetitle");
    let template = test_page_title1!(template,);
    let csrf_token = Cow::from("thetoken");
    let mut template = test_csrf_token2!(template, csrf_token);
    let template = test_template6!(template);
    test_copyright_year7!(template, Cow::from("2023".to_string()));
}

pub fn main() {}
