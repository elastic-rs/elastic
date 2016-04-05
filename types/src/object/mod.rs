//! Requirements for mapping `user-defined` types.
//! 
//! # Examples
//! 
//! Define your Elasticsearch types using _Plain Old Rust Structures_. 
//! Your types should at least derive `Default`, `Clone` and `serde::Serialize`.
//! 
//! ## Mapping with Macros
//! 
//! The `impl_type_mapping` macro can be used to hide a lot of the boilerplate in serialising the mapping:
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::DateTime;
//! 
//! //Define a struct for your type
//! #[derive(Default, Clone, Serialize, Deserialize)]
//! pub struct MyType {
//! 	pub my_date: DateTime,
//! 	pub my_string: String,
//! 	pub my_num: i32
//! }
//! 
//! //Define the object mapping for the type
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
//! The fields passed to `impl_type_mapping` must all implement `ElasticType`. 
//! Note that not all fields need to be included in the macro call.
//! Also remember that Elasticsearch will automatically update mappings based on the objects it sees though,
//! so if your 'un-mapped' field is serialised on `index`, then some mapping will be added for it.
//! 
//! Because `MyType` now implements `ElasticType`, it too can be used in another type, and will be mapped as a [nested object](https://www.elastic.co/guide/en/elasticsearch/guide/current/nested-objects.html):
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::{ DateTime, EpochMillis };
//! # use elastic_types::string::ElasticString;
//! # #[derive(Default, Clone, Serialize, Deserialize)]
//! # pub struct MyType {
//! # 	pub my_date1: DateTime,
//! # 	pub my_date2: DateTime<EpochMillis>,
//! # 	pub my_string: ElasticString<DefaultStringMapping>,
//! # 	pub my_num: i32
//! # }
//! # #[derive(Default, Clone)]
//! # struct MyTypeMapping;
//! # impl ElasticObjectMapping for MyTypeMapping { }
//! # impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [my_date1, my_date2, my_string, my_num]);
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
//! #[derive(Default, Clone, Serialize, Deserialize)]
//! pub struct MyOtherType {
//! 	pub my_date: DateTime,
//! 	pub my_type: MyType,
//! 	pub my_num: i32
//! }
//! 
//! #[derive(Default, Clone)]
//! struct MyOtherTypeMapping;
//! impl ElasticObjectMapping for MyOtherTypeMapping { }
//! 
//! impl_object_mapping!(MyOtherType, MyOtherTypeMapping, "my_other_type", inner2, [my_date, my_type, my_num]);
//! # impl serde::Serialize for MyOtherType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyOtherType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```

use serde;

/// Visitor for serialising a user-defined Rust struct as a field of another type.
pub trait ElasticObjectTypeVisitor<'a, T: 'a + Clone + Default>
where Self: serde::ser::MapVisitor { 
	/// Create a new visitor from a borrowed user-defined type.
	fn new(data: &'a T) -> Self;
}

mod object;
mod user_type;

pub use self::object::*;
pub use self::user_type::*;