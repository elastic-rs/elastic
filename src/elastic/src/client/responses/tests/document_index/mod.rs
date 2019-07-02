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
fn success_parse_response() {
    let f = include_bytes!("index_success.json");
    let deserialized = parse::<IndexResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.created());
    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(1), deserialized.version());
}

#[test]
fn error_parse_index_already_exists() {
    let f = include_bytes!("../error/error_index_already_exists.json");
    let deserialized = parse::<IndexResponse>()
        .from_slice(StatusCode::BAD_REQUEST, f as &[_])
        .unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexAlreadyExists { ref index }) if index == "carrots" => {
            true
        }
        _ => false,
    };

    assert!(valid);
}
