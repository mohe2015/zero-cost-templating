#![feature(gen_blocks, async_iterator)]

extern crate alloc;

use std::pin::pin;

use tokio::io::{stdout, AsyncWriteExt};
use zero_cost_templating::template_stream;

// https://github.com/dtolnay/cargo-expand

// export RUSTFLAGS="-Z proc-macro-backtrace"
// cargo build
// cargo expand --package zero-cost-templating --bin idea5
// cargo run --release --bin idea5

// RUSTFLAGS="-Zprint-type-sizes" cargo run --release --bin idea5 > type-sizes.txt
// search for
// `{static coroutine@
/*
#[stream(item = Cow<'static, str>)]
pub async fn get_articles_stream() {
    yield Cow::from("ef>eft&t<lef\"efe");
    yield Cow::from("ab>eeehvdft&t<l\"e");
}

#[template_stream("test.html.hbs")]
pub async fn test() {
    let template = initial0!();
    let template = template0!(template);
    let page_title = "the>t&it<l\"e";
    let template = page_title1!(template, page_title);
    let csrf_token = "the>t&ok<e\"n";
    let mut template = csrf_token2!(template, csrf_token);
    let articles = get_articles_stream();
    #[for_await]
    for article in articles {
        let inner_template = template3!(template);
        let inner_template = title4!(inner_template, article);
        template = text5!(inner_template, "twdhfewfe>et&ieft<l\"e");
    }
    let template = template6!(template);
    copyright_year7!(template, "errhj>et&t<l\"e");
}

#[template_stream("test2.html.hbs")]
pub async fn test2() {
    let template = initial0!();
    let template = template0!(template);
    let page_title = "the>t&it<l\"e";
    let template = page_title1!(template, page_title);
    let csrf_token = "the>t&ok<e\"n";
    let mut template = csrf_token2!(template, csrf_token);
    let articles = get_articles_stream();
    #[for_await]
    for article in articles {
        let inner_template = template3!(template);
        let inner_template = title4!(inner_template, article);
        template = text5!(inner_template, "twdhfewfe>et&ieft<l\"e");
    }
    let template = template6!(template);
    copyright_year7!(template, "errhj>et&t<l\"e");
}

#[template_stream("partial_block_partial.html.hbs")]
pub async fn partial_block_partial() {
    todo!()
}
 */
#[template_stream("partial_block.html.hbs", "partial_block_partial.html.hbs")]
fn partial_block() -> std::borrow::Cow<'static, str> {
    // is it important that this possibly stays composable?
    // TODO FIXME make the naming so its easier to know which method to call next
    // currently the .dot file are probably most helpful (the edge numbers should be
    // the method names and the node numbers should be the types?)
    // xdot zero-cost-templating/partial_block.dot
    // xdot zero-cost-templating/partial_block_partial.dot
    let template = partial_block_initial0!();
    let template = template.partial_block_template0();
    let template = template.partial_block_partial_template0();
    let template = template.partial_block_template1();
    let template = partial_block_template2!(template);
    let template = partial_block_partial_template2!(template);
    partial_block_template4!(template);
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    {
        let stream = partial_block();
        let mut stream = pin!(stream);
        while let Some(value) = stream.next().await {
            stdout.write_all(value.as_bytes()).await?;
        }
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }
    Ok(())
}

// Licensed under MIT and Apache-2.0 from https://github.com/rust-lang/rust/blob/master/tests/ui/coroutine/async_gen_fn_iter.rs

use std::pin::Pin;
use std::task::*;
use std::async_iter::AsyncIterator;
use std::future::Future;

trait AsyncIterExt {
    fn next(&mut self) -> Next<'_, Self>;
}

impl<T> AsyncIterExt for T {
    fn next(&mut self) -> Next<'_, Self> {
        Next { s: self }
    }
}

struct Next<'s, S: ?Sized> {
    s: &'s mut S,
}

impl<'s, S: AsyncIterator> Future for Next<'s, S> where S: Unpin {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut *self.s).poll_next(cx)
    }
}

pub fn noop_waker() -> Waker {
    let raw = RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE);

    // SAFETY: the contracts for RawWaker and RawWakerVTable are upheld
    unsafe { Waker::from_raw(raw) }
}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

unsafe fn noop_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

unsafe fn noop(_p: *const ()) {}
