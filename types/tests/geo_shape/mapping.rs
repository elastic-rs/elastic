use serde_json;
use elastic_types::prelude::*;
use ::geo_shape_fixtures::*;

#[test]
fn serialise_mapping_default() {
    let ser = serde_json::to_string(&Field::from(DefaultGeoShapeMapping)).unwrap();

    let expected = json_str!({
        "type": "geo_shape"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
    let ser = serde_json::to_string(&Field::from(MyGeoShapeMapping)).unwrap();

    let expected = json_str!({
        "type": "geo_shape",
        "tree": "geohash",
        "precision": "50m",
        "tree_levels": 8,
        "strategy": "recursive",
        "distance_error_pct": 0.5,
        "orientation": "cw",
        "points_only": false
    });

    assert_eq!(expected, ser);
}
