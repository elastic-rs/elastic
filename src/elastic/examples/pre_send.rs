//! Tweak the raw request before sending.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;
extern crate reqwest;
extern crate env_logger;

use std::error::Error;
use std::io::Read;
use elastic::prelude::*;

// TODO: It's not possible to read a `reqwest::Body`
fn hash_request(request: &mut reqwest::Request) -> Result<(), Box<Error + Send>> {
    let mut body = Vec::new();
    let mut request_body = request.body_mut();

    if let &mut Some(ref mut body) = request_body {
        body.read_to_end(&mut body)?;
    }

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
