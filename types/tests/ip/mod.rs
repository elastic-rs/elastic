use serde;
use serde_json;
use elastic_types;

pub mod mapping;

use std::net::Ipv4Addr;

use elastic_types::mapping::prelude::*;
use elastic_types::ip::prelude::*;
use ::ip_fixtures::*;

#[test]
fn can_change_ip_mapping() {
    fn takes_custom_mapping(_: Ip<MyIpMapping>) -> bool {
        true
    }

    let ip: Ip<DefaultIpMapping> = Ip::new(Ipv4Addr::new(127, 0, 0, 1));

    assert!(takes_custom_mapping(ip.remap()));
}

#[test]
fn serialise_elastic_ip() {
    let ip: Ip<DefaultIpMapping> = Ip::new(Ipv4Addr::new(127, 0, 0, 1));

    let ser = serde_json::to_string(&ip).unwrap();

    assert_eq!(r#""127.0.0.1""#, ser);
}

#[test]
fn deserialise_elastic_ip() {
    let ip: Ip<DefaultIpMapping> = serde_json::from_str(r#""127.0.0.1""#).unwrap();

    assert_eq!(Ipv4Addr::new(127, 0, 0, 1), ip);
}
