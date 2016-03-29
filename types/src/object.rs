//! Requirements for mapping user-defined types.
//! 
//! # Examples
//! 
//! Define your Elasticsearch types using _Plain Old Rust Structures_. 
//! Your types should at least derive `Default`, `Clone` and `serde::Serialize`.
//! 
//! ## Derive Mapping
//! 
//! _TODO: Fill this in_
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
//! # use serde::ser::Serialize;
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::{ DateTime, EpochMillis };
//! use elastic_types::string::ElasticString;
//! 
//! //Define a struct for your type
//! #[derive(Default, Clone, Serialize)]
//! pub struct MyType {
//! 	pub my_date1: DateTime,
//! 	pub my_date2: DateTime<EpochMillis>,
//! 	pub my_string: ElasticString<DefaultStringMapping>,
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
//! 	fn dynamic() -> Option<bool> {
//! 		Some(true)
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
//! impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [my_date1, my_date2, my_string, my_num]);
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
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
//! # use serde::ser::Serialize;
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::{ DateTime, EpochMillis };
//! # use elastic_types::string::ElasticString;
//! # #[derive(Default, Clone, Serialize)]
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
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! #[derive(Default, Clone, Serialize)]
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
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//! 
//! ## Mapping manually
//! 

//TODO: Rework this. It sucks
use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use ::mapping::{ ElasticTypeMapping };

/// Visitor for the `ElasticObjectProperties` struct and given user-defined type.
/// 
/// The purpose of this trait is to serialise the mapping for each datatype on the user-defined type `T`.
/// To make this easier, the `FieldMapper` can be used to infer the mapping for a field that implements `ElasticType`.
/// 
/// # Examples
/// 
/// Implement `ElasticUserTypeVisitor` for a user-defined type:
/// 
/// ```
/// //TODO: Implement
/// ```
pub trait ElasticObjectTypeVisitor<'a, T: 'a + Clone + Default>
where Self: serde::ser::MapVisitor { 
	/// Create a new visitor from a borrowed user-defined type.
	fn new(data: &'a T) -> Self;
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticTypeProperties<'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<M>
}
impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> ElasticTypeProperties<'a, T, M> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticTypeProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> serde::Serialize for ElasticTypeProperties<'a, T, M> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping<T>>::PropertiesVisitor::new(&self.data))
	}
}

/// The base requirements for mapping a user-defined type.
pub trait ElasticUserTypeMapping<'a, T: 'a + Clone + Default>
where Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticObjectTypeVisitor<'a, T>;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticObjectTypeVisitor<'a, T>;

	/// The name of the user-defined type used in Elasticsearch.
	fn name() -> &'static str;
}

pub trait ElasticObjectMapping
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
    fn data_type() -> &'static str {
        "nested"
    }

    fn dynamic() -> Option<bool> {
    	None
    }

	fn enabled() -> Option<bool> {
		None
	}

	fn include_in_all() -> Option<bool> {
		None
	}
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticObjectProperties<'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<V>
}
impl <'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> ElasticObjectProperties<'a, T, V> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticObjectProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> serde::Serialize for ElasticObjectProperties<'a, T, V> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", V::new(&self.data))
	}
}

#[derive(Debug, PartialEq, Default)]
pub struct ElasticObjectMappingVisitor<T: ElasticObjectMapping> {
	phantom: PhantomData<T>
}

impl <T: ElasticObjectMapping> serde::ser::MapVisitor for ElasticObjectMappingVisitor<T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		if let Some(dynamic) = T::dynamic() {
			try!(serializer.serialize_struct_elt("dynamic", dynamic));
		};

		if let Some(enabled) = T::enabled() {
			try!(serializer.serialize_struct_elt("enabled", enabled));
		};

		if let Some(include_in_all) = T::include_in_all() {
			try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
		};

		Ok(None)
	}
}