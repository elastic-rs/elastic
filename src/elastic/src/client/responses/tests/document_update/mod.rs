// TODO:
// - error: action_request_validation
// - error: document_missing_exception

use crate::{
    client::responses::*,
    error::*,
    http::{
        receiver::{
            parse,
            ResponseError,
        },
        StatusCode,
    },
};

#[test]
fn success_parse_updated_doc_response() {
    let f = include_bytes!("update_updated.json");
    let deserialized = parse::<UpdateResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(5), deserialized.version());

    assert!(deserialized.updated());
}

#[test]
fn success_parse_noop_doc_response() {
    let f = include_bytes!("update_noop.json");
    let deserialized = parse::<UpdateResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(4), deserialized.version());

    assert!(!deserialized.updated());
}

#[test]
fn error_parse_document_missing() {
    let f = include_bytes!("../error/error_document_missing.json");
    let deserialized = parse::<UpdateResponse>()
        .from_slice(StatusCode::NOT_FOUND, f as &[_])
        .unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::DocumentMissing { ref index }) if index == "carrots" => true,
        _ => false,
    };

    assert!(valid);
}
