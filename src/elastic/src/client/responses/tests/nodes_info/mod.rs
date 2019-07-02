use crate::{
    client::responses::*,
    http::{
        receiver::parse,
        StatusCode,
    },
};

#[test]
fn deserialise_nodes() {
    let f = include_bytes!("nodes_info.json");
    let deserialized = parse::<NodesInfoResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    let expected = vec!["1.1.1.1:9200", "1.1.1.2:9200"];

    assert_eq!(expected, deserialized.iter_addrs().collect::<Vec<_>>());
}

#[test]
fn deserialise_nodes_empty() {
    let f = include_bytes!("nodes_info_empty.json");
    let deserialized = parse::<NodesInfoResponse>()
        .from_slice(StatusCode::OK, f as &[_])
        .unwrap();

    assert_eq!(0, deserialized.iter_addrs().count());
}
