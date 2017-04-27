//! Mapping for the Elasticsearch `boolean` type.

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use private::field::{FieldMapping, SerializeField};
use document::{Field, FieldType};

/// A field that will be mapped as a `boolean`.
pub trait BooleanFieldType<M> where M: BooleanMapping {}

impl<T, M> FieldType<M, BooleanFormat> for T
    where M: BooleanMapping,
          T: BooleanFieldType<M> + Serialize
{
}

#[derive(Default)]
struct BooleanFormat;

/// The base requirements for mapping a `boolean` type.
///
/// Custom mappings can be defined by implementing `BooleanMapping`.
///
/// # Examples
///
/// Define a custom `BooleanMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # use elastic_types::prelude::*;
/// # fn main() {
/// #[derive(Default)]
/// struct MyBooleanMapping;
/// impl BooleanMapping for MyBooleanMapping {
///     //Overload the mapping functions here
///     fn boost() -> Option<f32> {
///         Some(1.5)
///     }
/// }
/// # }
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # #[cfg(feature = "nightly")]
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # #[derive(Default)]
/// # struct MyBooleanMapping;
/// # impl BooleanMapping for MyBooleanMapping {
/// #     //Overload the mapping functions here
/// #     fn boost() -> Option<f32> {
/// #         Some(1.5)
/// #     }
/// # }
/// # fn main() {
/// # let json = json_str!(
/// {
///     "type": "boolean",
///     "boost": 1.5
/// }
/// # );
/// # #[cfg(feature = "nightly")]
/// # let mapping = serde_json::to_string(&Field::from(MyBooleanMapping)).unwrap();
/// # #[cfg(not(feature = "nightly"))]
/// # let mapping = json.clone();
/// # assert_eq!(json, mapping);
/// # }
/// ```
pub trait BooleanMapping
    where Self: Default
{
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
    fn index() -> Option<bool> {
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

impl<T> FieldMapping<BooleanFormat> for T
    where T: BooleanMapping
{
    fn data_type() -> &'static str {
        "boolean"
    }
}

impl<T> SerializeField<BooleanFormat> for T
    where T: BooleanMapping
{
    type Field = Field<T, BooleanFormat>;
}

impl<T> Serialize for Field<T, BooleanFormat>
    where T: FieldMapping<BooleanFormat> + BooleanMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 6));

        try!(state.serialize_field("type", T::data_type()));

        ser_field!(state, "boost", T::boost());
        ser_field!(state, "doc_values", T::doc_values());
        ser_field!(state, "index", T::index());
        ser_field!(state, "store", T::store());
        ser_field!(state, "null_value", T::null_value());

        state.end()
    }
}

/// Default mapping for `bool`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl BooleanMapping for DefaultBooleanMapping {}
