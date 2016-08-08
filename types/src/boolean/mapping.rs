//! Mapping for the Elasticsearch `boolean` type.

use std::marker::PhantomData;
use serde;
use serde::Serialize;
use ::mapping::ElasticFieldMapping;

/// Elasticsearch datatype name.
pub const BOOLEAN_DATATYPE: &'static str = "boolean";

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
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
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
/// //We also need to implement the base `ElasticFieldMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticFieldMapping<()> for MyBooleanMapping {
/// 	type Visitor = ElasticBooleanMappingVisitor<MyBooleanMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		BOOLEAN_DATATYPE
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
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	const BOOST: Option<f32> = None;

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	const DOC_VALUES: Option<bool> = None;

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	const INDEX: Option<bool> = None;

	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	const NULL_VALUE: Option<bool> = None;

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	const STORE: Option<bool> = None;
}

macro_rules! impl_boolean_mapping {
	($t:ident) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str {
				$crate::boolean::mapping::BOOLEAN_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 6));

				try!(serializer.serialize_struct_elt(&mut state, "type", M::data_type()));

				ser_field!(serializer, &mut state, $t::BOOST, "boost");
				ser_field!(serializer, &mut state, $t::DOC_VALUES, "doc_values");
				ser_field!(serializer, &mut state, $t::INDEX, "index");
				ser_field!(serializer, &mut state, $t::STORE, "store");
				ser_field!(serializer, &mut state, $t::NULL_VALUE, "null_value");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Default mapping for `bool`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl ElasticBooleanMapping for DefaultBooleanMapping { }

impl_boolean_mapping!(DefaultBooleanMapping);
