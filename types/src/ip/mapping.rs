//! Mapping for the Elasticsearch `ip` type.

use std::net::Ipv4Addr;
use serde::{Serialize, Serializer};
use ::field::{FieldMapping, FieldSerWrapper};

/// Elasticsearch datatype name.
pub const IP_DATATYPE: &'static str = "ip";

#[doc(hidden)]
#[derive(Default)]
pub struct IpFormat;

/// The base requirements for mapping a `ip` type.
///
/// Custom mappings can be defined by implementing `IpMapping`.
///
/// # Examples
///
/// Define a custom `IpMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_derive)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// #[derive(Default)]
/// struct MyIpMapping;
/// impl IpMapping for MyIpMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
/// 			Some(1.5)
/// 		}
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_derive)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # #[derive(Default)]
/// # struct MyIpMapping;
/// # impl IpMapping for MyIpMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
/// 	# 		Some(1.5)
/// 	# 	}
/// # }
/// # fn main() {
/// # let mapping = FieldMapper::to_string(MyIpMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "ip",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
pub trait IpMapping
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
    fn null_value() -> Option<Ipv4Addr> {
        None
    }

    /// Whether the field value should be stored and retrievable separately from the `_source` field.
    /// Accepts `true` or `false` (default).
    fn store() -> Option<bool> {
        None
    }
}

impl<T> FieldMapping<IpFormat> for T
    where T: IpMapping
{
    type Field = FieldSerWrapper<T, IpFormat>;

    fn data_type() -> &'static str {
        IP_DATATYPE
    }
}

impl<T> Serialize for FieldSerWrapper<T, IpFormat>
    where T: FieldMapping<IpFormat> + IpMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 6));

        try!(serializer.serialize_struct_elt(&mut state, "type", T::data_type()));

        ser_field!(serializer, &mut state, "boost", T::boost());
        ser_field!(serializer, &mut state, "doc_values", T::doc_values());
        ser_field!(serializer, &mut state, "index", T::index());
        ser_field!(serializer, &mut state, "store", T::store());
        ser_field!(serializer, &mut state, "null_value", T::null_value());

        serializer.serialize_struct_end(state)
    }
}

/// Default mapping for `geo_shape`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIpMapping;
impl IpMapping for DefaultIpMapping {}
