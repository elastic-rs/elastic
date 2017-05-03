extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use std::fs::File;
use std::io::{Read, Cursor};

fn load_file_as_response(status: u16, p: &str) -> HttpResponseRead<Cursor<Vec<u8>>> {
    let mut f = File::open(p).unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();
    
    HttpResponse::from_read(status, Cursor::new(s))
}

#[test]
fn test_read_response() {
    let mut res = HttpResponse::from_read(200, Cursor::new(vec![1, 2, 3]));

    let mut actual = Vec::new();
    res.read_to_end(&mut actual).unwrap();

    let expected = vec![1, 2, 3];

    assert_eq!(expected, actual);
}

#[test]
fn test_as_ref_response() {
    let res = HttpResponse::from_slice(200, vec![1, 2, 3]);

    assert_eq!(&[1, 2, 3], res.as_ref());
}

pub mod ping;
pub mod get;
pub mod search;
pub mod bulk;