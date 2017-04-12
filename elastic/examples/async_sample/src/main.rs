extern crate elastic_requests;
extern crate elastic_responses;

extern crate serde_json;

extern crate futures;
extern crate tokio_proto;
extern crate tokio_core;

extern crate memmap;
extern crate hyper;
extern crate futures_cpupool;

#[macro_use]
extern crate quick_error;

use std::str;
use std::io::Read;

use elastic_requests::BulkRequest;
use elastic_responses::BulkErrorsResponse;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future::ok;
use futures_cpupool::CpuPool;

mod body;
mod error;
mod hyper_req;

fn main() {
    let url = "http://localhost:9200";

    // Create an IO core and thread pool
    let mut core = Core::new().unwrap();
    let pool = CpuPool::new(4);

    // Create a `hyper` client expecting `MappedFileBody` bodies
    let client = hyper::client::Config::default()
        .body::<body::MappedFileBody>()
        .build(&core.handle());

    // Get a future to buffer a bulk file
    let (buffer_future, body) = body::mapped_file("./data/accounts.json").unwrap();
    let buffer_future = pool.spawn(buffer_future);

    // Get a future to send a bulk request
    let req = BulkRequest::for_index_ty("bulk-test", "bulk-ty", body);
    let req_future = client.request(hyper_req::build(&url, req))
        .and_then(|res| {
            // Buffer the response and parse as a bulk response
            res.body()
                .fold(Vec::new(), |mut buf, chunk| {
                    chunk.as_ref()
                        .read_to_end(&mut buf)
                        .map(|_| buf)
                })
                .and_then(|buf| {
                    // Do the deserialisation on the CPU pool
                    pool.spawn_fn(move || {
                        let res: BulkErrorsResponse = serde_json::from_slice(&buf).unwrap();
                        println!("{:?}", res);

                        ok(())
                    })
                })
        })
        .map_err(|e| e.into());

    // Join the future to buffer the request body with the future to send the request
    let req_future = buffer_future.join(req_future);

    core.run(req_future).unwrap();
}
