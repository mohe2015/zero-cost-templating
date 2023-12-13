
#![feature(prelude_import)]
#![feature(coroutines)]
#![feature(core_panic)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
use alloc::borrow::Cow;
use zero_cost_templating_macros::template_stream;
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
pub struct TestTemplate0;
impl TemplateTypy for TestTemplate0 {}
#[must_use]
pub struct TestTemplate1;
impl TemplateTypy for TestTemplate1 {}
#[must_use]
pub struct TestTemplate2;
impl TemplateTypy for TestTemplate2 {}
#[must_use]
pub struct TestTemplate3;
impl TemplateTypy for TestTemplate3 {}
#[must_use]
pub struct TestTemplate4;
impl TemplateTypy for TestTemplate4 {}
#[must_use]
pub struct TestTemplate5;
impl TemplateTypy for TestTemplate5 {}
#[must_use]
pub struct TestTemplate6;
impl TemplateTypy for TestTemplate6 {}
#[must_use]
pub struct TestTemplate7;
impl TemplateTypy for TestTemplate7 {}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate0, Partial, After> {
    pub fn test_template0(self) -> Template<TestTemplate1, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate1, Partial, After> {
    pub fn test_page_title1(
        self,
        page_title: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Template<TestTemplate2, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate2, Partial, After> {
    pub fn test_csrf_token2(
        self,
        csrf_token: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Template<TestTemplate3, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate3, Partial, After> {
    pub fn test_template3(self) -> Template<TestTemplate4, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate4, Partial, After> {
    pub fn test_title4(
        self,
        title: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Template<TestTemplate5, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate5, Partial, After> {
    pub fn test_text5(
        self,
        text: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Template<TestTemplate3, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate3, Partial, After> {
    pub fn test_template6(self) -> Template<TestTemplate6, Partial, After> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl<Partial: Templaty, After: Templaty> Template<TestTemplate6, Partial, After> {
    pub fn test_copyright_year7(
        self,
        copyright_year: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> After {
        ::core::panicking::panic("not yet implemented")
    }
}
const _test_FORCE_RECOMPILE: &'static str = "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>title</title>\n    <link rel=\"stylesheet\" href=\"style.css\">\n    <script src=\"script.js\"></script>\n  </head>\n  <body>\n    <h1>{{page_title}}</h1>\n    <input type=\"hidden\" value=\"{{csrf_token}}\">\n    <ul>\n    {{#each articles}}\n      <li>{{title}}</li>\n      <li>{{text}}</li>\n    {{/each}}\n    </ul>\n    <span>{{copyright_year}}</span>\n  </body>\n</html>";
pub fn test() -> impl ::futures_async_stream::__private::stream::Stream<
    Item = alloc::borrow::Cow<'static, str>,
> {
    ::futures_async_stream::__private::stream::from_coroutine(static move |
        mut __task_context: ::futures_async_stream::__private::future::ResumeTy,
    | -> () {
        let (): () = {
            let template = if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    Template::<TestTemplate0, _, _> {
                        r#type: TestTemplate0,
                        partial: (),
                        after: (),
                    }
                }
            };
            let template = if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    let _magic_expression_result: Template<TestTemplate0, _, _> = template;
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        ::alloc::borrow::Cow::from(
                            "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>title</title>\n    <link rel=\"stylesheet\" href=\"style.css\">\n    <script src=\"script.js\"></script>\n  </head>\n  <body>\n    <h1>",
                        ),
                    ));
                    Template::<TestTemplate1, _, _> {
                        r#type: TestTemplate1,
                        partial: _magic_expression_result.partial,
                        after: _magic_expression_result.after,
                    }
                }
            };
            let page_title = Cow::from("thetitle");
            let template = if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    let _magic_expression_result: Template<TestTemplate1, _, _> = template;
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        zero_cost_templating::encode_element_text(page_title),
                    ));
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        ::alloc::borrow::Cow::from(
                            "</h1>\n    <input type=\"hidden\" value=\"",
                        ),
                    ));
                    Template::<TestTemplate2, _, _> {
                        r#type: TestTemplate2,
                        partial: _magic_expression_result.partial,
                        after: _magic_expression_result.after,
                    }
                }
            };
            let csrf_token = Cow::from("thetoken");
            let template = if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    let _magic_expression_result: Template<TestTemplate2, _, _> = template;
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        zero_cost_templating::encode_double_quoted_attribute(csrf_token),
                    ));
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        ::alloc::borrow::Cow::from("\">\n    <ul>\n    "),
                    ));
                    Template::<TestTemplate3, _, _> {
                        r#type: TestTemplate3,
                        partial: _magic_expression_result.partial,
                        after: _magic_expression_result.after,
                    }
                }
            };
            let template = if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    let _magic_expression_result: Template<TestTemplate3, _, _> = template;
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        ::alloc::borrow::Cow::from("\n    </ul>\n    <span>"),
                    ));
                    Template::<TestTemplate6, _, _> {
                        r#type: TestTemplate6,
                        partial: _magic_expression_result.partial,
                        after: _magic_expression_result.after,
                    }
                }
            };
            if false {
                ::core::panicking::panic("internal error: entered unreachable code")
            } else {
                {
                    let _magic_expression_result: Template<TestTemplate6, _, _> = template;
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        zero_cost_templating::encode_element_text(
                            Cow::from("2023".to_string()),
                        ),
                    ));
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        ::alloc::borrow::Cow::from("</span>\n  </body>\n</html>"),
                    ));
                    _magic_expression_result.after
                }
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