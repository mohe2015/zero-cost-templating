#![feature(gen_blocks, async_iterator)]

extern crate alloc;

use std::pin::pin;

use futures::executor::block_on;
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};
use zero_cost_templating_macros::template_stream;

#[template_stream("partial_block.html.hbs", "partial_block_partial.html.hbs")]
fn partial_block() -> std::borrow::Cow<'static, str> {
    // is it important that this possibly stays composable?
    // TODO FIXME make the naming so its easier to know which method to call next
    // currently the .dot file are probably most helpful (the edge numbers should be
    // the method names and the node numbers should be the types?)
    // xdot zero-cost-templating/partial_block.dot
    // xdot zero-cost-templating/partial_block_partial.dot

    // TODO FIXME the test variable is not required
    let template = partial_block_initial0!();
    let template = partial_block_template0!(template);
    let template = partial_block_partial_template0!(template);
    let template = partial_block_template1!(template);
    let template = partial_block_template2!(template);
    let template = partial_block_partial_template2!(template);
    partial_block_template4!(template);
}

async fn build_template() -> String {
    let mut output = String::new();
    let stream = partial_block();
    let mut stream = pin!(stream);
    while let Some(value) = stream.next().await {
        output += &value;
    }
    output
}

#[library_benchmark]
#[bench::short()]
fn bench_template() -> String {
    black_box(block_on(build_template()))
}

library_benchmark_group!(
    name = bench_template_group;
    benchmarks = bench_template
);

main!(library_benchmark_groups = bench_template_group);


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
