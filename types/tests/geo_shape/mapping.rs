use elastic_types::mapping::prelude::*;
use ::geo_shape_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let ser = FieldMapper::to_string(DefaultGeoShapeMapping).unwrap();

	let expected = json_str!({
		"type": "geo_shape"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let ser = FieldMapper::to_string(MyGeoShapeMapping).unwrap();

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
