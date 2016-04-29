//! Mapping for the Elasticsearch `number` types.
//!
//! Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
//! The implementation is the same for all number types, the only difference is the return type of `null_value`.
//!
//! # Examples
//!
//! Define a custom `ElasticIntegerMapping`:
//!
//! ## Derive Mapping
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::number::prelude::*;
//!
//! #[derive(Debug, Clone, Default, ElasticIntegerMapping)]
//! pub struct MyIntegerMapping;
//! impl ElasticIntegerMapping for MyIntegerMapping {
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
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # extern crate serde_json;
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::number::prelude::*;
//! # #[derive(Debug, Clone, Default, ElasticIntegerMapping)]
//! # pub struct MyIntegerMapping;
//! # impl ElasticIntegerMapping for MyIntegerMapping {
//! # 	//Overload the mapping functions here
//! # 	fn null_value() -> Option<i32> {
//! # 		Some(42)
//! # 	}
//! # }
//! # fn main() {
//! # let mapping = serde_json::to_string(&MyIntegerMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "type": "integer",
//! 	"null_value": 42
//! }
//! # );
//! # assert_eq!(json, &mapping);
//! # }
//! ```
//!
//! ## Manually
//!
//! ```
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::number::prelude::*;
//!
//! #[derive(Debug, Clone, Default)]
//! pub struct MyIntegerMapping;
//! impl ElasticIntegerMapping for MyIntegerMapping {
//! 	//Overload the mapping functions here
//! 	fn null_value() -> Option<i32> {
//! 		Some(42)
//! 	}
//! }
//!
//! impl ElasticTypeMapping<()> for MyIntegerMapping {
//! 	type Visitor = ElasticIntegerMappingVisitor<MyIntegerMapping>;
//! 	fn data_type() -> &'static str {
//! 		"integer"
//! 	}
//! }
//!
//! impl serde::Serialize for MyIntegerMapping {
//! 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! 	where S: serde::Serializer {
//! 		serializer.serialize_struct("mapping", Self::get_visitor())
//! 	}
//! }
//! # }
//! ```

use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use ::mapping::{ ElasticType, ElasticTypeMapping, ElasticTypeVisitor, IndexAnalysis };

macro_rules! number_mapping {
    ($m:ident, $v:ident, $n:ty) => (
    	/// Base `number` mapping.
    	pub trait $m
		where Self : ElasticTypeMapping<()> + Sized + Serialize {
			/// Try to convert strings to numbers and truncate fractions for integers. Accepts `true` (default) and `false`.
			fn coerce() -> Option<bool> {
				None
			}

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

			/// If `true`, malformed numbers are ignored. If `false` (default),
			/// malformed numbers throw an exception and reject the whole document.
			fn ignore_malformed() -> Option<bool> {
				None
			}

			/// Whether or not the field value should be included in the `_all` field?
			/// Accepts `true` or `false`. Defaults to false if index is set to no,
			/// or if a parent object field sets `include_in_all` to false.
			/// Otherwise defaults to `true`.
			fn include_in_all() -> Option<bool> {
				None
			}

			/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
			fn index() -> Option<IndexAnalysis> {
				None
			}

			/// Accepts a numeric value of the same type as the field which is substituted for any explicit null values.
			/// Defaults to `null`, which means the field is treated as missing.
			fn null_value() -> Option<$n> {
				None
			}

			/// Controls the number of extra terms that are indexed to make range queries faster.
			/// The default depends on the numeric type.
			fn precision_step() -> Option<u32> {
				None
			}

			/// Whether the field value should be stored and retrievable separately from the `_source` field.
			/// Accepts true or false (default).
			fn store() -> Option<bool> {
				None
			}
		}

		/// Visitor for a `number` field mapping.
		#[derive(Debug, PartialEq)]
		pub struct $v<T> where T: $m {
			phantom: PhantomData<T>
		}

        impl <T> ElasticTypeVisitor for $v<T> where T: $m {
            fn new() -> Self {
        		$v {
                    phantom: PhantomData
                }
        	}
        }
		impl <T> serde::ser::MapVisitor for $v<T> where T: $m {
			fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
			where S: Serializer {
				try!(serializer.serialize_struct_elt("type", T::data_type()));

				if let Some(coerce) = T::coerce() {
					try!(serializer.serialize_struct_elt("coerce", coerce));
				}

				if let Some(boost) = T::boost() {
					try!(serializer.serialize_struct_elt("boost", boost));
				}

				if let Some(doc_values) = T::doc_values() {
					try!(serializer.serialize_struct_elt("doc_values", doc_values));
				}

				if let Some(ignore_malformed) = T::ignore_malformed() {
					try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed));
				}

				if let Some(include_in_all) = T::include_in_all() {
					try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
				}

				if let Some(null_value) = T::null_value() {
					try!(serializer.serialize_struct_elt("null_value", null_value));
				}

				if let Some(precision_step) = T::precision_step() {
					try!(serializer.serialize_struct_elt("precision_step", precision_step));
				}

				if let Some(store) = T::store() {
					try!(serializer.serialize_struct_elt("store", store));
				}

				Ok(None)
			}
		}
    )
}

/// Base mapping requirements for an `integer`.
number_mapping!(ElasticIntegerMapping, ElasticIntegerMappingVisitor, i32);

/// Base mapping requirements for an `long`.
number_mapping!(ElasticLongMapping, ElasticLongMappingVisitor, i64);

/// Base mapping requirements for an `short`.
number_mapping!(ElasticShortMapping, ElasticShortMappingVisitor, i16);

/// Base mapping requirements for an `byte`.
number_mapping!(ElasticByteMapping, ElasticByteMappingVisitor, i8);

/// Base mapping requirements for an `float`.
number_mapping!(ElasticFloatMapping, ElasticFloatMappingVisitor, f32);

/// Base mapping requirements for an `double`.
number_mapping!(ElasticDoubleMapping, ElasticDoubleMappingVisitor, f64);

/// Default mapping for an `integer` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl ElasticIntegerMapping for DefaultIntegerMapping { }
impl_integer_mapping!(DefaultIntegerMapping);
impl ElasticType<DefaultIntegerMapping, ()> for i32 { }

/// Default mapping for a `long` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl ElasticLongMapping for DefaultLongMapping { }
impl_long_mapping!(DefaultLongMapping);
impl ElasticType<DefaultLongMapping, ()> for i64 { }
impl ElasticType<DefaultLongMapping, ()> for isize { }

/// Default mapping for a `short` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl ElasticShortMapping for DefaultShortMapping { }
impl_short_mapping!(DefaultShortMapping);
impl ElasticType<DefaultShortMapping, ()> for i16 { }

/// Default mapping for a `byte` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl ElasticByteMapping for DefaultByteMapping { }
impl_byte_mapping!(DefaultByteMapping);
impl ElasticType<DefaultByteMapping, ()> for i8 { }

/// Default mapping for a `float` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl ElasticFloatMapping for DefaultFloatMapping { }
impl_float_mapping!(DefaultFloatMapping);
impl ElasticType<DefaultFloatMapping, ()> for f32 { }

/// Default mapping for a `double` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl ElasticDoubleMapping for DefaultDoubleMapping { }
impl_double_mapping!(DefaultDoubleMapping);
impl ElasticType<DefaultDoubleMapping, ()> for f64 { }
