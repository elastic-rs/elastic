//! Mapping for the Elasticsearch `date` type.

use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use super::{ DateFormat, DefaultFormat };
use ::mapping::{ ElasticMapping, IndexAnalysis };

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
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::date::prelude::*;
/// 
/// #[derive(Debug, Clone, Default)]
/// pub struct MyDateMapping<T: DateFormat> {
/// 	phantom: std::marker::PhantomData<T>
/// }
/// 
/// impl <T: DateFormat> ElasticMapping<T> for MyDateMapping<T> {
/// 	type Visitor = ElasticDateMappingVisitor<T, MyDateMapping<T>>;
/// 
/// 	fn data_type() -> &'static str {
/// 		"date"
/// 	}
/// }
/// 
/// impl <T: DateFormat> ElasticDateMapping<T> for MyDateMapping<T> {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
/// 
/// impl <T: DateFormat> serde::Serialize for MyDateMapping<T> {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
/// 
/// The above example binds the mapping to the `BasicDateTime` format, so `get_null_value` returns a properly formated value.
pub trait ElasticDateMapping<T: DateFormat>
where Self : ElasticMapping<T> + Sized + Serialize + Default {
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

/// Default mapping for `DateTime`.
#[derive(Debug, Clone, Default)]
pub struct DefaultDateMapping<T: DateFormat = DefaultFormat> {
	phantom: PhantomData<T>
}

impl <T: DateFormat> ElasticMapping<T> for DefaultDateMapping<T> {
	type Visitor = ElasticDateMappingVisitor<T, DefaultDateMapping<T>>;

	fn data_type() -> &'static str {
		"date"
	}
}

impl <T: DateFormat> ElasticDateMapping<T> for DefaultDateMapping<T> { }

impl <T: DateFormat> serde::Serialize for DefaultDateMapping<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", Self::get_visitor())
	}
}

/// Visitor for a `date` map.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticDateMappingVisitor<F: DateFormat, T: ElasticDateMapping<F>> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F: DateFormat, T: ElasticDateMapping<F>> serde::ser::MapVisitor for ElasticDateMappingVisitor<F, T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", T::data_type()));

		if let Some(boost) = T::boost() {
			try!(serializer.serialize_struct_elt("boost", boost));
		};

		if let Some(doc_values) = T::doc_values() {
			try!(serializer.serialize_struct_elt("doc_values", doc_values));
		};
		
		if let Some(include_in_all) = T::include_in_all() {
			try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
		};

		if let Some(index) = T::index() {
			try!(serializer.serialize_struct_elt("index", index));
		};

		if let Some(store) = T::store() {
			try!(serializer.serialize_struct_elt("store", store));
		};

		try!(serializer.serialize_struct_elt("format", T::format()));

		if let Some(ignore_malformed) = T::ignore_malformed() {
			try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed));
		};

		if let Some(null_value) = T::null_value() {
			try!(serializer.serialize_struct_elt("null_value", null_value));
		};

		if let Some(precision_step) = T::precision_step() {
			try!(serializer.serialize_struct_elt("precision_step", precision_step));
		};

		Ok(None)
	}
}