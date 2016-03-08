//! Implementation for data type mappings.

pub mod prelude {
    //! Includes mapping types for all data types.
    //! 
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    
    pub use super::{ ElasticDataType, ElasticMapping, NullMapping, IndexAnalysis, ElasticMappingVisitor };
    pub use ::date::mapping::*;
    pub use ::string::mapping::*;
}

use std::marker::PhantomData;
use serde;

/// The base representation of an Elasticsearch data type.
/// 
/// # Links
/// 
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
pub trait ElasticDataType<T: ElasticMapping<F>, F> : TypeEllision { }

#[doc(hidden)]
pub enum TypeEllisionKind {
	Explicit,
	Ellided
}
#[doc(hidden)]
pub trait TypeEllision {
	fn get_ellision() -> TypeEllisionKind {
		TypeEllisionKind::Explicit
	}
}

/// The base requirements for mapping an Elasticsearch data type.
/// 
/// Each type has its own implementing structures with extra type-specific mapping parameters.
pub trait ElasticMapping<F = ()>
where Self: Default + serde::Serialize {
	#[doc(hidden)]
	type Visitor : serde::ser::MapVisitor + Default;

	/// An optional associated type that mappings may need.
	/// 
	/// For example; the `Format` trait on `DateTime`.
	type Format = F;

	#[doc(hidden)]
	fn get_visitor() -> Self::Visitor {
		Self::Visitor::default()
	}

	/// Get the type name for this mapping, like `date` or `string`.
	fn data_type() -> &'static str {
		""
	}
}

/// A mapping implementation for a non-core type, or any where nobody cares about how it's mapped.
#[derive(Debug, PartialEq, Default)]
pub struct NullMapping;
impl ElasticMapping for NullMapping {
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

impl ElasticDataType<(), NullMapping> for .. { }

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

//TODO: Make this take in str for field name
/// Base visitor for serialising datatype mappings.
pub struct ElasticMappingVisitor<T: ElasticMapping> {
	phantom: PhantomData<T>
}

impl <T: ElasticMapping> Default for ElasticMappingVisitor<T> {
	fn default() -> ElasticMappingVisitor<T> {
		ElasticMappingVisitor::<T> {
			phantom: PhantomData
		}
	}
}

impl <T: ElasticMapping> serde::ser::MapVisitor for ElasticMappingVisitor<T> {
	fn visit<S>(&mut self, _: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		Ok(None)
	}
}