extern crate elastic_responses;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::fs::File;

fn load_file(p: &str) -> File {
    File::open(p).unwrap()
}

pub mod command;
pub mod ping;
pub mod get;
pub mod search;
pub mod bulk;
pub mod index;
pub mod indices_exists;