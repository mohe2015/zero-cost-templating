#![feature(prelude_import)]
#![feature(coroutines)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
use zero_cost_templating::template_stream;
#[must_use]
pub struct PartialBlockTemplate0<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockTemplate2<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockTemplate3<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockTemplate4<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockTemplate5<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
const _partial_block_FORCE_RECOMPILE: &'static str = "<span>hello{{#>partial_block_partial}}childrenstart{{test}}childrenend{{/partial_block_partial}}world</span>";
#[must_use]
pub struct PartialBlockPartialTemplate0<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
#[must_use]
pub struct PartialBlockPartialTemplate2<PartialType, EndType> {
    partial_type: PartialType,
    end_type: EndType,
}
const _partial_block_partial_FORCE_RECOMPILE: &'static str = "a{{>@partial-block}}b";
pub fn partial_block() -> impl ::futures_async_stream::__private::stream::Stream<
    Item = alloc::borrow::Cow<'static, str>,
> {
    ::futures_async_stream::__private::stream::from_coroutine(static move |
        mut __task_context: ::futures_async_stream::__private::future::ResumeTy,
    | -> () {
        let (): () = {
            let template = {
                PartialBlockTemplate0::<_, _> {
                    partial_type: (),
                    end_type: (),
                }
            };
            let template = {
                let magic_expression_result: PartialBlockTemplate0<_, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("<span>hello"),
                ));
                PartialBlockPartialTemplate0::<
                    PartialBlockTemplate2<PartialBlockTemplate4<(), ()>, ()>,
                    PartialBlockTemplate4<(), ()>,
                > {
                    partial_type: PartialBlockTemplate2::<
                        PartialBlockTemplate4<(), ()>,
                        (),
                    > {
                        partial_type: PartialBlockTemplate4::<(), ()> {
                            partial_type: (),
                            end_type: (),
                        },
                        end_type: (),
                    },
                    end_type: PartialBlockTemplate4::<(), ()> {
                        partial_type: (),
                        end_type: (),
                    },
                }
            };
            let template = {
                let magic_expression_result: PartialBlockPartialTemplate0<_, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("a"),
                ));
                magic_expression_result.partial_type
            };
            let template = {
                let magic_expression_result: PartialBlockTemplate2<_, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("childrenstart"),
                ));
                PartialBlockTemplate3::<_, _> {
                    partial_type: magic_expression_result.partial_type,
                    end_type: magic_expression_result.end_type,
                }
            };
            let template = {
                let magic_expression_result: PartialBlockTemplate3<_, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("childrenend"),
                ));
                magic_expression_result.end_type
            };
        };
        #[allow(unreachable_code)]
        {
            return;
            loop {
                __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
            }
        }
    })
}
pub fn main() {}