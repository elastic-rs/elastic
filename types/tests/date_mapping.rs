#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::*;
use elastic_types::date::*;

//A custom date mapping
struct MyDateMapping;
impl ElasticMapping for MyDateMapping {
	type Visitor = ElasticDateMappingVisitor<EpochMillis, MyDateMapping>;
	
	fn get_boost() -> Option<f32> {
		Some(1.01)
	}
}
impl ElasticDateMapping<EpochMillis> for MyDateMapping {
	fn get_null_value() -> Option<NullValue> {
		Some(NullValue::Default("0"))
	}
}

impl serde::Serialize for MyDateMapping {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_struct("mapping", ElasticDateMappingVisitor::<EpochMillis, MyDateMapping>::default())
    }
}

#[test]
fn mapping_serialises_overriden_params() {
	let mapping = MyDateMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"boost":1.01,"format":"epoch_millis","null_value":"0"}"#, ser);
}

#[test]
fn default_mapping_serialises_only_format() {
	let mapping = DefaultDateMapping::<BasicDateTime>::new();
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"format":"basic_date_time"}"#, ser);
}