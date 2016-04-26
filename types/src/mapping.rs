//! Base requirements for type mappings.
//!
//! There are two kinds of types we can map in Elasticsearch; `field`/`data` types and `user-defined` types.
//! Either kind of type must implement `ElasticType`, which captures the mapping and possible formatting requirements as generic parameters.
//! Most of the work lives in the `ElasticTypeMapping`, which holds the serialisation requirements to convert a Rust type into an Elasticsearch mapping.
//! User-defined types must also implement `ElasticUserTypeMapping`, which maps the fields of a struct as properties, and treats the type as `nested` when used as a field itself.
//!
//! # Notes
//!
//! Currently, there's a lot of ceremony around the type mapping. The reason for doing this with types instead of simple hashmaps is to try and capture type mapping using types themselves.
//! This means more boilerplate while certain Rust features haven't landed yet ([specialisation](https://github.com/rust-lang/rust/issues/31844) and [negative trait bounds](https://github.com/rust-lang/rfcs/issues/1053)),
//! but it also constrains the shapes that Elasticsearch types can take by using the Rust type system. That seems like a nice property.
//!
//! The mapping serialisation in general tries to limit allocations wherever possible, but more can be done to clean this up.
//!
//! # Links
//! - [Field Types](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
//! - [User-defined Types](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html)

pub mod prelude {
	//! Includes mapping types for all data types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::{
		ElasticType,
		ElasticTypeMapping,
		NullMapping,
		IndexAnalysis
	};

	pub use ::object::*;
	pub use ::mappers::*;

	#[cfg(feature="date_ty")]
	pub use ::date::mapping::*;
	#[cfg(feature="string_ty")]
	pub use ::string::mapping::*;
	#[cfg(feature="number_ty")]
	pub use ::number::mapping::*;
	#[cfg(feature="boolean_ty")]
	pub use ::boolean::mapping::*;
}

use std::marker::PhantomData;
use serde;

/// The base representation of an Elasticsearch data type.
///
/// `ElasticType` is the main `trait` you need to care about when building your own Elasticsearch types.
/// Each type has two generic arguments that help define its mapping:
///
/// - A mapping type, which implements `ElasticTypeMapping`
/// - A format type, which is usually `()`. Types with multiple formats, like `ElasticDate`, can use the format in the type definition.
///
/// # Links
///
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
pub trait ElasticType<T, F> where
T: ElasticTypeMapping<F>,
Self : serde::Serialize + serde::Deserialize {
	/// Get the type name for the given mapping.
	fn name() -> &'static str {
		T::name()
	}
}

/// The base requirements for mapping an Elasticsearch data type.
///
/// Each type has its own implementing structures with extra type-specific mapping parameters.
/// If you're building your own Elasticsearch types, see `ElasticUserTypeMapping`,
/// which is a specialization of `ElasticTypeMapping<()>`.
pub trait ElasticTypeMapping<F>
where Self: Default + Clone + serde::Serialize {
	//TODO: Make this bound take ElasticTypeVisitor
	#[doc(hidden)]
	type Visitor : serde::ser::MapVisitor + Default;

	/// An optional associated type that mappings may need.
	///
	/// For example the `DateFormat` trait on `ElasticDate`.
	type Format = F;

	#[doc(hidden)]
	fn get_visitor() -> Self::Visitor {
		Self::Visitor::default()
	}

	/// Get the type name for this mapping, like `date` or `string`.
	fn data_type() -> &'static str {
		"object"
	}

	#[doc(hidden)]
	fn name() -> &'static str {
		Self::data_type()
	}
}

//TODO: Determine if the bound on just T is sufficient
/// Base visitor for serialising a datatype.
pub trait ElasticTypeVisitor<'a, T> where
T: 'a,
Self: serde::ser::MapVisitor {
	/// Create a new visitor from a borrowed datatype.
	fn new(data: &'a T) -> Self;
}

/// A mapping implementation for a non-core type, or any where it's ok for Elasticsearch to infer the mapping at index-time.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct NullMapping;
impl ElasticTypeMapping<()> for NullMapping {
	type Visitor = NullMappingVisitor;

	fn data_type() -> &'static str {
		"object"
	}
}

impl serde::Serialize for NullMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", NullMappingVisitor::default())
	}
}

/// A default empty visitor.
#[derive(Default, Debug, PartialEq)]
pub struct NullMappingVisitor;
impl serde::ser::MapVisitor for NullMappingVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", NullMapping::data_type()));

		Ok(None)
	}
}

impl serde::Serialize for IndexAnalysis {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_str(match *self {
			IndexAnalysis::Analyzed => "analyzed",
			IndexAnalysis::NotAnalyzed => "not_analyzed",
			IndexAnalysis::No => "no"
		})
	}
}

/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
#[derive(Debug, Clone, Copy)]
pub enum IndexAnalysis {
	/// This option applies only to string fields, for which it is the default.
	/// The string field value is first analyzed to convert the string into terms
	/// (e.g. a list of individual words), which are then indexed.
	/// At search time, the query string is passed through (usually) the same analyzer
	/// to generate terms in the same format as those in the index.
	/// It is this process that enables full text search.
	Analyzed,
	/// Add the field value to the index unchanged, as a single term.
	/// This is the default for all fields that support this option except for string fields.
	/// `not_analyzed` fields are usually used with term-level queries for structured search.
	NotAnalyzed,
	/// Do not add this field value to the index. With this setting, the field will not be queryable.
	No
}

/// Mapping for a collection.
///
/// In Elasticsearch, arrays aren't a special type, anything can be indexed as an array.
/// So the mapping for an array is just the mapping for its members.
#[derive(Debug, Default, Clone)]
pub struct ElasticArrayMapping<M, F> where
M: ElasticTypeMapping<F>,
F: Default + Clone {
	phantom_m: PhantomData<M>,
	phantom_f: PhantomData<F>
}

impl <M, F> ElasticTypeMapping<F> for ElasticArrayMapping<M, F> where
M: ElasticTypeMapping<F>,
F: Default + Clone {
	type Visitor = M::Visitor;

	fn data_type() -> &'static str {
		M::data_type()
	}
}

impl <M, F> serde::Serialize for ElasticArrayMapping<M, F> where
M: ElasticTypeMapping<F>,
F: Default + Clone {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", Self::get_visitor())
	}
}

impl <T, M, F> ElasticType<ElasticArrayMapping<M, F>, F> for Vec<T> where
T: ElasticType<M, F>,
M: ElasticTypeMapping<F>,
F: Default + Clone {

}
