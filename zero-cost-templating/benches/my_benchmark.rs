#![feature(coroutines)]

extern crate alloc;

use std::pin::pin;

use futures::{executor::block_on_stream, StreamExt};
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};
use zero_cost_templating_macros::template_stream;

#[template_stream("partial_block.html.hbs", "partial_block_partial.html.hbs")]
pub async fn partial_block() {
    // is it important that this possibly stays composable?
    // TODO FIXME make the naming so its easier to know which method to call next
    // currently the .dot file are probably most helpful (the edge numbers should be
    // the method names and the node numbers should be the types?)
    // xdot zero-cost-templating/partial_block.dot
    // xdot zero-cost-templating/partial_block_partial.dot
    let template = partial_block_initial0!();
    let template = partial_block_template0!(template);
    let template = partial_block_partial_template0!(template);
    let template = partial_block_template1!(template);
    let template = partial_block_template2!(template);
    let template = partial_block_partial_template2!(template);
    partial_block_template4!(template);
}

fn build_template() -> String {
    let mut output = String::new();
    let stream = partial_block();
    let stream = pin!(stream);
    let stream = block_on_stream(stream);
    for value in stream {
        output.push_str(&value);
    }
    output
}

#[library_benchmark]
#[bench::short()]
fn bench_template() -> String {
    black_box(build_template())
}

library_benchmark_group!(
    name = bench_template_group;
    benchmarks = bench_template
);

main!(library_benchmark_groups = bench_template_group);
