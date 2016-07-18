//! Mapping for the Elasticsearch `date` type.

use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use super::{ DateFormat, ElasticDate };
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor, IndexAnalysis };

/// Elasticsearch datatype name.
pub const DATE_DATATYPE: &'static str = "date";

/// The base requirements for mapping a `date` type.
///
/// # Examples
///
/// Define a custom `ElasticDateMapping`:
///
/// ## Derive Mapping
///
/// Currently, deriving mapping only works for structs that take a generic `DateFormat` parameter.
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use std::marker::PhantomData;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::date::prelude::*;
///
/// #[derive(Default, Clone, Copy, ElasticDateMapping)]
/// pub struct MyDateMapping<F: DateFormat> {
/// 	phantom: PhantomData<F>
/// }
/// impl <F: DateFormat> ElasticDateMapping<F> for MyDateMapping<F> {
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
/// # use std::marker::PhantomData;
/// # use elastic_types::mapping::prelude::*;
/// # use elastic_types::date::prelude::*;
/// # #[derive(Default, Clone, Copy, ElasticDateMapping)]
/// # pub struct MyDateMapping<F: DateFormat = EpochMillis> {
/// # 	phantom: PhantomData<F>
/// # }
/// # impl <F: DateFormat> ElasticDateMapping<F> for MyDateMapping<F> {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyDateMapping::<EpochMillis>::default()).unwrap();
/// # let json = json_str!(
/// {
///     "type": "date",
/// 	"boost": 1.5,
/// 	"format": "epoch_millis"
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Limitations
///
/// Automatically deriving mapping has the following limitations:
///
/// - Non-generic mappings aren't supported by auto deriving.
/// So your date mapping must take generic parameter `<F: DateFormat>`.
///
/// The above limitation can be worked around by implementing the mapping manually.
///
/// ## Manually
///
/// Define a date mapping that's only valid for the `EpochMillis` format:
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # use std::marker::PhantomData;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::date::prelude::*;
///
/// #[derive(Default, Clone)]
/// pub struct MyDateMapping;
///
/// impl ElasticFieldMapping<EpochMillis> for MyDateMapping {
/// 	type Visitor = ElasticDateMappingVisitor<EpochMillis, MyDateMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		DATE_DATATYPE
/// 	}
/// }
///
/// impl ElasticDateMapping<EpochMillis> for MyDateMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// impl serde::Serialize for MyDateMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
///
/// Define a date mapping that's valid for any `DateFormat` (equivalent to the auto derive example):
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # use std::marker::PhantomData;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::date::prelude::*;
///
/// #[derive(Default, Clone)]
/// pub struct MyDateMapping<F: DateFormat> {
/// 	phantom: PhantomData<F>
/// }
///
/// impl <F: DateFormat> ElasticFieldMapping<F> for MyDateMapping<F> {
/// 	type Visitor = ElasticDateMappingVisitor<F, MyDateMapping<F>>;
///
/// 	fn data_type() -> &'static str {
/// 		DATE_DATATYPE
/// 	}
/// }
///
/// impl <F: DateFormat> ElasticDateMapping<F> for MyDateMapping<F> {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// impl <F: DateFormat> serde::Serialize for MyDateMapping<F> {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticDateMapping<F> where
F: DateFormat,
Self: ElasticFieldMapping<F> + Sized + Serialize {
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

	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<IndexAnalysis> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> {
		None
	}

	/// The date format(s) that can be parsed.
	fn format() -> &'static str {
		F::name()
	}

	/// If `true`, malformed numbers are ignored.
	/// If `false` (default), malformed numbers throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> {
		None
	}

	/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<ElasticDate<F, Self>> {
		None
	}

	/// Controls the number of extra terms that are indexed to make range queries faster. Defaults to 16.
	fn precision_step() -> Option<i32> {
		None
	}
}

/// Default mapping for `ElasticDate`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultDateMapping<F> where
F: DateFormat {
	phantom: PhantomData<F>
}
impl <F> ElasticDateMapping<F> for DefaultDateMapping<F> where
F: DateFormat { }

impl_date_mapping!(DefaultDateMapping<F>);

/// Visitor for a `date` map.
#[derive(Debug, PartialEq)]
pub struct ElasticDateMappingVisitor<F, M> where
F: DateFormat,
M: ElasticDateMapping<F> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<M>
}

impl <F, M> ElasticTypeVisitor for ElasticDateMappingVisitor<F, M> where
F: DateFormat,
M: ElasticDateMapping<F> {
	fn new() -> Self {
		ElasticDateMappingVisitor {
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}
}
impl <F, M> serde::ser::MapVisitor for ElasticDateMappingVisitor<F, M>  where
F: DateFormat,
M: ElasticDateMapping<F> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

		if let Some(boost) = M::boost() {
			try!(serializer.serialize_struct_elt("boost", boost));
		};

		if let Some(doc_values) = M::doc_values() {
			try!(serializer.serialize_struct_elt("doc_values", doc_values));
		};

		if let Some(include_in_all) = M::include_in_all() {
			try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
		};

		if let Some(index) = M::index() {
			try!(serializer.serialize_struct_elt("index", index));
		};

		if let Some(store) = M::store() {
			try!(serializer.serialize_struct_elt("store", store));
		};

		try!(serializer.serialize_struct_elt("format", M::format()));

		if let Some(ignore_malformed) = M::ignore_malformed() {
			try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed));
		};

		if let Some(null_value) = M::null_value() {
			try!(serializer.serialize_struct_elt("null_value", null_value));
		};

		if let Some(precision_step) = M::precision_step() {
			try!(serializer.serialize_struct_elt("precision_step", precision_step));
		};

		Ok(None)
	}
}
