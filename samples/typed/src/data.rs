use std::net::Ipv4Addr;
use elastic_types::prelude::*;

// The type we want to index in Elasticsearch
#[derive(Clone, Debug, Serialize, Deserialize, ElasticType)]
pub struct MyStruct {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateFormat>,
    pub geo: GeoLocation,
}

#[derive(Clone, Debug, Serialize, Deserialize, ElasticType)]
pub struct GeoLocation {
    pub ip: Ipv4Addr,
    pub loc: GeoPoint<DefaultGeoPointFormat>,
}

// An index request type with mappings bundled in
#[derive(Default, Serialize)]
pub struct Index {
    mappings: Mappings,
}

#[derive(Default, Serialize)]
struct Mappings {
    mystruct: Document<MyStructMapping>,
}
