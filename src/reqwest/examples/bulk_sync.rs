//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This sample demonstrates a request with a large body.

extern crate elastic_reqwest;

use elastic_reqwest::{Error, SyncElasticClient, SyncFromResponse};
use elastic_reqwest::req::BulkRequest;
use elastic_reqwest::res::{parse, BulkResponse};

// Create a bulk request to index a bunch of docs.
fn get_req() -> String {
    let mut bulk = String::new();
    for i in 1..1000 {
        let header = format!(
            "{{ \"index\" : {{ \"_index\" : \"test\", \"_type\" : \"ty\", \"_id\" : \"{}\" }} }}",
            i
        );
        let body = format!("{{ \"title\" : \"string value {}\" }}", i);

        bulk.push_str(&header);
        bulk.push('\n');
        bulk.push_str(&body);
        bulk.push('\n');
    }

    bulk
}

fn run() -> Result<(), Error> {
    // Get a new default client.
    let (client, params) = elastic_reqwest::sync::default()?;

    // Send the bulk request.
    let http_res = client.elastic_req(&params, BulkRequest::new(get_req()))?;

    let res = parse::<BulkResponse>().from_response(http_res)?;

    println!("{:?}", res);

    Ok(())
}

fn main() {
    run().unwrap();
}
