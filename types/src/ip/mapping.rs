//! Mapping for the Elasticsearch `ip` type.

use std::marker::PhantomData;
use std::net::Ipv4Addr;
use serde;
use serde::Serialize;
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };

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

/// Default mapping for `ip`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultIpMapping;
impl ElasticIpMapping for DefaultIpMapping { }

impl_ip_mapping!(DefaultIpMapping);

/// Base visitor for serialising ip mappings.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticIpMappingVisitor<M> where M: ElasticIpMapping {
	phantom: PhantomData<M>
}

impl <M> ElasticTypeVisitor for ElasticIpMappingVisitor<M> where
M: ElasticIpMapping {
	fn new() -> Self {
		ElasticIpMappingVisitor {
			phantom: PhantomData
		}
	}
}
impl <M> serde::ser::MapVisitor for ElasticIpMappingVisitor<M> where
M: ElasticIpMapping {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

		ser_field!(serializer, M::boost(), "boost");
		ser_field!(serializer, M::doc_values(), "doc_values");
		ser_field!(serializer, M::index(), "index");
		ser_field!(serializer, M::store(), "store");
		ser_field!(serializer, M::null_value(), "null_value");

		Ok(None)
	}
}
