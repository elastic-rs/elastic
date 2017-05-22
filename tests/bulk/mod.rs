extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file;

#[test]
fn success_parse_index_ops() {
    let f = load_file("tests/samples/bulk_index.json");
    let deserialized = parse::<BulkResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_ok());

    assert_eq!(0, deserialized.items.err.len());
    assert_eq!(5, deserialized.items.ok.len());
}

#[test]
fn success_parse_index_ops_errors_only() {
    let f = load_file("tests/samples/bulk_index.json");
    let deserialized = parse::<BulkErrorsResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_ok());
    assert_eq!(0, deserialized.items.len());
}

#[test]
fn success_parse_multi_ops() {
    let f = load_file("tests/samples/bulk_multiple_ops.json");
    let deserialized = parse::<BulkResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_ok());

    let mut index_count = 0;
    let mut create_count = 0;
    let mut update_count = 0;
    let mut delete_count = 0;

    for item in deserialized.items.ok {
        match item.action {
            BulkAction::Index => index_count += 1,
            BulkAction::Create => create_count += 1,
            BulkAction::Update => update_count += 1,
            BulkAction::Delete => delete_count += 1,
        }
    }

    assert_eq!((1, 1, 1, 1), (index_count, create_count, update_count, delete_count));
}

#[test]
fn success_parse_multi_ops_errors_only() {
    let f = load_file("tests/samples/bulk_multiple_ops.json");
    let deserialized = parse::<BulkErrorsResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_ok());
    assert_eq!(0, deserialized.items.len());
}

#[test]
fn success_parse_with_errors() {
    let f = load_file("tests/samples/bulk_error.json");
    let deserialized = parse::<BulkResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_err());

    assert_eq!(1, deserialized.items.err.len());
    assert_eq!(1, deserialized.items.ok.len());
}

#[test]
fn success_parse_with_errors_errors_only() {
    let f = load_file("tests/samples/bulk_error.json");
    let deserialized = parse::<BulkErrorsResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.is_err());

    assert_eq!(1, deserialized.items.len());
}

#[test]
fn error_parse_action_request_validation() {
    let f = load_file("tests/samples/error_action_request_validation.json");
    let deserialized = parse::<BulkResponse>().from_reader(400, f).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::ActionRequestValidation { ref reason })
        if reason == "Validation Failed: 1: index is missing;2: type is missing;" => true,
        _ => false
    };

    assert!(valid);
}

#[test]
fn error_parse_action_request_validation_errors_only() {
    let f = load_file("tests/samples/error_action_request_validation.json");
    let deserialized = parse::<BulkErrorsResponse>().from_reader(400, f).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::ActionRequestValidation { ref reason })
        if reason == "Validation Failed: 1: index is missing;2: type is missing;" => true,
        _ => false
    };

    assert!(valid);
}