//! Implementation for data type mappings.

pub mod prelude {
	//! Includes mapping types for all data types.
	//! 
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
	
	pub use super::{ 
		ElasticDataType, 
		ElasticMapping, 
		NullMapping, 
		IndexAnalysis, 
		ElasticMappingVisitor,
		TypeMapping,
		TypeMappingVisitor
	};
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
pub trait ElasticDataType<T: ElasticMapping<F>, F> 
where Self : serde::Serialize { }

/// The base requirements for mapping an Elasticsearch data type.
/// 
/// Each type has its own implementing structures with extra type-specific mapping parameters.
pub trait ElasticMapping<F>
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
	fn data_type() -> &'static str {
		""
	}
}

/// A mapping implementation for a non-core type, or any where nobody cares about how it's mapped.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct NullMapping;
impl ElasticMapping<()> for NullMapping {
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
pub struct ElasticMappingVisitor<T: ElasticMapping<()>> {
	phantom: PhantomData<T>
}

impl <T: ElasticMapping<()>> Default for ElasticMappingVisitor<T> {
	fn default() -> ElasticMappingVisitor<T> {
		ElasticMappingVisitor::<T> {
			phantom: PhantomData
		}
	}
}

impl <T: ElasticMapping<()>> serde::ser::MapVisitor for ElasticMappingVisitor<T> {
	fn visit<S>(&mut self, _: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		Ok(None)
	}
}

//TODO: Rename ElasticTypeMapping
/// The base requirements for mapping a user-defined type.
pub trait TypeMapping<'a, T>
where Self: ElasticMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor : TypeMappingVisitor<'a, T>;

	#[doc(hidden)]
	fn get_visitor(t: &'a T) -> <Self as TypeMapping<'a, T>>::Visitor {
		<Self as TypeMapping<'a, T>>::Visitor::new(&t)
	}
}

/// Base visitor trait for serialising user-defined type mappings.
pub trait TypeMappingVisitor<'a, T>
where Self: Default + serde::ser::MapVisitor {
	/// Create a visitor instance from a given type
	fn new(data: &'a T) -> Self;
}

/// Helper for mapping data type fields.
#[derive(Default)]
pub struct DataMapper<T: ElasticDataType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T: ElasticDataType<M, F>, M: ElasticMapping<F>, F> DataMapper<T, M, F> {
	/// Infer the mapping of a data type and map using its `Visitor`.
	pub fn map<S>(key: &'static str, _: &T, serializer: &mut S) -> Result<(), S::Error> 
	where S: serde::Serializer {
		serializer.serialize_struct_elt(key, M::default())
	}
}

/// Helper for mapping user-defined types.
pub struct TypeMapper<'a, T: ElasticDataType<M, ()>, M: TypeMapping<'a, T>> {
	phantom_a: PhantomData<&'a ()>,
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>
}
impl <'a, T: ElasticDataType<M, ()>, M: TypeMapping<'a, T>> TypeMapper<'a, T, M> {
	/// Map a user-defined type.
	pub fn map<S>(t: &'a T, serializer: &mut S) -> Result<(), S::Error> 
	where S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticMapping<()>>::data_type(), 
			<M as TypeMapping<'a, T>>::get_visitor(&t)
		)
	}
}

macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticDataType<NullMapping, ()> for $t { }
		)*
	)
}

impl_mapping!(
	bool,
	isize,
	i8,
	i16,
	i32,
	i64,
	usize,
	u8,
	u16,
	u32,
	u64,
	f32,
	f64,
	char
);

impl <T: serde::Serialize> ElasticDataType<NullMapping, ()> for Vec<T> { }
impl <'a, T: serde::Serialize> ElasticDataType<NullMapping, ()> for &'a [T] { }