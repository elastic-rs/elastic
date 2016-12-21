//! Mapping for the Elasticsearch `number` types.
//!
//! Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
//! The implementation is the same for all number types, the only difference is the return type of `null_value`.
//!
//! # Examples
//!
//! Define a custom `IntegerMapping`:
//!
//! ## Derive Mapping
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_derive)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Default)]
//! struct MyIntegerMapping;
//! impl IntegerMapping for MyIntegerMapping {
//! 	//Overload the mapping functions here
//! 	fn null_value() -> Option<i32> {
//! 		Some(42)
//! 	}
//! }
//! # fn main() {}
//! ```
//!
//! This will produce the following mapping:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_types_derive)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # #[derive(Default)]
//! # struct MyIntegerMapping;
//! # impl IntegerMapping for MyIntegerMapping {
//! # 	//Overload the mapping functions here
//! # 	fn null_value() -> Option<i32> {
//! # 		Some(42)
//! # 	}
//! # }
//! # fn main() {
//! # let mapping = FieldMapper::to_string(MyIntegerMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "type": "integer",
//! 	"null_value": 42
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```

use serde::Serialize;
use ::mapping::{ElasticType, ElasticFieldMapping, Field};

/// Elasticsearch datatype name.
pub const INTEGER_DATATYPE: &'static str = "integer";
/// Elasticsearch datatype name.
pub const LONG_DATATYPE: &'static str = "long";
/// Elasticsearch datatype name.
pub const SHORT_DATATYPE: &'static str = "short";
/// Elasticsearch datatype name.
pub const BYTE_DATATYPE: &'static str = "byte";
/// Elasticsearch datatype name.
pub const DOUBLE_DATATYPE: &'static str = "double";
/// Elasticsearch datatype name.
pub const FLOAT_DATATYPE: &'static str = "float";

macro_rules! number_mapping {
	($m:ident, $f:ident, $cn:ident, $n:ty) => (
		#[doc(hidden)]
		#[derive(Default)]
		pub struct $f;

/// Base `number` mapping.
		pub trait $m where
		Self: Default {
/// Try to convert strings to numbers and truncate fractions for integers. Accepts `true` (default) and `false`.
			fn coerce() -> Option<bool> { None }

/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
			fn boost() -> Option<f32> { None }

/// Should the field be stored on disk in a column-stride fashion,
/// so that it can later be used for sorting, aggregations, or scripting?
/// Accepts `true` (default) or `false`.
			fn doc_values() -> Option<bool> { None }

/// If `true`, malformed numbers are ignored. If `false` (default),
/// malformed numbers throw an exception and reject the whole document.
			fn ignore_malformed() -> Option<bool> { None }

/// Whether or not the field value should be included in the `_all` field?
/// Accepts `true` or `false`. Defaults to false if index is set to no,
/// or if a parent object field sets `include_in_all` to false.
/// Otherwise defaults to `true`.
			fn include_in_all() -> Option<bool> { None }

/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
			fn index() -> Option<bool> { None }

/// Accepts a numeric value of the same type as the field which is substituted for any explicit null values.
/// Defaults to `null`, which means the field is treated as missing.
			fn null_value() -> Option<$n> { None }

/// Whether the field value should be stored and retrievable separately from the `_source` field.
/// Accepts true or false (default).
			fn store() -> Option<bool> { None }
		}

		impl <T> ElasticFieldMapping<$f> for T where
		T: $m {
			type FieldSerType = Field<T, $f>;

			fn data_type() -> &'static str { $cn }
		}

		impl <T> Serialize for Field<T, $f> where
		T: ElasticFieldMapping<$f> + $m {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
			S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 8));

				try!(serializer.serialize_struct_elt(&mut state, "type", T::data_type()));

				ser_field!(serializer, &mut state, "coerce", T::coerce());
				ser_field!(serializer, &mut state, "boost", T::boost());
				ser_field!(serializer, &mut state, "doc_values", T::doc_values());
				ser_field!(serializer, &mut state, "ignore_malformed", T::ignore_malformed());
				ser_field!(serializer, &mut state, "include_in_all", T::include_in_all());
				ser_field!(serializer, &mut state, "null_value", T::null_value());
				ser_field!(serializer, &mut state, "store", T::store());

				serializer.serialize_struct_end(state)
			}
		}
	)
}

number_mapping!(IntegerMapping, IntegerFormat, INTEGER_DATATYPE, i32);
number_mapping!(LongMapping, LongFormat, LONG_DATATYPE, i64);
number_mapping!(ShortMapping, ShortFormat, SHORT_DATATYPE, i16);
number_mapping!(ByteMapping, ByteFormat, BYTE_DATATYPE, i8);
number_mapping!(FloatMapping, FloatFormat, FLOAT_DATATYPE, f32);
number_mapping!(DoubleMapping, DoubleFormat, DOUBLE_DATATYPE, f64);

/// Default mapping for an `integer` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl IntegerMapping for DefaultIntegerMapping {}
impl ElasticType<DefaultIntegerMapping, IntegerFormat> for i32 {}

/// Default mapping for a `long` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl LongMapping for DefaultLongMapping {}
impl ElasticType<DefaultLongMapping, LongFormat> for i64 {}
impl ElasticType<DefaultLongMapping, LongFormat> for isize {}

/// Default mapping for a `short` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl ShortMapping for DefaultShortMapping {}
impl ElasticType<DefaultShortMapping, ShortFormat> for i16 {}

/// Default mapping for a `byte` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl ByteMapping for DefaultByteMapping {}
impl ElasticType<DefaultByteMapping, ByteFormat> for i8 {}

/// Default mapping for a `float` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl FloatMapping for DefaultFloatMapping {}
impl ElasticType<DefaultFloatMapping, FloatFormat> for f32 {}

/// Default mapping for a `double` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl DoubleMapping for DefaultDoubleMapping {}
impl ElasticType<DefaultDoubleMapping, DoubleFormat> for f64 {}
