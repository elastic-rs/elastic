#![feature(test)]

extern crate elastic;
extern crate test;

use test::Bencher;

use elastic::client::sender::{NextParams, PreRequestParams};
use elastic::client::sender::static_nodes::StaticNodes;

#[bench]
fn round_robin(b: &mut Bencher) {
    let nodes = StaticNodes::round_robin(vec!["http://hosta:9200", "http://hostb:9200", "http://hostc:9200"], PreRequestParams::default());

    b.iter(|| {
        nodes.next()
    });
}
