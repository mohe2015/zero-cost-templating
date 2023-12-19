#![deny(warnings)]
#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = test_initial0();
    let template = template.test_template0();
    let page_title = Cow::from("thetitle");
    let template = template.test_page_title1(page_title);
    let template = template.test_template2();
    let csrf_token = Cow::from("thetoken");
    let template = template.test_csrf_token3(csrf_token);
    template.test_template4();
}

pub fn main() {}