extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file_as_response;

#[test]
fn success_parse_with_errors() {
    let s = load_file_as_response(200, "tests/samples/bulk_error.json");
    let deserialized = BulkResponse::from_response(s).unwrap();

    assert_eq!(1, deserialized.items.err.len());
    assert_eq!(1, deserialized.items.ok.len());
}

fn stuff(bulk: BulkResponse) {
    let mut failed_index = vec![];
    let mut failed_create = vec![];
    let mut failed_update = vec![];
    let mut failed_delete = vec![];

    for item in &bulk.items.err {
        match item.action {
            BulkAction::Index => failed_index.push(item.clone()),
            BulkAction::Create => failed_create.push(item.clone()),
            BulkAction::Update => failed_update.push(item.clone()),
            BulkAction::Delete => failed_delete.push(item.clone()),
        }
    }
}