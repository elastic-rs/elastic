//! This sample is a more fleshed out application using `elastic`.
//!
//! It expects you have an Elasticsearch node running on `localhost:9200`.

#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;

extern crate elastic;
#[macro_use]
extern crate serde_json;

pub mod model;
pub mod ops;

use ops::{
    commands::{
        EnsureBankIndexExists,
        PutBulkAccounts,
    },
    queries::SimpleSearchQuery,
    Client,
};
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let client = Client::new("http://localhost:9200")?;

    println!("checking index");

    client.ensure_bank_index_exists()?;

    println!("updating docs");

    client.put_bulk_accounts("data/accounts.json")?;

    let accounts = client.simple_search_query("Bruce Coffey")?;

    for account in accounts.hits() {
        println!("{:?}", account);
    }

    Ok(())
}

fn main() {
    run().unwrap()
}
