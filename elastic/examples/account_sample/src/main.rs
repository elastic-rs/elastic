//! This sample is a more fleshed out application using `elastic`.
//!
//! It expects you have an Elasticsearch node running on `localhost:9200`.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate json_str;
#[macro_use]
extern crate quick_error;

extern crate serde;
extern crate serde_json;
extern crate elastic;

pub mod model;
pub mod ops;

use elastic::client::RequestParams;
use ops::Client;
use ops::commands::{EnsureBankIndexExists, PutBulkAccounts};
use ops::queries::SimpleSearchQuery;

fn main() {
    let client = Client::new(RequestParams::default()).unwrap();

    println!("checking index");

    client.ensure_bank_index_exists().unwrap();

    println!("updating docs");

    client.put_bulk_accounts("data/accounts.json").unwrap();

    let accounts = client.simple_search_query("Bruce Coffey").unwrap();

    for account in accounts.hits() {
    	println!("{:?}", account);
    }
}
