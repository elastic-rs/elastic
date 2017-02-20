pub mod mapping;
pub mod formats;

use serde_json;
use chrono;

use chrono::offset::TimeZone;

use elastic_types::date::prelude::*;

#[derive(ElasticDateFormat, Default, Clone)]
#[elastic(date_format="yyyy/MM/dd HH:mm:ss", date_format_name="test_date_1")]
pub struct NamedDateFormat;

#[derive(ElasticDateFormat, Default, Clone, Copy)]
#[elastic(date_format="yyyyMMdd")]
pub struct UnNamedDateFormat;

#[test]
fn date_format_uses_name_if_supplied() {
    assert_eq!("test_date_1", NamedDateFormat::name());
}

#[test]
fn date_format_uses_format_if_name_not_supplied() {
    assert_eq!("yyyyMMdd", UnNamedDateFormat::name());
}

#[test]
fn dates_should_use_chrono_format() {
    let dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
    let expected = dt.format("%Y/%m/%d %H:%M:%S").to_string();

    let dt = Date::<NamedDateFormat>::new(dt.clone());
    let actual = dt.format();

    assert_eq!(expected, actual);
}

#[test]
fn dates_should_use_es_format() {
    let dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
    let expected = "20150513".to_string();

    let dt = Date::<UnNamedDateFormat>::new(dt.clone());
    let actual = dt.format();

    assert_eq!(expected, actual);
}

#[test]
fn can_change_date_mapping() {
    fn takes_epoch_millis(_: Date<EpochMillis>) -> bool {
        true
    }

    let date: Date<BasicDateTime> = Date::now();

    assert!(takes_epoch_millis(date.remap()));
}

#[test]
fn can_build_date_from_chrono() {
    let date: Date<DefaultDateFormat> = Date::new(
        chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap()
    );

    assert_eq!((2015, 5, 13, 0, 0, 0), (
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
    ));
}

#[test]
fn can_build_date_from_prim() {
    let date: Date<DefaultDateFormat> = Date::build(
        2015, 5, 13, 0, 0, 0, 0
    );

    assert_eq!((2015, 5, 13, 0, 0, 0), (
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
    ));
}

#[test]
fn serialise_elastic_date() {
    let date = Date::<BasicDateTime>::new(
        chrono::UTC.datetime_from_str(
            "13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S"
        ).unwrap()
    );

    let ser = serde_json::to_string(&date).unwrap();

    assert_eq!(r#""20150513T000000.000Z""#, ser);
}

#[test]
fn deserialise_elastic_date() {
    let date: Date<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

    assert_eq!((2015, 5, 13), (
        date.year(),
        date.month(),
        date.day()
    ));
}

#[test]
fn serialise_elastic_date_brw() {
    let chrono_date = chrono::UTC.datetime_from_str(
        "13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S"
    ).unwrap();

    let date = DateBrw::<BasicDateTime>::new(&chrono_date);

    let ser = serde_json::to_string(&date).unwrap();

    assert_eq!(r#""20150513T000000.000Z""#, ser);
}
