#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use serde_json::ser::Serializer;
use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;
use elastic_types::string::prelude::*;
use ::date_fixtures::*;

//A type we can add to Elasticsearch
#[derive(Default, Clone, Serialize)]
struct MyType {
	pub my_date1: DateTime,
	pub my_date2: DateTime<EpochMillis, MyDateMapping>,
	pub my_string: ElasticString<DefaultStringMapping>,
	pub my_num: i32
}

//TODO: Macro for deriving this. Should only need to take props to map in definition
mod mytype_mapping {
	use std::marker::PhantomData;
	use std::borrow::Cow;
	use serde;
	use serde::Serialize;
	use elastic_types::mapping::prelude::*;
	use super::MyType;

	impl <'a> ElasticType<MyTypeMapping<'a>, ()> for MyType { }

	#[derive(Default, Clone)]
	struct MyTypeMapping<'a> {
		phantom: PhantomData<&'a ()>
	}

	impl <'a> ElasticTypeMapping<()> for MyTypeMapping<'a> {
		type Visitor = MyTypeNestedMappingVisitor<'a>;

		fn data_type() -> &'static str {
			"nested"
		}
	}
	impl <'a> ElasticUserTypeMapping<'a, MyType> for MyTypeMapping<'a> {
		type Visitor = MyTypeMappingVisitor<'a>;
		type PropertiesVisitor = MyTypePropertiesVisitor<'a>;

		fn name() -> &'static str {
			"mytype"
		}
	}

	impl <'a> serde::Serialize for MyTypeMapping<'a> {
		fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer {
			serializer.serialize_struct(Self::name(), MyTypeNestedMappingVisitor::default())
		}
	}

	struct MyTypeNestedMappingVisitor<'a> { 
		data: Cow<'a, MyType>
	}

	impl <'a> MyTypeNestedMappingVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypeNestedMappingVisitor {
				data: Cow::Borrowed(data)
			}
		}
	}

	impl <'a> Default for MyTypeNestedMappingVisitor<'a> {
		fn default() -> MyTypeNestedMappingVisitor<'a> {
			MyTypeNestedMappingVisitor {
				data: Cow::Owned(MyType::default())
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypeNestedMappingVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt("type", MyTypeMapping::data_type()));
			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));

			Ok(None)
		}
	}

	struct MyTypeMappingVisitor<'a> { 
		data: &'a MyType
	}

	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypeMappingVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypeMappingVisitor {
				data: data
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));

			Ok(None)
		}
	}

	struct MyTypePropertiesVisitor<'a> {
		data: &'a MyType
	}

	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypePropertiesVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypePropertiesVisitor {
				data: data
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypePropertiesVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(FieldMapper::map("my_date1", &self.data.my_date1, serializer));
			try!(FieldMapper::map("my_date2", &self.data.my_date2, serializer));
			try!(FieldMapper::map("my_string", &self.data.my_string, serializer));
			try!(FieldMapper::map("my_num", &self.data.my_num, serializer));

			Ok(None)
		}
	}
}

#[test]
fn serialise_mapping_type() {
	//Define an instance of our mapping type
	let mytype = MyType::default();

	//Build a serialiser and use the mapper to serialise the mapping for the given type
	let mut writer = Vec::with_capacity(128);
	{
		let mut ser = Serializer::new(&mut writer);
		let _ = TypeMapper::map(&mytype, &mut ser).unwrap();
	}
	let ser = String::from_utf8(writer).unwrap();

	let expected = json_str!({
		"properties": {
			"my_date1":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_date2": {
				"type": "date",
				"boost": 1.01,
				"doc_values": true,
				"include_in_all": false,
				"index": "no",
				"store": true,
				"format": "epoch_millis",
				"ignore_malformed": true,
				"null_value": "0",
				"precision_step": 6
			},
			"my_string":{
				"type":"string"
			},
			"my_num":{
				"type":"object"
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_type_as_nested() {
	//Define an instance of our mapping type
	let mytype = MyType::default();

	//Build a serialiser and use the mapper to serialise the mapping for the given type
	let mut writer = Vec::with_capacity(128);
	{
		let mut ser = Serializer::new(&mut writer);
		let _ = FieldMapper::map("mytype", &mytype, &mut ser).unwrap();
	}
	let ser = String::from_utf8(writer).unwrap();

	//TODO: Test this on a derived subtype
	let expected = json_str!(, "mytype": {
		"type": "nested",
		"properties": {
			"my_date1":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_date2": {
				"type": "date",
				"boost": 1.01,
				"doc_values": true,
				"include_in_all": false,
				"index": "no",
				"store": true,
				"format": "epoch_millis",
				"ignore_malformed": true,
				"null_value": "0",
				"precision_step": 6
			},
			"my_string":{
				"type":"string"
			},
			"my_num":{
				"type":"object"
			}
		}
	});

	assert_eq!(expected, ser);
}