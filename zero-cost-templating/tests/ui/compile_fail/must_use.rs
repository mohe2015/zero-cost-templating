#![deny(warnings)]
#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating::yields;
use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub gen fn test() -> Cow<'static, str> {
    let template = yields!(test0());
    let page_title = Cow::from("thetitle");
    let template = yields!(template.page_title0(page_title));
    let csrf_token = Cow::from("thetoken");
    yields!(template.csrf_token1(csrf_token));
}

pub fn main() {}
