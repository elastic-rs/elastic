//! Tweak the raw http request before sending.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;
extern crate reqwest;
extern crate env_logger;

use std::error::Error;
use std::io::Read;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use elastic::prelude::*;
use elastic::http::SyncHttpRequest;

fn hash_request(request: &mut SyncHttpRequest) -> Result<(), Box<Error + Send + Sync>> {
    let &mut SyncHttpRequest { ref mut url, ref mut method, ref mut body, ref mut headers, .. } = request;

    // Read the body into a temporary buffer
    let mut buffered = Vec::new();
    if let &mut Some(ref mut body) = body {
        body.reader().read_to_end(&mut buffered)?;
    }

    // Access the request data
    let mut hasher = DefaultHasher::new();

    url.hash(&mut hasher);
    method.hash(&mut hasher);
    buffered.hash(&mut hasher);
    
    for header in headers.iter() {
        header.to_string().hash(&mut hasher);
    }
    
    // Add a raw header to the request
    let hash = hasher.finish();
    headers.set_raw("X-BadHash", hash.to_string());

    Ok(())
}

fn run() -> Result<(), Box<Error>> {
    // A HTTP client and request parameters
    let client = SyncClientBuilder::new()
        .pre_send_raw(hash_request)
        .build()?;

    // Ping the cluster
    let ping = client.ping().send()?;

    println!("{:?}", ping);

    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap()
}
