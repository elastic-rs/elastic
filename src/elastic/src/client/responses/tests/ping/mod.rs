use crate::{
    client::responses::*,
    http::{
        receiver::parse,
        StatusCode,
    },
};

#[test]
fn success_parse_ping_response() {
    let f = include_bytes!("ping_success.json");
    let deserialized = parse::<PingResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert_eq!("Scorcher", deserialized.name());
}
