#![feature(test, plugin)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_macros;
extern crate rs_es;
extern crate hyper;

extern crate test;

/*

Working on a few ideas for the client implementation. 

This is using rs-es as a benchmark.

There's absolutely nothing scientific going on here, and only synchronous requests are being benchmarked,
even though an asynchronous API is a primary goal. Watch this space :)

*/

pub mod hyper;
pub mod rotor;
pub mod rses;