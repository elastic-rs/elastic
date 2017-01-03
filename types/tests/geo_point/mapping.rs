use serde_json;
use elastic_types::prelude::*;
use ::geo_point_fixtures::*;

#[test]
fn serialise_mapping_default() {
    let ser = serde_json::to_string(&Field::from(DefaultGeoPointMapping::<DefaultGeoPointFormat>::default())).unwrap();

    let expected = json_str!({
        "type": "geo_point"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
    let ser = serde_json::to_string(&Field::from(MyGeoPointMapping)).unwrap();

    let expected = json_str!({
        "type": "geo_point",
        "geohash": false,
        "geohash_precision": "50m",
        "geohash_prefix": true,
        "ignore_malformed": true,
        "lat_lon": true
    });

    assert_eq!(expected, ser);
}
