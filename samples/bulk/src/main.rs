//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//! 
//! This sample demonstrates a request with a large body.
//! 
//! If you compile with `--features profiling` then it uses the system allocator 
//! to play nicely with valgrind for profiling.
//! 
//! If you compile with `--features lazy_static` then the sample uses a borrowed
//! request instead of an owned one.
//! This will avoid allocating multiple copies of the request if the same body
//! is used multiple times.

#![cfg_attr(feature="profiling", feature(alloc_system))]
#[cfg(feature="profiling")]
extern crate alloc_system;

#[cfg(feature="lazy_static")]
#[cfg_attr(feature = "lazy_static", macro_use)]
extern crate lazy_static;

extern crate elastic_requests;
extern crate elastic_reqwest;

use elastic_reqwest::ElasticClient;
use elastic_requests::BulkRequest;

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

        BulkRequest::new(bulk)
    })
}

#[cfg(feature="lazy_static")]
lazy_static! {
    static ref REQUEST: BulkRequest<'static> = {
        bulk_req!()
    };
}

#[cfg(not(feature="lazy_static"))]
fn get_req() -> BulkRequest<'static> {
    bulk_req!()
}

#[cfg(feature="lazy_static")]
fn get_req() -> &'static BulkRequest<'static> {
    &REQUEST
}

fn main() {
    // Get a new default client.
    let (client, params) = elastic_reqwest::default().unwrap();

    // Send the bulk request.
    let res = client.elastic_req(&params, get_req()).unwrap();

    println!("{:?}", res);
}
