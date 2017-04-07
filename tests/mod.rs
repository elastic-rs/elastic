extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use std::fs::File;
use std::io::{Read, Cursor};

fn load_file_as_response(status: u16, p: &str) -> HttpResponse<Cursor<Vec<u8>>> {
    let mut f = File::open(p).unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();
    
    HttpResponse::new(status, Cursor::new(s))
}

#[test]
fn test_read_response() {
    let mut res = HttpResponse::new(200, Cursor::new(vec![1, 2, 3]));

    let mut actual = Vec::new();
    res.read_to_end(&mut actual).unwrap();

    let expected = vec![1, 2, 3];

    assert_eq!(expected, actual);
}

pub mod ping;
pub mod get;
pub mod search;
