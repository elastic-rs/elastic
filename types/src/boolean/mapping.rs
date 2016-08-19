//! Mapping for the Elasticsearch `boolean` type.

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
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # use elastic_types::prelude::*;
/// # fn main() {
/// boolean_mapping!(MyBooleanMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// });
/// # }
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # extern crate serde;
/// # extern crate elastic_types;
/// # boolean_mapping!(MyBooleanMapping {
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # });
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
pub trait ElasticBooleanMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> { None }

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> { None }

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<bool> { None }

	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<bool> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }
}

/// Implement `serde` serialisation for a `boolean` mapping type.
#[macro_export]
macro_rules! boolean_ser {
    ($t:ident) => (
		impl ::serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 6));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::data_type()));

				ser_field!(serializer, &mut state, $t::boost(), "boost");
				ser_field!(serializer, &mut state, $t::doc_values(), "doc_values");
				ser_field!(serializer, &mut state, $t::index(), "index");
				ser_field!(serializer, &mut state, $t::store(), "store");
				ser_field!(serializer, &mut state, $t::null_value(), "null_value");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define a `boolean` mapping.
/// 
/// # Examples
/// 
/// ## Define mapping struct inline
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// # fn main() {}
/// boolean_mapping!(MyMapping {
/// 	fn null_value() -> Option<bool> { Some(true) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping` and `ElasticBooleanMapping`, along with a few default traits:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// ```
/// 
/// ## Define mapping for existing struct
/// 
/// If you want to control the default implementations yourself, you can define your
/// mapping type and just pass it the macro to implement `ElasticFieldMapping`:
/// 
/// ```
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// # fn main() {}
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// impl ElasticBooleanMapping for MyMapping { 
/// 	fn null_value() -> Option<bool> { Some(true) }
/// }
/// 
/// boolean_mapping!(MyMapping);
/// ```
#[macro_export]
macro_rules! boolean_mapping {
	($t:ident) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { $crate::boolean::mapping::BOOLEAN_DATATYPE }
		}

		boolean_ser!($t);
	);
	($t:ident $b:tt) => (
		#[derive(Debug, Default, Clone, Copy)]
		pub struct $t;

		impl $crate::boolean::mapping::ElasticBooleanMapping for $t $b

		boolean_mapping!($t);
	)
}

/// Default mapping for `bool`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl ElasticBooleanMapping for DefaultBooleanMapping { }

boolean_mapping!(DefaultBooleanMapping);