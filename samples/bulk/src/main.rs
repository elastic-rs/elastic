//! Elasticsearch Reqwest Client Integration sample
//!
//! This sample assumes you have a node running on `localhost`.
//! 
//! This sample demonstrates a bulk request with a large body.

extern crate elastic_reqwest;
extern crate elastic_responses;

use elastic_reqwest::ElasticClient;
use elastic_reqwest::req::BulkRequest;
use elastic_responses::{HttpResponse, BulkErrorsResponse};

fn get_req() -> String {
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
}

fn main() {

    let (client, params) = elastic_reqwest::default().unwrap();

    // Send the request and read the response.
    let http_res = {
        let res = client.elastic_req(&params, BulkRequest::new(get_req())).unwrap();
        HttpResponse::from_read(res.status().to_u16(), res)
    };

    //Parse body to JSON. You could also use `BulkErrorsResponse`.
    let body_as_json: BulkErrorsResponse = http_res.into_response().unwrap();

    println!("{:?}", body_as_json);
}
