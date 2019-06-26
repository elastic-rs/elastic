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
fn success_parse_index_ops() {
    let f = include_bytes!("bulk_index.json");
    let deserialized = parse::<BulkResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_ok());

    assert_eq!(0, deserialized.iter().filter(Result::is_err).count());
    assert_eq!(5, deserialized.iter().filter(Result::is_ok).count());
}

#[test]
fn success_parse_index_ops_errors_only() {
    let f = include_bytes!("bulk_index.json");
    let deserialized = parse::<BulkErrorsResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_ok());
    assert_eq!(0, deserialized.iter().count());
}

#[test]
fn success_parse_multi_ops() {
    let f = include_bytes!("bulk_multiple_ops.json");
    let deserialized = parse::<BulkResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_ok());

    let mut index_count = 0;
    let mut create_count = 0;
    let mut update_count = 0;
    let mut delete_count = 0;

    for item in deserialized.into_iter().filter_map(Result::ok) {
        match item.action() {
            bulk::Action::Index => index_count += 1,
            bulk::Action::Create => create_count += 1,
            bulk::Action::Update => update_count += 1,
            bulk::Action::Delete => delete_count += 1,
        }
    }

    assert_eq!(
        (1, 1, 1, 1),
        (index_count, create_count, update_count, delete_count)
    );
}

#[test]
fn success_parse_multi_ops_errors_only() {
    let f = include_bytes!("bulk_multiple_ops.json");
    let deserialized = parse::<BulkErrorsResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_ok());
    assert_eq!(0, deserialized.iter().count());
}

#[test]
fn success_parse_with_errors() {
    let f = include_bytes!("bulk_error.json");
    let deserialized = parse::<BulkResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_err());

    assert_eq!(1, deserialized.iter().filter(Result::is_err).count());
    assert_eq!(1, deserialized.iter().filter(Result::is_ok).count());
}

#[test]
fn success_parse_with_errors_errors_only() {
    let f = include_bytes!("bulk_error.json");
    let deserialized = parse::<BulkErrorsResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.is_err());

    assert_eq!(1, deserialized.iter().count());
}

#[test]
fn error_parse_action_request_validation() {
    let f = include_bytes!("../error/error_action_request_validation.json");
    let deserialized = parse::<BulkResponse>()
        .from_slice(StatusCode::BAD_REQUEST, f as &[_])
        .unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::ActionRequestValidation { ref reason })
            if reason == "Validation Failed: 1: index is missing;2: type is missing;" =>
        {
            true
        }
        _ => false,
    };

    assert!(valid);
}

#[test]
fn error_parse_action_request_validation_errors_only() {
    let f = include_bytes!("../error/error_action_request_validation.json");
    let deserialized = parse::<BulkErrorsResponse>()
        .from_slice(StatusCode::BAD_REQUEST, f as &[_])
        .unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::ActionRequestValidation { ref reason })
            if reason == "Validation Failed: 1: index is missing;2: type is missing;" =>
        {
            true
        }
        _ => false,
    };

    assert!(valid);
}
