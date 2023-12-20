#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating::yields;
use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(test_initial0());
    //let template = template.template0();
    let page_title = Cow::from("thetitle");
    let template = yields!(template.test_page_title1(page_title));
    let template = yields!(template.test_template2());
    let csrf_token = Cow::from("thetoken");
    let template = yields!(template.test_csrf_token3(csrf_token));
    let template = yields!(template.test_template4());
    let template = yields!(template.test_template10());
    let template = yields!(template.test_copyright_year11(Cow::from("2023".to_string())));
    yields!(template.test_template12());
}

pub fn main() {}
