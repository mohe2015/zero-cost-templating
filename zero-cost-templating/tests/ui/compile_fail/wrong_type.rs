#![deny(warnings)]
#![feature(coroutines)]

extern crate alloc;

use alloc::borrow::Cow;

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
    let test: String = { template.test_copyright_year7(Cow::from("2023".to_string())) };
}

pub fn main() {}
