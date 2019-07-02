//! Tweak the raw http request before sending.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;
extern crate env_logger;
extern crate reqwest;

use elastic::{
    http::{
        header::{
            HeaderName,
            HeaderValue,
        },
        SyncHttpRequest,
    },
    prelude::*,
};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{
        Hash,
        Hasher,
    },
    io::Read,
    str::FromStr,
};

fn hash_request(request: &mut SyncHttpRequest) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Read the body into a temporary buffer
    let mut buffered = Vec::new();
    if let Some(ref mut body) = request.body_mut() {
        body.reader().read_to_end(&mut buffered)?;
    }

    // Access the request data
    let mut hasher = DefaultHasher::new();

    request.url_mut().hash(&mut hasher);
    request.method_mut().hash(&mut hasher);
    buffered.hash(&mut hasher);

    for (key, value) in request.headers_mut().iter() {
        format!("{}: {}", key, value.to_str()?).hash(&mut hasher);
    }

    // Add a raw header to the request
    let hash = hasher.finish();
    request
        .headers_mut()
        .insert(HeaderName::from_str("X-BadHash")?, HeaderValue::from(hash));

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // A HTTP client and request parameters
    let client = SyncClient::builder().pre_send_raw(hash_request).build()?;

    // Ping the cluster
    let ping = client.ping().send()?;

    println!("{:?}", ping);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
