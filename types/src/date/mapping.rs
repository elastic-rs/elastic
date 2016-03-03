use std::marker::PhantomData;
use chrono::{ Datelike, Timelike };
use serde;
use serde::{ Serializer, Serialize };
use super::{ Format, ParseError, DefaultFormat };
use ::mapping::{ ElasticMapping, ElasticType, ElasticMappingVisitor, IndexAnalysis };

/// The base requirements for mapping a `date` type.
/// 
/// # Examples
/// 
/// Custom mappings can be defined by implementing `ElasticDateMapping`:
/// 
/// ```
/// use elastic_types::date::{ ElasticDateMapping, BasicDateTime, NullValue };
/// use elastic_types::date::Format;
/// 
/// struct MyDateMapping;
/// impl ElasticDateMapping<BasicDateTime> for MyDateMapping {
/// 	fn get_ignore_malformed() -> Option<bool> {
/// 		Some(true)
///		}
/// 
/// 	fn get_null_value() -> Option<NullValue> {
/// 		Some(NullValue::Default("20150701T000000.000Z"))
/// 	}
/// }
/// ```
/// 
/// The above example binds the mapping to the `BasicDateTime` format, so `get_null_value` returns a properly formated value.
pub trait ElasticDateMapping<T: Format>
where Self : Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn get_boost() -> Option<f32> {
		None
	}

	/// Should the field be stored on disk in a column-stride fashion, 
	/// so that it can later be used for sorting, aggregations, or scripting? 
	/// Accepts `true` (default) or `false`.
	fn get_doc_values() -> Option<bool> {
		None
	}

	/// Whether or not the field value should be included in the `_all` field? 
	/// Accepts true or false. 
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false. 
	/// Otherwise defaults to `true`.
	fn get_include_in_all() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn get_index() -> Option<IndexAnalysis> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	fn get_store() -> Option<bool> {
		None
	}

	/// The date format(s) that can be parsed.
	fn get_format() -> &'static str {
		T::name()
	}

	/// If `true`, malformed numbers are ignored. 
	/// If `false` (default), malformed numbers throw an exception and reject the whole document.
	fn get_ignore_malformed() -> Option<bool> {
		None
	}

	/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	fn get_null_value() -> Option<&'static str> {
		None
	}

	/// Controls the number of extra terms that are indexed to make range queries faster. Defaults to 16.
	fn get_precision_step() -> Option<i32> {
		None
	}
}

impl <T: Format, M: ElasticDateMapping<T>> ElasticMapping<T> for M {
	type Visitor = ElasticDateMappingVisitor<T, M>;

	fn get_type() -> &'static str {
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

/// A Rust representation of an Elasticsearch `date`.
pub trait ElasticDateType<F: Format = DefaultFormat, T: ElasticMapping<F> + ElasticDateMapping<F> = DefaultDateMapping<F>> 
where Self: Sized + ElasticType<T, F> + Datelike + Timelike {
	/// Parse the date and time from a string.
	/// 
	/// The format of the string must match the given `Format`.
	/// 
	/// # Examples
	/// 
	/// Parsing from a specified `Format`.
	/// 
	/// ```
	/// use elastic_types::date::{ ElasticDateType, DateTime, BasicDateTime };
	/// 
	/// let date = DateTime::<BasicDateTime>::parse("20151126T145543.778Z").unwrap();
	/// ```
	fn parse(date: &str) -> Result<Self, ParseError>;

	/// Format the date and time as a string.
	/// 
	/// The format of the string is specified by the given `Format`.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::date::{ ElasticDateType, DateTime, BasicDateTime };
	/// 
	/// let date: DateTime = DateTime::now();
	/// let fmt = date.format();
	/// 
	/// //eg: 20151126T145543.778Z
	/// println!("{}", fmt);
	/// ```
	fn format<'a>(&self) -> String;
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
		match T::get_boost() {
			Some(boost) => try!(serializer.serialize_struct_elt("boost", boost)),
			None => ()
		};
		match T::get_doc_values() {
			Some(doc_values) => try!(serializer.serialize_struct_elt("doc_values", doc_values)),
			None => ()
		};
		match T::get_include_in_all() {
			Some(include_in_all) => try!(serializer.serialize_struct_elt("include_in_all", include_in_all)),
			None => ()
		};
		match T::get_index() {
			Some(index) => try!(serializer.serialize_struct_elt("index", index)),
			None => ()
		};
		match T::get_store() {
			Some(store) => try!(serializer.serialize_struct_elt("store", store)),
			None => ()
		};

		try!(serializer.serialize_struct_elt("format", T::get_format()));

		match T::get_ignore_malformed() {
			Some(ignore_malformed) => try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed)),
			None => ()
		};

		match T::get_null_value() {
			Some(null_value) => try!(serializer.serialize_struct_elt("null_value", null_value)),
			None => ()
		};

		match T::get_precision_step() {
			Some(precision_step) => try!(serializer.serialize_struct_elt("precision_step", precision_step)),
			None => ()
		};

		Ok(None)
	}
}