//! Mapping for the Elasticsearch `date` type.

use std::marker::PhantomData;
use serde;
use serde::Serialize;
use super::{ DateFormat, ElasticDate };
use ::mapping::ElasticFieldMapping;

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
/// 	"format": "epoch_millis",
/// 	"boost": 1.5
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
	fn boost() -> Option<f32> { None }

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> { None }

	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> { None }

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<bool> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }

	/// The date format(s) that can be parsed.
	fn format() -> &'static str { F::name() }

	/// If `true`, malformed numbers are ignored.
	/// If `false` (default), malformed numbers throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> { None }

	/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<ElasticDate<F, Self>> { None }
}

/// Implement `serde` serialisation for a `date` mapping type.
macro_rules! date_ser {
    ($t:ident: $f:ident) => (
		impl <$f: $crate::date::DateFormat> serde::Serialize for $t<$f> {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 8));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::<$f>::data_type()));
				try!(serializer.serialize_struct_elt(&mut state, "format", $t::<$f>::format()));

				ser_field!(serializer, &mut state, $t::<$f>::boost(), "boost");
				ser_field!(serializer, &mut state, $t::<$f>::doc_values(), "doc_values");
				ser_field!(serializer, &mut state, $t::<$f>::include_in_all(), "include_in_all");
				ser_field!(serializer, &mut state, $t::<$f>::index(), "index");
				ser_field!(serializer, &mut state, $t::<$f>::ignore_malformed(), "ignore_malformed");
				ser_field!(serializer, &mut state, $t::<$f>::null_value(), "null_value");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define a `date` mapping for all formats.
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// date_mapping!(MyMapping {
/// 	fn boost() -> Option<f32> { Some(1.03) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping<F: DateFormat>` and `ElasticDateMapping<F: DateFormat>`, along with a few default traits:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping<F: DateFormat> {
/// 	_marker: PhantomData<F>
/// }
/// ```
/// 
/// If you want to control the default implementations yourself, you can define your
/// mapping type and just pass it the macro to implement `ElasticFieldMapping<F>`:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping<F: 'static + DateFormat> {
/// 	_marker: PhantomData<F>
/// }
/// impl <F: 'static + DateFormat> ElasticDateMapping<F> for MyMapping<F> { }
/// date_mapping_all!(MyMapping: F);
/// ```
macro_rules! date_mapping {
	($t:ident: $f:ident) => (
		impl <$f: 'static + $crate::date::DateFormat> $crate::mapping::ElasticFieldMapping<F> for $t<$f> {
			fn data_type() -> &'static str { $crate::date::mapping::DATE_DATATYPE }
		}

		date_ser!($t: $f);
	);
	($t:ident $b:tt) => (
		#[derive(Debug, Default, Clone, Copy)]
		pub struct $t<F: 'static + $crate::date::DateFormat> {
			_marker: PhantomData<F>
		}

		impl <F: 'static + $crate::date::DateFormat> $crate::mapping::ElasticFieldMapping<F> for $t<F> {
			fn data_type() -> &'static str { $crate::date::mapping::DATE_DATATYPE }
		}

		impl <F: 'static + $crate::date::DateFormat> $crate::date::mapping::ElasticDateMapping<F> for $t<F> $b

		date_ser!($t: F);
	)
}

/// Implement `DateFormat` for the given type.
/// 
/// The macro takes 2 string literals; the format to parse and the name to use in Elasticsearch:
/// 
/// ```
/// struct MyFormat;
/// impl_date_fmt!(MyFormat, "yyyy-MM-ddTHH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ssZ");
/// ```
macro_rules! impl_date_fmt {
	($t:ty, $f:tt, $n:expr) => (
		impl $crate::date::DateFormat for $t {
			fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
				date_fmt!($f)
			}

			fn name() -> &'static str { $n }
		}
	)
}

/// Default mapping for `date`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultDateMapping<F: 'static + DateFormat> {
	_marker: PhantomData<F>
}
impl <F: 'static + DateFormat> ElasticDateMapping<F> for DefaultDateMapping<F> { }

date_mapping!(DefaultDateMapping: F);