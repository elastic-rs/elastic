//! Mapping for the Elasticsearch `ip` type.

use std::net::Ipv4Addr;
use serde;
use serde::Serialize;
use ::mapping::ElasticFieldMapping;

/// Elasticsearch datatype name.
pub const IP_DATATYPE: &'static str = "ip";

/// The base requirements for mapping a `ip` type.
///
/// Custom mappings can be defined by implementing `ElasticIpMapping`.
///
/// # Examples
///
/// Define a custom `ElasticIpMapping`:
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
/// use elastic_types::ip::prelude::*;
///
/// #[derive(Debug, Clone, Default, ElasticIpMapping)]
/// pub struct MyIpMapping;
/// impl ElasticIpMapping for MyIpMapping {
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
/// # #[derive(Debug, Clone, Default, ElasticIpMapping)]
/// # pub struct MyIpMapping;
/// # impl ElasticIpMapping for MyIpMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyIpMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "ip",
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
/// use elastic_types::ip::prelude::*;
///
/// #[derive(Debug, Clone, Default)]
/// pub struct MyIpMapping;
/// impl ElasticIpMapping for MyIpMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticFieldMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticFieldMapping<()> for MyIpMapping {
/// 	type Visitor = ElasticIpMappingVisitor<MyIpMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		IP_DATATYPE
/// 	}
/// }
///
/// impl serde::Serialize for MyIpMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticIpMapping where
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
	fn null_value() -> Option<Ipv4Addr> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }
}

/// Implement `serde` serialisation for a `geo_shape` mapping type.
#[macro_export]
macro_rules! ip_ser {
    ($t:ident) => (
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer {
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

/// Define an `ip` mapping.
/// 
/// # Examples
/// 
/// ## Define mapping struct inline
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// ip_mapping!(MyMapping {
///     fn boost() -> Option<f32> { Some(1.03) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping` and `ElasticIpMapping`, along with a few default traits:
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
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// impl ElasticIpMapping for MyMapping { 
///     fn boost() -> Option<f32> { Some(1.03) }
/// }
/// 
/// ip_mapping!(MyMapping);
/// ```
#[macro_export]
macro_rules! ip_mapping {
    ($t:ident) => (
        impl $crate::mapping::ElasticFieldMapping<()> for $t {
            fn data_type() -> &'static str { $crate::ip::mapping::IP_DATATYPE }
        }

        ip_ser!($t);
    );
    ($t:ident $b:tt) => (
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $t;

        impl $crate::ip::mapping::ElasticIpMapping for $t $b

        ip_mapping!($t);
    )
}

/// Default mapping for `geo_shape`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultIpMapping;
impl ElasticIpMapping for DefaultIpMapping { }

ip_mapping!(DefaultIpMapping);