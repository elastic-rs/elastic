//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//! 
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
//! 
//! # Examples
//! 
//! Derive `ElasticType` on your Elasticsearch-mappable types:
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::DateTime;
//! 
//! #[derive(Default, Clone, Serialize, Deserialize)]
//! pub struct MyType {
//! 	pub my_date: DateTime,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//! 
//! #[derive(Default, Clone)]
//! struct MyTypeMapping;
//! impl ElasticObjectMapping for MyTypeMapping {
//! 	fn data_type() -> &'static str {
//! 		"object"
//! 	}
//! 
//! 	fn dynamic() -> Option<Dynamic> {
//! 		Some(Dynamic::True)
//! 	}
//! 
//! 	fn enabled() -> Option<bool> {
//! 		Some(false)
//! 	}
//! 
//! 	fn include_in_all() -> Option<bool> {
//! 		Some(true)
//! 	}
//! }
//! 
//! impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [my_date, my_string, my_num]);
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![feature(custom_derive, custom_attribute, plugin, optin_builtin_traits, associated_type_defaults)]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![plugin(serde_macros, elastic_macros)]

extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
pub mod macros;
pub mod mapping;
pub mod mappers;

pub mod object;
pub mod date;
pub mod string;
pub mod number;

impl_mapping!(
	bool,
	char
);

//TODO: This should map as T
impl <T: serde::Serialize + serde::Deserialize> mapping::ElasticType<mapping::NullMapping, ()> for Vec<T> { }