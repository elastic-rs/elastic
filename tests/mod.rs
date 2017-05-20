extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use std::fs::File;
use std::io::{Read, Cursor};

fn load_file(p: &str) -> File {
    File::open(p).unwrap()
}

pub mod command;
pub mod ping;
pub mod get;
pub mod search;
pub mod bulk;
pub mod index;