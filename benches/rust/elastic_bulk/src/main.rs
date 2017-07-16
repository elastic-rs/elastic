//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//! 
//! This sample demonstrates a request with a large body.
//! 
//! If you compile with `--features profile_memory` then it uses the system allocator 
//! to play nicely with valgrind for profile_memory.
//! 
//! If you compile with `--features lazy_static` then the sample uses a borrowed
//! request instead of an owned one.
//! This will avoid allocating multiple copies of the request if the same body
//! is used multiple times.

#![cfg_attr(feature="profile_memory", feature(alloc_system))]
#[cfg(feature="profile_memory")]
extern crate alloc_system;

#[macro_use]
extern crate lazy_static;
#[cfg(feature="string_cache")]
extern crate string_cache;
#[cfg(feature="inlinable_string")]
extern crate inlinable_string;
extern crate elastic;

extern crate measure;

use elastic::http;
use elastic::prelude::*;

#[cfg(not(any(feature="string_cache", feature="inlinable_string")))]
type BulkResponseType = elastic::prelude::BulkResponse<String, String, String>;

#[cfg(feature="string_cache")]
type BulkResponseType = elastic::prelude::BulkResponse<string_cache::DefaultAtom, string_cache::DefaultAtom, string_cache::DefaultAtom>;

#[cfg(feature="inlinable_string")]
type BulkResponseType = elastic::prelude::BulkResponse<inlinable_string::InlinableString, inlinable_string::InlinableString, inlinable_string::InlinableString>;

// Create a bulk request to index a bunch of docs.
macro_rules! bulk_req {
    () => ({
        let mut bulk = String::new();
        for i in 1..1000 {
            let header = format!("{{ \"index\" : {{ \"_index\" : \"test\", \"_type\" : \"ty\", \"_id\" : \"{}\" }} }}", i);
            let body = format!("{{ \"title\" : \"string value {}\" }}", i);

            bulk.push_str(&header);
            bulk.push('\n');
            bulk.push_str(&body);
            bulk.push('\n');
        }

        bulk
    })
}

lazy_static! {
    static ref REQUEST: String = bulk_req!();
}

fn get_req() -> &'static str {
    &REQUEST
}

fn main() {
    let runs = measure::parse_runs_from_env();

    let client = ClientBuilder::new()
        .params(|p| p.header(http::header::Connection::keep_alive()))
        .build()
        .unwrap();
    
    let results = measure::run(runs, || {
        client.request(BulkRequest::new(get_req()))
              .send()
              .and_then(into_response::<BulkResponseType>)
              .unwrap()
    });

    println!("{}", results);
}