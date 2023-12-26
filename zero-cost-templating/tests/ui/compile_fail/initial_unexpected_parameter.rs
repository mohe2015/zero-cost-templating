#![feature(async_closure, async_iterator, coroutines, gen_blocks, noop_waker)]

extern crate alloc;

use alloc::borrow::Cow;

use zero_cost_templating::yields;
use zero_cost_templating_macros::template_stream;

#[template_stream("test.html.hbs")]
pub async gen fn test() -> Cow<'static, str> {
    let template = yields!(test0("unexpected"));
}

pub fn main() {}
