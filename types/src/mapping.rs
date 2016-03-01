//! Implementation for data type mappings.

use std::marker::PhantomData;
use serde;

/// The base requirements for mapping an Elasticsearch type.
/// 
/// Each type will have its own implementing structures with extra type-specific mapping parameters.
pub trait ElasticMapping {
	/// The serialisation visitor used to inspect this mapping.
	type Visitor : serde::ser::MapVisitor + Default;

	/// Gets an instance of the `Visitor` for serialisation.
	fn get_visitor() -> Self::Visitor {
		Self::Visitor::default()
	}

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
}

/// A mapping implementation for a non-core type, or any where nobody cares about how it's mapped.
pub struct NullMapping;
impl ElasticMapping for NullMapping {
	type Visitor = NullMappingVisitor;
}

impl serde::Serialize for NullMapping {
    fn serialize<S>(&self, _: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        Ok(())
    }
}

/// A default empty visitor.
#[derive(Default)]
pub struct NullMappingVisitor;
impl serde::ser::MapVisitor for NullMappingVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: serde::Serializer {
		Ok(None)
    }
}

impl ElasticType<NullMapping> for .. { }

/// A type that can be indexed in Elasticsearch.
//TODO: Rename to ElasticDataType
pub trait ElasticType<T: ElasticMapping> { }

//TODO: Need ElasticType, which is a main type that can be derived
//This needs to map each property. Probably through a custom derive

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
//TODO: Generate this code. A macro should be fine
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
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: serde::Serializer {
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

		Ok(None)
    }
}