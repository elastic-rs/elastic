//! Helper mappers for `ElasticFieldType`.
//!
//! Mapping for types is inferred from the generic mapping parameters on `ElasticFieldType`.
//!
//! `TypeMapper`, for mapping user-defined types for the [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html).
//!
//! `FieldMapper`, for mapping any type as a field of a user-defined type.
//!
//! # Examples
//!
//! Any type that derives `ElasticFieldType` can be mapped using one of the various mappers.
//!
//! ## Mapping to a json string
//!
//! ```
//! # #![feature(proc_macro)]
//! # #[macro_use]
//! # extern crate json_str;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_types_derive;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use elastic_types::prelude::*;
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: Date<DefaultDateFormat>,
//! # 	pub my_string: String,
//! # 	pub my_num: i32
//! # }
//! # fn main() {
//! let ser = TypeMapper::to_string(MyTypeMapping).unwrap();
//! # }
//! ```

use std::error::Error;
use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use serde::ser::Error as SerError;
use serde_json::{Error as JsonError, Serializer as JsonSerializer, Value};
use serde_json::value::Serializer as ValueSerializer;
use ::mapping::{ElasticFieldMapping, ObjectMapping};

/// Helper for mapping field types.
pub struct FieldMapper<M, F>
    where M: ElasticFieldMapping<F>,
          F: Default
{
    _m: PhantomData<M>,
    _f: PhantomData<F>,
}
impl<M, F> FieldMapper<M, F>
    where M: ElasticFieldMapping<F>,
          F: Default
{
    /// Map a field type with a given `Serializer`.
    pub fn to_writer<S>(_: M, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        M::ser().serialize(serializer)
    }

    /// Map a field type to a `String`.
    pub fn to_string(t: M) -> Result<String, JsonError> {
        let mut writer = Vec::new();
        {
            let mut ser = JsonSerializer::new(&mut writer);
            try!(Self::to_writer(t, &mut ser));
        }

        String::from_utf8(writer).map_err(|e| JsonError::custom(e.description()))
    }

    /// Map a field type to a `serde_json::Value`.
    pub fn to_value(t: M) -> Result<Value, JsonError> {
        let mut ser = ValueSerializer::new();
        try!(Self::to_writer(t, &mut ser));

        Ok(ser.unwrap())
    }
}

/// Helper for mapping user-defined types.
///
/// This mapper is designed to take a given user-defined type and pass it around to various visitors to map fields.
pub struct TypeMapper<M>
    where M: ObjectMapping
{
    _m: PhantomData<M>,
}
impl<M> TypeMapper<M>
    where M: ObjectMapping
{
    /// Map a user-defined type with a given `Serializer`.
    ///
    /// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(proc_macro)]
    /// # #[macro_use]
    /// # extern crate json_str;
    /// # #[macro_use]
    /// # extern crate serde_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types;
    /// # extern crate serde;
    /// # extern crate serde_json;
    /// # use elastic_types::prelude::*;
    /// # #[derive(Serialize, Deserialize, ElasticType)]
    /// # pub struct MyType {
    /// # 	pub my_date: Date<DefaultDateFormat>,
    /// # 	pub my_string: String,
    /// # 	pub my_num: i32
    /// # }
    /// # fn main() {
    /// let mut writer = Vec::new();
    /// let mut ser = serde_json::Serializer::new(&mut writer);
    /// let ser = TypeMapper::to_writer(MyTypeMapping, &mut ser).unwrap();
    /// # }
    /// ```
    pub fn to_writer<S>(_: M, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        M::serialize_type(serializer)
    }

    /// Map a user-defined type to a `String`.
    ///
    /// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(proc_macro)]
    /// # #[macro_use]
    /// # extern crate json_str;
    /// # #[macro_use]
    /// # extern crate serde_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types;
    /// # extern crate serde;
    /// # use elastic_types::prelude::*;
    /// # #[derive(Serialize, Deserialize, ElasticType)]
    /// # pub struct MyType {
    /// # 	pub my_date: Date<DefaultDateFormat>,
    /// # 	pub my_string: String,
    /// # 	pub my_num: i32
    /// # }
    /// # fn main() {
    /// let ser = TypeMapper::to_string(MyTypeMapping).unwrap();
    /// # }
    /// ```
    pub fn to_string(t: M) -> Result<String, JsonError> {
        let mut writer = Vec::new();
        {
            let mut ser = JsonSerializer::new(&mut writer);
            try!(Self::to_writer(t, &mut ser));
        }

        String::from_utf8(writer).map_err(|e| JsonError::custom(e.description()))
    }

    /// Map a user-defined type to a `serde_json::Value`.
    ///
    /// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(proc_macro)]
    /// # #[macro_use]
    /// # extern crate json_str;
    /// # #[macro_use]
    /// # extern crate serde_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types_derive;
    /// # #[macro_use]
    /// # extern crate elastic_types;
    /// # extern crate serde;
    /// # use elastic_types::prelude::*;
    /// # #[derive(Serialize, Deserialize, ElasticType)]
    /// # pub struct MyType {
    /// # 	pub my_date: Date<DefaultDateFormat>,
    /// # 	pub my_string: String,
    /// # 	pub my_num: i32
    /// # }
    /// # fn main() {
    /// let val = TypeMapper::to_value(MyTypeMapping).unwrap();
    ///
    /// let ty = val.lookup("properties.my_date.type");
    /// # }
    /// ```
    pub fn to_value(t: M) -> Result<Value, JsonError> {
        let mut ser = ValueSerializer::new();
        try!(Self::to_writer(t, &mut ser));

        Ok(ser.unwrap())
    }
}
