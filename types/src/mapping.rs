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
		IndexAnalysis, 
		ElasticTypeMappingVisitor
	};

	pub use ::user_type::*;
	pub use ::mappers::*;
	pub use ::date::mapping::*;
	pub use ::string::mapping::*;
}

use std::marker::PhantomData;
use serde;

/// The base representation of an Elasticsearch data type.
/// 
/// 
/// `ElasticType` is the main `trait` you need to care about when building your own Elasticsearch types.
/// Each type has two generic arguments that help define its mapping:
/// 
/// - A mapping type, which implements `ElasticTypeMapping`
/// - A format type, which is usually `()`. Types with multiple formats, like `DateTime`, can use the format in the type definition.
/// 
/// ### Automatic
/// 
/// The `elastic_macros` crate provides a plugin for you to automatically derive `ElasticType`:
/// 
/// ```
/// //TODO: Implement this
/// ```
/// 
/// ### Manual
/// 
/// You can also derive `ElasticType` manually to get more control over the structure of your type mapping.
/// 
/// ```
/// //TODO: Implement this
/// ```
/// 
/// # Links
/// 
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
pub trait ElasticType<T: ElasticTypeMapping<F>, F> 
where Self : serde::Serialize { }

/// The base requirements for mapping an Elasticsearch data type.
/// 
/// Each type has its own implementing structures with extra type-specific mapping parameters.
/// If you're building your own Elasticsearch types, see `TypeMapping`, which is a specialization of `ElasticTypeMapping<()>`.
pub trait ElasticTypeMapping<F>
where Self: Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor : serde::ser::MapVisitor + Default;

	/// An optional associated type that mappings may need.
	/// 
	/// For example; the `DateFormat` trait on `DateTime`.
	type Format = F;

	#[doc(hidden)]
	fn get_visitor() -> Self::Visitor {
		Self::Visitor::default()
	}

	/// Get the type name for this mapping, like `date` or `string`.
	/// 
	/// For user-defined types, this is the name of the type in Elasticsearch, like `my_type`.
	fn data_type() -> &'static str {
		""
	}
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

/// Base visitor for serialising datatype mappings.
pub struct ElasticTypeMappingVisitor<T: ElasticTypeMapping<()>> {
	phantom: PhantomData<T>
}

impl <T: ElasticTypeMapping<()>> Default for ElasticTypeMappingVisitor<T> {
	fn default() -> ElasticTypeMappingVisitor<T> {
		ElasticTypeMappingVisitor::<T> {
			phantom: PhantomData
		}
	}
}

impl <T: ElasticTypeMapping<()>> serde::ser::MapVisitor for ElasticTypeMappingVisitor<T> {
	fn visit<S>(&mut self, _: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		Ok(None)
	}
}