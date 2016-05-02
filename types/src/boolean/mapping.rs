//! Mapping for the Elasticsearch `boolean` type.

use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use ::mapping::{ ElasticTypeMapping, ElasticTypeVisitor, IndexAnalysis };

/// The base requirements for mapping a `boolean` type.
///
/// Custom mappings can be defined by implementing `ElasticBooleanMapping`.
///
/// # Examples
///
/// Define a custom `ElasticBooleanMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::boolean::prelude::*;
///
/// #[derive(Debug, Clone, Default, ElasticBooleanMapping)]
/// pub struct MyBooleanMapping;
/// impl ElasticBooleanMapping for MyBooleanMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::mapping::prelude::*;
/// # use elastic_types::boolean::prelude::*;
/// # #[derive(Debug, Clone, Default, ElasticBooleanMapping)]
/// # pub struct MyBooleanMapping;
/// # impl ElasticBooleanMapping for MyBooleanMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyBooleanMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "boolean",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Manually
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::boolean::prelude::*;
///
/// #[derive(Debug, Clone, Default)]
/// pub struct MyBooleanMapping;
/// impl ElasticBooleanMapping for MyBooleanMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticTypeMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticTypeMapping<()> for MyBooleanMapping {
/// 	type Visitor = ElasticBooleanMappingVisitor<MyBooleanMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		"boolean"
/// 	}
/// }
///
/// impl serde::Serialize for MyBooleanMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticBooleanMapping where
Self: ElasticTypeMapping<()> + Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> {
		None
	}

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<IndexAnalysis> {
		None
	}

	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<bool> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> {
		None
	}
}

/// Default mapping for `bool`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl ElasticBooleanMapping for DefaultBooleanMapping { }

impl_boolean_mapping!(DefaultBooleanMapping);

/// Base visitor for serialising string mappings.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticBooleanMappingVisitor<T> where T: ElasticBooleanMapping {
	phantom: PhantomData<T>
}

impl <T> ElasticTypeVisitor for ElasticBooleanMappingVisitor<T> where
T: ElasticBooleanMapping {
	fn new() -> Self {
		ElasticBooleanMappingVisitor {
			phantom: PhantomData
		}
	}
}
impl <T> serde::ser::MapVisitor for ElasticBooleanMappingVisitor<T> where
T: ElasticBooleanMapping {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", T::data_type()));

		if let Some(boost) = T::boost() {
			try!(serializer.serialize_struct_elt("boost", boost));
		}

		if let Some(doc_values) = T::doc_values() {
			try!(serializer.serialize_struct_elt("doc_values", doc_values));
		}

		if let Some(index) = T::index() {
			try!(serializer.serialize_struct_elt("index", index));
		}

		if let Some(store) = T::store() {
			try!(serializer.serialize_struct_elt("store", store));
		}

		if let Some(null_value) = T::null_value() {
			try!(serializer.serialize_struct_elt("null_value", null_value));
		}

		Ok(None)
	}
}
