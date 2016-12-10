use serde;
use serde_json;
use chrono;
use elastic_types;

use std::net::Ipv4Addr;
use chrono::offset::TimeZone;

use elastic_types::response::*;
use ::object_fixtures::*;

#[test]
fn deserialise_elastic_response_hits() {
    let expected = vec![
        MyStruct {
            id: 1,
            title: String::from("Some Title"),
            timestamp: chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap(),
            geo: GeoLocation {
                ip: Ipv4Addr::new(10, 0, 0, 1)
            }
        },
        MyStruct {
            id: 2,
            title: String::from("Some Other Title"),
            timestamp: chrono::UTC.datetime_from_str("15/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap(),
            geo: GeoLocation {
                ip: Ipv4Addr::new(10, 0, 0, 2)
            }
        }
    ];

    let ser = json_str!({
        "took" : 63,
        "timed_out" : false,
        "_shards" : {
            "total" : 5,
            "successful" : 5,
            "failed" : 0
        },
        "hits" : {
            "total" : 2,
            "max_score" : 1.0,
            "hits" : [
                {
                    "_index" : "test",
                    "_type" : "mystruct",
                    "_id" : "1",
                    "_score" : 1.0,
                    "_source" : {
                        "id": 1,
                        "title": "Some Title",
                        "timestamp": "2015-05-13T00:00:00Z",
                        "geo": {
                            "ip": "10.0.0.1"
                        }
                    }
                },
                {
                    "_index" : "test",
                    "_type" : "mystruct",
                    "_id" : "2",
                    "_score" : 1.0,
                    "_source" : {
                        "id": 2,
                        "title": "Some Other Title",
                        "timestamp": "2015-05-15T00:00:00Z",
                        "geo": {
                            "ip": "10.0.0.2"
                        }
                    }
                }
            ]
        }
    });

    let res: SearchResponse<MyStruct> = serde_json::from_str(&ser).unwrap();

    let mut hits = Vec::<MyStruct>::with_capacity(2);
    for hit in res.hits.hits {
        if let Some(hit) = hit.source {
            hits.push(hit.clone());
        }
    }

    assert_eq!(expected, hits);
}
