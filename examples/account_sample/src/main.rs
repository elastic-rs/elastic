//! This sample is a more fleshed out application using `elastic`.
//!
//! It expects you have an Elasticsearch node running on `localhost:9200`.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate json_str;
#[macro_use]
extern crate quick_error;

extern crate serde;
extern crate serde_json;
extern crate elastic;

pub mod model;
pub mod commands;

use elastic::client::{Client, RequestParams};
use commands::{EnsureBankIndexExists, PutBulkAccounts};

fn main() {
    let client = Client::new(RequestParams::default()).unwrap();

    println!("checking index");

    client.ensure_bank_index_exists().unwrap();

    println!("updating docs");

    client.put_bulk_accounts("data/accounts.json").unwrap();
}
