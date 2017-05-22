//! Elasticsearch Reqwest Client Integration sample
//!
//! This sample assumes you have a node running on `localhost`.
//! 
//! This sample demonstrates a bulk request with a large body.

extern crate elastic_reqwest;
extern crate elastic_responses;

use elastic_reqwest::ElasticClient;
use elastic_reqwest::req::BulkRequest;
use elastic_responses::{parse, BulkErrorsResponse};

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
    let res = client.elastic_req(&params, BulkRequest::new(get_req())).unwrap();

    //Parse body to JSON. You could also use `BulkErrorsResponse`.
    let body_as_json: BulkErrorsResponse = parse::<BulkErrorsResponse>().from_reader(res.status().to_u16(), res).unwrap();

    println!("{:?}", body_as_json);
}
