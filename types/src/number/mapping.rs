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
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! #[derive(Default)]
//! struct MyIntegerMapping;
//! impl IntegerMapping for MyIntegerMapping {
//!     //Overload the mapping functions here
//!     fn null_value() -> Option<i32> {
//!         Some(42)
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! This will produce the following mapping:
//!
//! ```
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # #[cfg(feature = "nightly")]
//! # extern crate serde_json;
//! # use elastic_types::prelude::*;
//! # #[derive(Default)]
//! # struct MyIntegerMapping;
//! # impl IntegerMapping for MyIntegerMapping {
//! #   //Overload the mapping functions here
//! #   fn null_value() -> Option<i32> {
//! #       Some(42)
//! #   }
//! # }
//! # fn main() {
//! # let json = json_str!(
//! {
//!     "type": "integer",
//!     "null_value": 42
//! }
//! # );
//! # #[cfg(feature = "nightly")]
//! # let mapping = serde_json::to_string(&Field::from(MyIntegerMapping)).unwrap();
//! # #[cfg(not(feature = "nightly"))]
//! # let mapping = json.clone();
//! # assert_eq!(json, mapping);
//! # }
//! ```

use serde::Serialize;
use serde::ser::SerializeStruct;
use private::field::{FieldMapping, SerializeField};
use document::{Field, FieldType};

macro_rules! number_mapping {
    ($mapping:ident, $format:ident, $field_trait:ident, $datatype_name:expr, $std_ty:ty) => (
        #[derive(Default)]
        struct $format;

        /// A field that will be mapped as a number.
        pub trait $field_trait<M> where M: $mapping {}

        impl<T, M> FieldType<M, $format> for T
            where M: $mapping,
                T: $field_trait<M> + Serialize
        {
        }

        /// Base `number` mapping.
        pub trait $mapping where
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
            fn null_value() -> Option<$std_ty> { None }

            /// Whether the field value should be stored and retrievable separately from the `_source` field.
            /// Accepts true or false (default).
            fn store() -> Option<bool> { None }
        }

        impl <T> FieldMapping<$format> for T 
            where T: $mapping 
        {
            fn data_type() -> &'static str { $datatype_name }
        }

        impl<T> SerializeField<$format> for T
            where T: $mapping
        {
            type Field = Field<T, $format>;
        }

        impl <T> Serialize for Field<T, $format> where
        T: FieldMapping<$format> + $mapping {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: ::serde::Serializer {
                let mut state = try!(serializer.serialize_struct("mapping", 8));

                try!(state.serialize_field("type", &T::data_type()));

                ser_field!(state, "coerce", T::coerce());
                ser_field!(state, "boost", T::boost());
                ser_field!(state, "doc_values", T::doc_values());
                ser_field!(state, "ignore_malformed", T::ignore_malformed());
                ser_field!(state, "include_in_all", T::include_in_all());
                ser_field!(state, "null_value", T::null_value());
                ser_field!(state, "store", T::store());

                state.end()
            }
        }
    )
}

number_mapping!(IntegerMapping, IntegerFormat, IntegerFieldType, "integer", i32);
number_mapping!(LongMapping, LongFormat, LongFieldType, "long", i64);
number_mapping!(ShortMapping, ShortFormat, ShortFieldType, "short", i16);
number_mapping!(ByteMapping, ByteFormat, ByteFieldType, "byte", i8);
number_mapping!(FloatMapping, FloatFormat, FloatFieldType, "float", f32);
number_mapping!(DoubleMapping, DoubleFormat, DoubleFieldType, "double", f64);

/// Default mapping for an `integer` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl IntegerMapping for DefaultIntegerMapping {}
impl IntegerFieldType<DefaultIntegerMapping> for i32 {}

/// Default mapping for a `long` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl LongMapping for DefaultLongMapping {}
impl LongFieldType<DefaultLongMapping> for i64 {}
impl LongFieldType<DefaultLongMapping> for isize {}

/// Default mapping for a `short` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl ShortMapping for DefaultShortMapping {}
impl ShortFieldType<DefaultShortMapping> for i16 {}

/// Default mapping for a `byte` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl ByteMapping for DefaultByteMapping {}
impl ByteFieldType<DefaultByteMapping> for i8 {}

/// Default mapping for a `float` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl FloatMapping for DefaultFloatMapping {}
impl FloatFieldType<DefaultFloatMapping> for f32 {}

/// Default mapping for a `double` type.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl DoubleMapping for DefaultDoubleMapping {}
impl DoubleFieldType<DefaultDoubleMapping> for f64 {}
