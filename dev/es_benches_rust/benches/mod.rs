#![feature(test, plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types_codegen;
extern crate rs_es;
extern crate hyper;

extern crate test;

/*

Working on a few ideas for the client implementation. 

This is using rs-es as a benchmark.

*/

pub mod hyper;
pub mod rses;