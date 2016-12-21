use std::net::Ipv4Addr;
use serde::ser::{Serializer, Error};
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
    #[serde(serialize_with = "serialise_mappings")]
    mappings: (),
}

fn serialise_mappings<S>(_: &(), serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer
{
    let mut state = serializer.serialize_struct("mappings", 1)?;

    let mapping = TypeMapper::to_value(MyStruct::mapping()).map_err(|e| S::Error::custom("failed to build mapping"))?;

    serializer.serialize_struct_elt(&mut state, MyStruct::name(), mapping)?;
    
    serializer.serialize_struct_end(state)
}
