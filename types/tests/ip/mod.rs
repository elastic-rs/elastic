#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use std::net::Ipv4Addr;
use elastic_types::ip::mapping::*;
use elastic_types::ip::prelude::*;
use ::ip_fixtures::*;

#[test]
fn can_change_ip_mapping() {
	fn takes_custom_mapping(_: ElasticIp<MyIpMapping>) -> bool {
		true
	}

	let ip: ElasticIp<DefaultIpMapping> = ElasticIp::new(Ipv4Addr::new(127, 0, 0, 1));

	assert!(takes_custom_mapping(ip.remap()));
}

#[test]
fn serialise_elastic_ip() {
	let ip: ElasticIp<DefaultIpMapping> = ElasticIp::new(Ipv4Addr::new(127, 0, 0, 1));

	let ser = serde_json::to_string(&ip).unwrap();

	assert_eq!(r#""127.0.0.1""#, ser);
}

#[test]
fn deserialise_elastic_ip() {
	let ip: ElasticIp<DefaultIpMapping> = serde_json::from_str(r#""127.0.0.1""#).unwrap();

	assert_eq!(Ipv4Addr::new(127, 0, 0, 1), ip);
}
