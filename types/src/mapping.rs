//! Base requirements for type mappings.
//!
//! There are two kinds of types we can map in Elasticsearch; `field`/`data` types and `user-defined` types.
//! Either kind of type must implement `ElasticType`, which captures the mapping and possible formatting
//! requirements as generic parameters.
//! Most of the work lives in the `ElasticFieldMapping`, which holds the serialisation requirements
//! to convert a Rust type into an Elasticsearch mapping.
//! User-defined types must also implement `ElasticUserTypeMapping`, which maps the fields of a struct as properties,
//! and treats the type as `nested` when used as a field itself.
//!
//! # Notes
//!
//! Currently, there's a lot of ceremony around the type mapping.
//! The reason for doing this with types instead of simple hashmaps is to try and capture type mapping using types themselves.
//! This means more boilerplate while certain Rust features haven't landed yet ([negative trait bounds](https://github.com/rust-lang/rfcs/issues/1053)),
//! but it also constrains the shapes that Elasticsearch types can take by using the Rust type system.
//! That seems like a nice property.
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
		ElasticFieldMapping,
		DefaultMapping,
		IndexAnalysis
	};

	//pub use ::mappers::*;
	//pub use ::object::*;
	//pub use ::date::mapping::*;
	//pub use ::ip::mapping::*;
	//pub use ::geo::mapping::*;
	//pub use ::string::mapping::*;
	//pub use ::number::mapping::*;
	//pub use ::boolean::mapping::*;
}

use std::collections::{ BTreeMap, HashMap };
use std::hash::Hash;
use std::marker::PhantomData;
use serde::{ Serialize, Serializer };
use serde_json::Value;

/// The base representation of an Elasticsearch data type.
///
/// `ElasticType` is the main `trait` you need to care about when building your own Elasticsearch types.
/// Each type has two generic arguments that help define its mapping:
///
/// - A mapping type, which implements `ElasticFieldMapping`
/// - A format type, which is usually `()`. Types with multiple formats, like `ElasticDate`, can use the format in the type definition.
///
/// # Links
///
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
pub trait ElasticType<M, F> where
M: ElasticFieldMapping<F>,
F: Default,
Self: Serialize {
	/// Get the type name for the given mapping.
	fn name() -> &'static str {
		M::name()
	}

	/// Get the mapping for this type.
	fn mapping() -> M {
		M::default()
	}
}

/// The base requirements for mapping an Elasticsearch data type.
///
/// Each type has its own implementing structures with extra type-specific mapping parameters.
/// If you're building your own Elasticsearch types, see `ElasticUserTypeMapping`,
/// which is a specialization of `ElasticFieldMapping<()>`.
pub trait ElasticFieldMapping<F>
where Self: Default,
F: Default {
	#[doc(hidden)]
	type SerType: Serialize + Default;
	#[doc(hidden)]
	fn ser_type() -> Self::SerType {
		Self::SerType::default()
	}

	/// Get the type name for this mapping, like `date` or `string`.
	fn data_type() -> &'static str { "object" }

	#[doc(hidden)]
	fn name() -> &'static str { Self::data_type() }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ElasticFieldMappingWrapper<M, F> where
M: ElasticFieldMapping<F>,
F: Default {
	_m: PhantomData<M>,
	_f: PhantomData<F>
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

impl Serialize for IndexAnalysis {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		serializer.serialize_str(match *self {
			IndexAnalysis::Analyzed => "analyzed",
			IndexAnalysis::NotAnalyzed => "not_analyzed",
			IndexAnalysis::No => "no"
		})
	}
}

/// A mapping implementation for a non-core type, or anywhere it's ok for Elasticsearch to infer the mapping at index-time.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct DefaultMapping;
impl ElasticFieldMapping<()> for DefaultMapping { 
	type SerType = ElasticFieldMappingWrapper<Self, ()>;
}

impl Serialize for ElasticFieldMappingWrapper<DefaultMapping, ()> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 1));

		try!(serializer.serialize_struct_elt(&mut state, "type", DefaultMapping::data_type()));

		serializer.serialize_struct_end(state)
	}
}

/// Mapping for a wrapped value.
///
/// In Elasticsearch, arrays and optional types aren't special, anything can be indexed as an array or null.
/// So the mapping for an array or optional type is just the mapping for the type it contains.
#[derive(Debug, Default, Clone)]
pub struct ElasticWrappedMapping<M, F> where
M: ElasticFieldMapping<F>,
F: Default {
	_m: PhantomData<M>,
	_f: PhantomData<F>
}

impl <M, F> ElasticFieldMapping<F> for ElasticWrappedMapping<M, F> where
M: ElasticFieldMapping<F>,
F: Default {
	type SerType = M::SerType;

	fn data_type() -> &'static str { M::data_type() }
}

/// Mapping implementation for a `serde_json::Value`.
impl ElasticType<DefaultMapping, ()> for Value { }

/// Mapping implementation for a standard binary tree map.
impl <K, V> ElasticType<DefaultMapping, ()> for BTreeMap<K, V> where
K: AsRef<str> + Ord + Serialize,
V: Serialize { }

/// Mapping implementation for a standard hash map.
impl <K, V> ElasticType<DefaultMapping, ()> for HashMap<K, V> where
K: AsRef<str> + Eq + Hash + Serialize,
V: Serialize { }

impl <T, M, F> ElasticType<ElasticWrappedMapping<M, F>, F> for Vec<T> where
T: ElasticType<M, F>,
M: ElasticFieldMapping<F>,
F: Default { }

impl <T, M, F> ElasticType<ElasticWrappedMapping<M, F>, F> for Option<T> where
T: ElasticType<M, F>,
M: ElasticFieldMapping<F>,
F: Default { }