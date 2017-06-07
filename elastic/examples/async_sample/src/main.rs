#![feature(conservative_impl_trait)]

extern crate elastic_requests;
extern crate elastic_responses;
extern crate string_cache;

extern crate futures;
extern crate tokio_proto;
extern crate tokio_core;

extern crate memmap;
extern crate hyper;
extern crate futures_cpupool;

#[macro_use]
extern crate quick_error;

mod body;
mod error;
mod hyper_req;

use std::str;

use elastic_requests::BulkRequest;
use elastic_responses::parse;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future::lazy;
use futures_cpupool::{CpuPool, Builder as CpuPoolBuilder};

use hyper::Client;
use hyper::client::HttpConnector;

use error::Error;

type RequestBody = body::FileBody;
type ResponseBody = body::HttpReadBody;

type AllocatedField = string_cache::DefaultAtom;
type BulkResponse = elastic_responses::BulkErrorsResponse<AllocatedField, AllocatedField, String>;

fn main() {
    let url = "http://localhost:9200";

    // Create an IO core and thread pool
    let mut core = Core::new().unwrap();
    let pool = CpuPoolBuilder::new().pool_size(4).name_prefix("pool-thread").create();

    // Create a `hyper` client
    let client = hyper::client::Config::default()
        .body::<RequestBody>()
        .build(&core.handle());
    
    let request = send_request(url, &client, pool);
    core.run(request).unwrap();
}

fn send_request(url: &'static str, client: &Client<HttpConnector, RequestBody>, pool: CpuPool) -> impl Future<Item = BulkResponse, Error = Error> {
    // Get a future to buffer a bulk file
    let (buffer_request_body, body) = body::mapped_file("./data/accounts.json").unwrap();
    let buffer_request_body = pool.spawn(buffer_request_body);

    // Build a Bulk request
    let req = BulkRequest::for_index_ty("bulk-async", "bulk-ty", body);

    // Send the request
    let send_request = client.request(hyper_req::build(&url, req)).map_err(Into::into);

    // Read and desrialise the response
    let read_response = send_request
        .and_then(buffer_response_body)
        .and_then(move |res| pool.spawn(deserialise_response(res)));

    // Join the future to buffer the request body with the future to send the request
    buffer_request_body.join(read_response).and_then(move |(_, res)| Ok(res))
}

// Read the response body into a queue of chunks
fn buffer_response_body(res: hyper::client::Response) -> impl Future<Item = (u16, ResponseBody), Error = Error> {
    let status: u16 = (*res.status()).into();

    // Buffer the response chunks into a synchronously readable sequence
    let buffer_body = res.body()
        .fold::<_, _, Result<_, hyper::Error>>(body::HttpReadBodyBuilder::new(), |mut buf, chunk| {
            buf.push(chunk);
            Ok(buf)
        });

    buffer_body.and_then(move |buf| {
        Ok((status, buf.build()))
    })
    .map_err(Into::into)
}

// Deserialise the queue of chunks as a BulkResponse
fn deserialise_response((status, mut buf): (u16, ResponseBody)) -> impl Future<Item = BulkResponse, Error = Error> {
    lazy(move || parse::<BulkResponse>().from_reader(status, &mut buf)).map_err(Into::into)
}
