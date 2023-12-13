
#![feature(prelude_import)]
#![feature(coroutines)]
#![feature(core_panic)]
#![feature(unsafe_pin_internals)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
use std::pin::pin;
use futures::StreamExt;
use tokio::io::{stdout, AsyncWriteExt};
use zero_cost_templating::template_stream;
pub trait Templaty {}
pub trait TemplateTypy {}
#[must_use]
pub struct Template<Type: TemplateTypy, Partial: Templaty, After: Templaty> {
    r#type: Type,
    partial: Partial,
    after: After,
}
impl Templaty for () {}
impl<Type: TemplateTypy, Partial: Templaty, After: Templaty> Templaty
for Template<Type, Partial, After> {}
#[must_use]
pub struct PartialBlockTemplate0;
impl TemplateTypy for PartialBlockTemplate0 {}
#[must_use]
pub struct PartialBlockTemplate2;
impl TemplateTypy for PartialBlockTemplate2 {}
#[must_use]
pub struct PartialBlockTemplate3;
impl TemplateTypy for PartialBlockTemplate3 {}
#[must_use]
pub struct PartialBlockTemplate4;
impl TemplateTypy for PartialBlockTemplate4 {}
#[must_use]
pub struct PartialBlockTemplate5;
impl TemplateTypy for PartialBlockTemplate5 {}
#[must_use]
pub struct PartialBlockTemplate6;
impl TemplateTypy for PartialBlockTemplate6 {}
impl<
    Partial: Templaty,
    After: Templaty,
> Template<PartialBlockTemplate0, Partial, After> {
    pub fn partial_block_template0(
        self,
    ) -> Template<
        PartialBlockPartialTemplate0,
        Template<PartialBlockTemplate2, (), Template<PartialBlockTemplate5, (), ()>>,
        Template<PartialBlockTemplate5, (), ()>,
    > {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<
    Partial: Templaty,
    After: Templaty,
> Template<PartialBlockTemplate2, Partial, After> {
    pub fn partial_block_template1(
        self,
    ) -> Template<PartialBlockTemplate3, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<
    Partial: Templaty,
    After: Templaty,
> Template<PartialBlockTemplate3, Partial, After> {
    pub fn partial_block_test2(
        self,
        test: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> After {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<
    Partial: Templaty,
    After: Templaty,
> Template<PartialBlockTemplate5, Partial, After> {
    pub fn partial_block_template4(self) -> After {
        ::core::panicking::panic("not yet implemented")
    }
}
const _partial_block_FORCE_RECOMPILE: &'static str = "<span>hello{{#>partial_block_partial}}childrenstart{{test}}childrenend{{/partial_block_partial}}world</span>";
#[must_use]
pub struct PartialBlockPartialTemplate0;
impl TemplateTypy for PartialBlockPartialTemplate0 {}
#[must_use]
pub struct PartialBlockPartialTemplate2;
impl TemplateTypy for PartialBlockPartialTemplate2 {}
#[must_use]
pub struct PartialBlockPartialTemplate3;
impl TemplateTypy for PartialBlockPartialTemplate3 {}
impl<
    Partial: TemplateTypy,
    PartialPartial: Templaty,
    PartialAfter: Templaty,
    After: Templaty,
> Template<
    PartialBlockPartialTemplate0,
    Template<Partial, PartialPartial, PartialAfter>,
    After,
> {
    pub fn partial_block_partial_template0(
        self,
    ) -> Template<Partial, (), Template<PartialBlockPartialTemplate2, (), After>> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<
    Partial: Templaty,
    After: Templaty,
> Template<PartialBlockPartialTemplate2, Partial, After> {
    pub fn partial_block_partial_template2(self) -> After {
        ::core::panicking::panic("not yet implemented")
    }
}
const _partial_block_partial_FORCE_RECOMPILE: &'static str = "<p>{{>@partial-block}}</p>";
pub fn partial_block() -> impl ::futures_async_stream::__private::stream::Stream<
    Item = alloc::borrow::Cow<'static, str>,
> {
    ::futures_async_stream::__private::stream::from_coroutine(static move |
        mut __task_context: ::futures_async_stream::__private::future::ResumeTy,
    | -> () {
        let (): () = {
            let template = {
                Template::<PartialBlockTemplate0, _, _> {
                    r#type: PartialBlockTemplate0,
                    partial: (),
                    after: (),
                }
            };
            let template = {
                let _magic_expression_result: Template<PartialBlockTemplate0, _, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("<span>hello"),
                ));
                Template::<
                    PartialBlockPartialTemplate0,
                    Template<
                        PartialBlockTemplate2,
                        (),
                        Template<PartialBlockTemplate5, (), ()>,
                    >,
                    Template<PartialBlockTemplate5, (), ()>,
                > {
                    r#type: PartialBlockPartialTemplate0,
                    partial: Template::<
                        PartialBlockTemplate2,
                        (),
                        Template<PartialBlockTemplate5, (), ()>,
                    > {
                        r#type: PartialBlockTemplate2,
                        partial: (),
                        after: Template::<PartialBlockTemplate5, (), ()> {
                            r#type: PartialBlockTemplate5,
                            partial: (),
                            after: (),
                        },
                    },
                    after: Template::<PartialBlockTemplate5, (), ()> {
                        r#type: PartialBlockTemplate5,
                        partial: (),
                        after: (),
                    },
                }
            };
            let template = {
                let _magic_expression_result: Template<
                    PartialBlockPartialTemplate0,
                    _,
                    _,
                > = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("<p>"),
                ));
                Template::<_, (), Template<PartialBlockPartialTemplate2, (), _>> {
                    r#type: _magic_expression_result.partial.r#type,
                    partial: (),
                    after: Template {
                        r#type: PartialBlockPartialTemplate2,
                        partial: (),
                        after: _magic_expression_result.after,
                    },
                }
            };
            let template = {
                let _magic_expression_result: Template<PartialBlockTemplate2, _, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("childrenstart"),
                ));
                Template::<PartialBlockTemplate3, _, _> {
                    r#type: PartialBlockTemplate3,
                    partial: _magic_expression_result.partial,
                    after: _magic_expression_result.after,
                }
            };
            let template = {
                let _magic_expression_result: Template<PartialBlockTemplate3, _, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    zero_cost_templating::encode_element_text("Hi"),
                ));
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("childrenend"),
                ));
                _magic_expression_result.after
            };
            let template = {
                let _magic_expression_result: Template<
                    PartialBlockPartialTemplate2,
                    _,
                    _,
                > = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("</p>"),
                ));
                _magic_expression_result.after
            };
            {
                let _magic_expression_result: Template<PartialBlockTemplate5, _, _> = template;
                __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                    ::alloc::borrow::Cow::from("world</span>"),
                ));
                _magic_expression_result.after
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
pub fn main() -> Result<(), std::io::Error> {
    let body = async {
        let mut stdout = stdout();
        {
            let stream = partial_block();
            let mut stream = ::core::pin::Pin::<&mut _> {
                pointer: &mut { stream },
            };
            while let Some(value) = stream.next().await {
                stdout.write_all(value.as_bytes()).await?;
            }
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}