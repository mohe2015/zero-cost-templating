#![deny(warnings)]
#![feature(coroutines)]

extern crate alloc;

use std::pin::pin;

use futures::StreamExt;
use zero_cost_templating_macros::template_stream;

// so to be able to modularly use this we need to generate the struct once for the whole crate

mod if_else_true {
    use zero_cost_templating_macros::template_stream;

    #[template_stream("if_else.html.hbs")]
    pub async fn test_true() {
        let template = if_else_initial0();
        let template = if true {
            let template = template.if_else_template0();
            template.if_else_template2()
        } else {
            let template = template.if_else_template1();
            template.if_else_template3()
        };
        template.if_else_template4();
    }
}

#[template_stream("if_else.html.hbs")]
pub async fn test_false() {
    let template = if_else_initial0();
    let template = if false {
        let template = template.if_else_template0();
        template.if_else_template2()
    } else {
        let template = template.if_else_template1();
        template.if_else_template3()
    };
    template.if_else_template4();
}

#[tokio::test]
async fn if_else_true_output() {
    use crate::if_else_true::test_true;

    let mut actual = String::new();
    let stream = test_true();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        actual.push_str(&value);
    }
    assert_eq!("true", actual);
}

#[tokio::test]
async fn if_else_false_output() {
    let mut actual = String::new();
    let stream = test_false();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        actual.push_str(&value);
    }
    assert_eq!("false", actual);
}
