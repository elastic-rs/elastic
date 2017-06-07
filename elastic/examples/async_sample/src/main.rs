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
use futures_cpupool::{CpuPool, Builder as CpuPoolBuilder};

use hyper::Client;
use hyper::client::HttpConnector;

use error::Error;

type RequestBody = body::request::FileBody;

type AllocatedField = string_cache::DefaultAtom;
type BulkResponse = elastic_responses::BulkErrorsResponse<AllocatedField, AllocatedField, String>;

fn main() {
    let url = "http://localhost:9200";

    // Create an IO core and thread pool
    let mut core = Core::new().unwrap();
    let pool = CpuPoolBuilder::new()
        .pool_size(4)
        .name_prefix("pool-thread")
        .create();

    // Create a `hyper` client
    let client = hyper::client::Config::default()
        .body::<RequestBody>()
        .build(&core.handle());

    // Send a request and get a future for the response
    let response_future = send_request(url, &client, pool);

    core.run(response_future).unwrap();
}

fn send_request(url: &'static str,
                client: &Client<HttpConnector, RequestBody>,
                pool: CpuPool)
                -> impl Future<Item = BulkResponse, Error = Error> {
    // Get a future to buffer a bulk file
    let (buffer_request_body, request_body) = body::request::mapped_file("./data/accounts.json").unwrap();
    let buffer_request_body = pool.spawn(buffer_request_body);

    // Build a Bulk request
    let request = BulkRequest::for_index_ty("bulk-async", "bulk-ty", request_body);

    // Send the request
    let send_request = client
        .request(hyper_req::build(&url, request))
        .map_err(Into::into);

    // Buffer the response chunks into a synchronously readable sequence
    let read_response = send_request.and_then(move |response| {
        let status: u16 = response.status().into();
        let chunks = body::response::ChunkBodyBuilder::new();

        response.body()
            .fold(chunks, concat_chunks)
            .map(move |chunks| (status, chunks.build()))
            .map_err(Into::into)
    });

    // Deserialise the response body into a concrete type
    let deserialise_response = read_response.and_then(move |(status, mut response_body)| {
        pool.spawn_fn(move || {
            parse::<BulkResponse>()
                .from_reader(status, &mut response_body)
                .map_err(Into::into)
        })
    });

    // Join the future to buffer the request body with the future to send the request
    buffer_request_body
        .join(deserialise_response)
        .map(move |(_, response)| response)
}

fn concat_chunks(mut chunks: body::response::ChunkBodyBuilder,
                 chunk: hyper::Chunk)
                 -> Result<body::response::ChunkBodyBuilder, hyper::Error> {
    chunks.append(chunk);
    Ok(chunks)
}
