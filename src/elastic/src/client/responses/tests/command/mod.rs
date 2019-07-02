use crate::{
    client::responses::*,
    http::{
        receiver::parse,
        StatusCode,
    },
};

#[test]
fn success_parse_command_response() {
    let f = include_bytes!("acknowledged.json");
    let deserialized = parse::<CommandResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert!(deserialized.acknowledged());
}
