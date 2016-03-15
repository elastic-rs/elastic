#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use serde_json::ser::Serializer;
use elastic_types::mapping::TypeMapper;
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

//TODO: Derive this -----
mod mytype_mapping {
	use std::marker::PhantomData;
	use std::borrow::Cow;
	use serde;
	use serde::Serialize;
	use elastic_types::mapping::DataMapper;
	use elastic_types::mapping::prelude::*;
	use super::MyType;

	//Implement the base data type on our struct
	impl <'a> ElasticDataType<MyTypeMapping<'a>, ()> for MyType { }

	//Define our custom mapping type for our struct
	#[derive(Default, Clone)]
	struct MyTypeMapping<'a> {
		phantom: PhantomData<&'a ()>
	}

	//Implement the base mapping type for our mapping 
	impl <'a> ElasticMapping<()> for MyTypeMapping<'a> {
		type Visitor = MyTypeMappingVisitor<'a>;

		fn data_type() -> &'static str {
			"mytype"
		}
	}

	//Implement the type mapping type for our mapping
	impl <'a> TypeMapping<'a, MyType> for MyTypeMapping<'a> {
		type Visitor = MyTypeMappingVisitor<'a>;
	}

	//Implement serialisation for our mapping
	impl <'a> Serialize for MyTypeMapping<'a> {
		fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer {
			serializer.serialize_struct("properties", MyTypeMappingVisitor::default())
		}
	}

	//Define a visitor for our mapping
	struct MyTypeMappingVisitor<'a> { 
		data: Cow<'a, MyType>
	}

	//Implement the base type mapping visitor for our visitor
	impl <'a> TypeMappingVisitor<'a, MyType> for MyTypeMappingVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypeMappingVisitor {
				data: Cow::Borrowed(data)
			}
		}
	}

	impl <'a> Default for MyTypeMappingVisitor<'a> {
		fn default() -> MyTypeMappingVisitor<'a> {
			MyTypeMappingVisitor {
				data: Cow::Owned(MyType::default())
			}
		}
	}

	//Derive serialisation for our visitor
	impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			//Dispatch serialisation of the mappable properties
			//Needs to iterate over each property and map
			try!(DataMapper::map("my_date1", &self.data.my_date1, serializer));
			try!(DataMapper::map("my_date2", &self.data.my_date2, serializer));
			try!(DataMapper::map("my_string", &self.data.my_string, serializer));
			try!(DataMapper::map("my_num", &self.data.my_num, serializer));

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

	let date2_map = mydatemapping_json();
	let expected = json!(date2_map, {
		"properties": {
			"my_date1":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_date2": $date2_map,
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