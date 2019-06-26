extern crate elastic_responses;
extern crate serde;
extern crate serde_json;

use elastic_responses::{
    error::*,
    *,
};
use load_file;
use serde_json::Value;

#[test]
fn success_parse_empty() {
    let f = load_file("tests/samples/search_empty.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 0);
}

#[test]
fn success_parse_hits_simple() {
    let f = load_file("tests/samples/search_hits_only.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn success_parse_hits_simple_of_t() {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Event {
        #[serde(rename = "@version")]
        version: String,
        #[serde(rename = "@timestamp")]
        timestamp: String,
        port: u16,
        #[serde(rename = "type")]
        ty: String,
        tags: Vec<String>,
        #[serde(rename = "destinationAddress")]
        destination_address: String,
        #[serde(rename = "countryCode", default)]
        country_code: String,
        #[serde(rename = "countryName", default)]
        country_name: String,
        #[serde(rename = "cityName", default)]
        city_name: String,
        #[serde(rename = "internetServiceProviderName", default)]
        internet_service_provider_name: String,
        #[serde(rename = "syslogProgram")]
        syslog_program: String,
    }

    let f = load_file("tests/samples/search_hits_only.json");
    let deserialized = parse::<SearchResponse<Event>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn success_parse_hits_no_score() {
    let f = load_file("tests/samples/search_null_score.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 1);
}

#[test]
fn success_parse_hits_bank_sample() {
    let f = load_file("tests/samples/search_bank_sample.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 10);
}

#[test]
fn success_aggs_when_not_present() {
    let f = load_file("tests/samples/search_hits_only.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.aggs().count(), 0);
}

#[test]
fn success_parse_simple_aggs() {
    let f = load_file("tests/samples/search_aggregation_simple.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    let _agg = deserialized
        .aggs()
        .filter_map(|agg| agg.get("myagg").and_then(|val| val.as_f64()))
        .nth(0);

    // TODO: Enable once we support simple values
    // assert_eq!(Some(10f64), agg);
}

#[test]
fn success_parse_simple_nested_aggs() {
    let f = load_file("tests/samples/search_aggregation_simple_nested.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.aggs().count(), 124);

    let doc_count = deserialized
        .aggs_raw()
        .and_then(|aggs| aggs["timechart"]["buckets"][0]["doc_count"].as_u64());

    assert_eq!(Some(101), doc_count);
}

#[test]
fn success_parse_3level_aggs() {
    let f = load_file("tests/samples/search_aggregation_3level.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!(deserialized.aggs().count(), 201);
}

#[test]
fn success_parse_3level_multichild_aggs() {
    let f = load_file("tests/samples/search_aggregation_3level_multichild.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    let mut first = true;
    let mut count = 0;

    for i in deserialized.aggs().take(500000) {
        count += 1;
        if first {
            assert_eq!(&json!(12), i["max_ack_pkts_sent"]);
            assert_eq!(&json!(7), i["avg_ack_pkts_sent"]);
            assert_eq!(&json!(2), i["min_ack_pkts_sent"]);

            first = false;
        }
    }
    assert_eq!(count, 201);
}

#[test]
fn success_parse_3level_multistats_aggs() {
    let f = load_file("tests/samples/search_aggregation_3level_multistats.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().take(500000) {
        count += 1;
        if first {
            assert_eq!(&json!(2), i["extstats_ack_pkts_sent_min"]);
            assert_eq!(&json!(7), i["stats_ack_pkts_sent_avg"]);
            assert_eq!(&json!(12), i["extstats_ack_pkts_sent_max"]);
            assert_eq!(
                &json!(17),
                i["extstats_ack_pkts_sent_std_deviation_bounds_upper"]
            );

            first = false;
        }
    }
    assert_eq!(count, 61);
}

#[test]
fn success_parse_simple_aggs_no_empty_first_record() {
    let f = load_file("tests/samples/search_aggregation_simple.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    let agg = "timechart";
    let mut first = true;
    for i in deserialized.aggs().take(50) {
        if first {
            assert!(i.contains_key(agg));
            first = false;
        }
    }
}

#[test]
fn success_parse_hits_simple_as_value() {
    let f = load_file("tests/samples/search_hits_only.json");
    let deserialized = parse::<Value>().from_reader(StatusCode::OK, f).unwrap();

    assert_eq!(deserialized["_shards"]["total"].as_u64().unwrap(), 5);
}

#[test]
fn error_parse_index_not_found() {
    let f = load_file("tests/samples/error_index_not_found.json");
    let deserialized = parse::<SearchResponse<Value>>()
        .from_reader(StatusCode::NOT_FOUND, f)
        .unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexNotFound { ref index }) if index == "carrots" => true,
        _ => false,
    };

    assert!(valid);
}
