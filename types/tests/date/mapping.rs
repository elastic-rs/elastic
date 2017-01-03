use serde;
use serde_json;
use elastic_types;

use chrono::{ DateTime, UTC };
use elastic_types::prelude::*;
use elastic_types::date::prelude::*;
use ::date_fixtures::*;

#[test]
fn datetime_has_default_mapping() {
    assert_eq!(DefaultDateMapping::<ChronoFormat>::default(), DateTime::<UTC>::mapping());
}

#[test]
fn serialise_mapping_default() {
    let ser = serde_json::to_string(&Field::from(DefaultDateMapping::<DefaultDateFormat>::default())).unwrap();

    let expected = json_str!({
        "type": "date",
        "format": "basic_date_time"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
    let ser = serde_json::to_string(&Field::from(MyDateMapping)).unwrap();

    let expected = json_str!({
        "type": "date",
        "format": "epoch_millis",
        "boost": 1.01,
        "doc_values": true,
        "include_in_all": false,
        "index": true,
        "store": true,
        "ignore_malformed": true,
        "null_value": "1426351513778"
    });

    assert_eq!(expected, ser);
}
