//! Mapping for Elasticsearch document types.

use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use super::{DocumentType, FieldType, Field, Document};
use private::field::{FieldMapping, SerializeField};

/// A field that will be mapped as a nested document.
pub trait DocumentFieldType<M> where M: DocumentMapping {}

impl<T, M> FieldType<M, DocumentFormat> for T
    where M: DocumentMapping,
          T: DocumentFieldType<M> + Serialize
{
}

impl<T, M> DocumentFieldType<M> for T
    where T: DocumentType<M>,
          M: DocumentMapping
{
}

/// Elasticsearch datatype name.
pub const OBJECT_DATATYPE: &'static str = "object";
/// Elasticsearch datatype name.
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";
/// Elasticsearch datatype name.
pub const NESTED_DATATYPE: &'static str = "nested";

#[derive(Default)]
struct DocumentFormat;

/// The base requirements for mapping an `object` type.
pub trait DocumentMapping
    where Self: PropertiesMapping + Default
{
    /// Get the indexed name for this mapping.
    fn name() -> &'static str;

    /// Get the type name for this mapping, like `object` or `nested`.
    fn data_type() -> &'static str {
        NESTED_DATATYPE
    }

    /// Whether or not new properties should be added dynamically to an existing object.
    /// Accepts `true` (default), `false` and `strict`.
    fn dynamic() -> Option<Dynamic> {
        None
    }

    /// Whether the JSON value given for the object field should be parsed and indexed
    /// (`true`, default) or completely ignored (`false`).
    fn enabled() -> Option<bool> {
        None
    }

    /// Sets the default `include_in_all` value for all the properties within the object.
    /// The object itself is not added to the `_all` field.
    fn include_in_all() -> Option<bool> {
        None
    }
}

/// Serialisation for the mapping of object properties.
///
/// This trait is designed to be auto-derived, so it expects you to be familiar with how `serde` works.
///
/// # Examples
///
/// Say we have a mappable type with 3 fields called `MyType` and a mapping type called `MyTypeMapping`:
///
/// ```
/// # use elastic_types::prelude::*;
/// struct MyType {
///     pub my_date: Date<DefaultDateFormat>,
///     pub my_string: String,
///     pub my_num: i32
/// }
///
/// #[derive(Default)]
/// struct MyTypeMapping;
/// ```
///
/// To serialise the mapping of each of `MyType`s fields, we implement `PropertiesMapping` for `MyTypeMapping`,
/// and use `serde` to serialise the mapping types for each field.
///
/// ```
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use serde::ser::SerializeStruct;
/// # use elastic_types::prelude::*;
/// # pub struct MyTypeMapping;
/// impl PropertiesMapping for MyTypeMapping {
///     fn props_len() -> usize { 3 }
///
///     fn serialize_props<S>(state: &mut S) -> Result<(), S::Error>
///     where S: SerializeStruct {
///         try!(field_ser(state, "my_date", Date::<DefaultDateFormat>::mapping()));
///         try!(field_ser(state, "my_string", String::mapping()));
///         try!(field_ser(state, "my_num", i32::mapping()));
///
///         Ok(())
///     }
/// }
/// # fn main() {
/// # }
/// ```
///
/// It's easy to get an instance of the mapping for a given type by calling the static `mapping` function.
/// This trait is automatically implemented for you when you `#[derive(ElasticType)]`.
pub trait PropertiesMapping {
    /// The number of mapped property fields for this type.
    ///
    /// This number should be the same as the number of fields being serialised by `serialize_props`.
    fn props_len() -> usize;

    /// Serialisation for the mapped property fields on this type.
    ///
    /// You can use the `field_ser` function to simplify `serde` calls.
    fn serialize_props<S>(state: &mut S) -> Result<(), S::Error> where S: SerializeStruct;
}

impl<T> FieldMapping<DocumentFormat> for T
    where T: DocumentMapping
{
    fn data_type() -> &'static str {
        <Self as DocumentMapping>::data_type()
    }
}

impl<T> SerializeField<DocumentFormat> for T
    where T: DocumentMapping
{
    type Field = Field<T, DocumentFormat>;
}

impl<T> Serialize for Field<T, DocumentFormat>
    where T: FieldMapping<DocumentFormat> + DocumentMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 5));

        let ty = <T as DocumentMapping>::data_type();
        try!(state.serialize_field("type", ty));

        ser_field!(state, "dynamic", T::dynamic());
        ser_field!(state, "include_in_all", T::include_in_all());

        if ty == OBJECT_DATATYPE {
            ser_field!(state, "enabled", T::enabled());
        }

        try!(state.serialize_field("properties", &Properties::<T>::default()));

        state.end()
    }
}

impl<M> Serialize for Document<M>
    where M: DocumentMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 1));

        try!(state.serialize_field("properties", &Properties::<M>::default()));

        state.end()
    }
}

#[derive(Default)]
struct Properties<M>
    where M: DocumentMapping
{
    _m: PhantomData<M>,
}

impl<M> Serialize for Properties<M>
    where M: DocumentMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("properties", M::props_len()));
        try!(M::serialize_props(&mut state));
        state.end()
    }
}

/// The dynamic setting may be set at the mapping type level, and on each inner object.
/// Inner objects inherit the setting from their parent object or from the mapping type.
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    /// Newly detected fields are added to the mapping. (default).
    True,
    /// Newly detected fields are ignored. New fields must be added explicitly.
    False,
    /// If new fields are detected, an exception is thrown and the document is rejected.
    Strict,
}

impl Serialize for Dynamic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Dynamic::True => serializer.serialize_bool(true),
            Dynamic::False => serializer.serialize_bool(false),
            Dynamic::Strict => serializer.serialize_str("strict"),
        }
    }
}
