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

	//pub use ::object::*;
	//pub use ::mappers::*;
	//pub use ::date::mapping::*;
	//pub use ::ip::mapping::*;
	//pub use ::geo::mapping::*;
	//pub use ::string::mapping::*;
	//pub use ::number::mapping::*;
	pub use ::boolean::mapping::*;
}

use std::collections;
use std::hash::Hash;
use std::marker::PhantomData;
use serde;
use serde_json;

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
pub trait ElasticType<T, F> where
T: 'static + ElasticFieldMapping<F>,
Self: serde::Serialize {
	/// Get the type name for the given mapping.
	const NAME: &'static str = T::NAME;

	/// Get the mapping for this type.
	const MAPPING: T = T::default();
}

/// The base requirements for mapping an Elasticsearch data type.
///
/// Each type has its own implementing structures with extra type-specific mapping parameters.
/// If you're building your own Elasticsearch types, see `ElasticUserTypeMapping`,
/// which is a specialization of `ElasticFieldMapping<()>`.
pub trait ElasticFieldMapping<F>
where Self: 'static + Default + Clone + serde::Serialize {
	/// Get the type name for this mapping, like `date` or `string`.
	const DATA_TYPE: &'static str = "object";

	#[doc(hidden)]
	const NAME: &'static str = Self::DATA_TYPE;
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

/// Mapping for a collection.
///
/// In Elasticsearch, arrays aren't a special type, anything can be indexed as an array.
/// So the mapping for an array is just the mapping for its members.
#[derive(Debug, Default, Clone)]
pub struct ElasticArrayMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	phantom_m: PhantomData<M>,
	phantom_f: PhantomData<F>
}

impl <M, F> ElasticFieldMapping<F> for ElasticArrayMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	const DATA_TYPE: &'static str = M::DATA_TYPE;
}

impl <M, F> serde::Serialize for ElasticArrayMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		M::MAPPING.serialize(&mut serializer)
	}
}

impl <T, M, F> ElasticType<ElasticArrayMapping<M, F>, F> for Vec<T> where
T: 'static + ElasticType<M, F>,
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {

}

/// Mapping for an optional type.
///
/// Elasticsearch doesn't differentiate between properties that are nullable or not.
/// That means the only _really_ safe way to map your fields is to make them all `Option<T>`
/// instead of `T`.
/// This probably isn't necessary unless you have no control over the indexed data though.
#[derive(Debug, Default, Clone)]
pub struct ElasticOptionMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	phantom_m: PhantomData<M>,
	phantom_f: PhantomData<F>
}

impl <M, F> ElasticFieldMapping<F> for ElasticOptionMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	const DATA_TYPE: &'static str = M::DATA_TYPE;
}

impl <M, F> serde::Serialize for ElasticOptionMapping<M, F> where
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		M::MAPPING.serialize(&mut serializer)
	}
}

impl <T, M, F> ElasticType<ElasticOptionMapping<M, F>, F> for Option<T> where
T: 'static + ElasticType<M, F>,
M: 'static + ElasticFieldMapping<F>,
F: 'static + Default + Clone {

}

//It's not possible to know at compile-time exactly what type Value can take.
//The only way to map it as as a default object.
impl ElasticType<DefaultMapping, ()> for serde_json::Value {

}

impl <K, V> ElasticType<DefaultMapping, ()> for collections::BTreeMap<K, V> where
K: AsRef<str> + Ord + serde::Serialize,
V: serde::Serialize {

}

impl <K, V> ElasticType<DefaultMapping, ()> for collections::HashMap<K, V> where
K: AsRef<str> + Eq + Hash + serde::Serialize,
V: serde::Serialize {

}
