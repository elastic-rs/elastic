//! Mapping for the Elasticsearch `date` type.

use std::marker::PhantomData;
use chrono::{ Datelike, Timelike };
use serde;
use serde::{ Serializer, Serialize };
use super::{ Format, ParseError, DefaultFormat };
use ::mapping::{ ElasticMapping, ElasticType, IndexAnalysis };

/// The base requirements for mapping a `date` type.
/// 
/// # Examples
/// 
/// Custom mappings can be defined by implementing `ElasticDateMapping`:
/// 
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::ElasticMapping;
/// use elastic_types::date::mapping::ElasticDateMapping;
/// use elastic_types::date::{ BasicDateTime, Format };
/// 
/// struct MyDateMapping;
/// impl serde::Serialize for MyDateMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", MyDateMapping::get_visitor())
/// 	}
/// }
/// impl ElasticDateMapping<BasicDateTime> for MyDateMapping {
/// 	fn ignore_malformed() -> Option<bool> {
/// 		Some(true)
///		}
/// 
/// 	fn null_value() -> Option<&'static str> {
/// 		Some("20150701T000000.000Z")
/// 	}
/// }
/// # }
/// ```
/// 
/// The above example binds the mapping to the `BasicDateTime` format, so `get_null_value` returns a properly formated value.
pub trait ElasticDateMapping<T: Format>
where Self : Sized + Serialize {
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
		T::name()
	}

	/// If `true`, malformed numbers are ignored. 
	/// If `false` (default), malformed numbers throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> {
		None
	}

	/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	fn null_value() -> Option<&'static str> {
		None
	}

	/// Controls the number of extra terms that are indexed to make range queries faster. Defaults to 16.
	fn precision_step() -> Option<i32> {
		None
	}
}

impl <T: Format, M: ElasticDateMapping<T>> ElasticMapping<T> for M {
	type Visitor = ElasticDateMappingVisitor<T, M>;

	fn field_type() -> &'static str {
		"date"
	}
}

//TODO: Make this take in str for field name
/// Default mapping for `DateTime`.
#[derive(Debug, Clone)]
pub struct DefaultDateMapping<T: Format = DefaultFormat> {
	phantom: PhantomData<T>
}
impl <T: Format> DefaultDateMapping<T> {
	/// Get a new default mapping
	pub fn new() -> DefaultDateMapping<T> {
		DefaultDateMapping {
			phantom: PhantomData
		}
	}
}

impl <T: Format> ElasticDateMapping<T> for DefaultDateMapping<T> { }

impl <T: Format> serde::Serialize for DefaultDateMapping<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", ElasticDateMappingVisitor::<T, DefaultDateMapping<T>>::default())
	}
}

/// Visitor for a `date` map.
#[derive(Debug, PartialEq)]
pub struct ElasticDateMappingVisitor<F: Format, T: ElasticDateMapping<F>> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F: Format, T: ElasticDateMapping<F>> Default for ElasticDateMappingVisitor<F, T> {
	fn default() -> ElasticDateMappingVisitor<F, T> {
		ElasticDateMappingVisitor::<F, T> {
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}
}

impl <F: Format, T: ElasticDateMapping<F>> serde::ser::MapVisitor for ElasticDateMappingVisitor<F, T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", T::field_type()));

		match T::boost() {
			Some(boost) => try!(serializer.serialize_struct_elt("boost", boost)),
			None => ()
		};
		match T::doc_values() {
			Some(doc_values) => try!(serializer.serialize_struct_elt("doc_values", doc_values)),
			None => ()
		};
		match T::include_in_all() {
			Some(include_in_all) => try!(serializer.serialize_struct_elt("include_in_all", include_in_all)),
			None => ()
		};
		match T::index() {
			Some(index) => try!(serializer.serialize_struct_elt("index", index)),
			None => ()
		};
		match T::store() {
			Some(store) => try!(serializer.serialize_struct_elt("store", store)),
			None => ()
		};

		try!(serializer.serialize_struct_elt("format", T::format()));

		match T::ignore_malformed() {
			Some(ignore_malformed) => try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed)),
			None => ()
		};

		match T::null_value() {
			Some(null_value) => try!(serializer.serialize_struct_elt("null_value", null_value)),
			None => ()
		};

		match T::precision_step() {
			Some(precision_step) => try!(serializer.serialize_struct_elt("precision_step", precision_step)),
			None => ()
		};

		Ok(None)
	}
}