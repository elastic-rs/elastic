//! Requirements for mapping user-defined types.

use std::marker::PhantomData;
use serde;
use ::mapping::{ ElasticTypeMapping };

/// The base requirements for mapping a user-defined type.
/// 
/// # Examples
/// 
/// Define a custom type mapping:
/// 
/// ```
/// //TODO: Implement this
/// ```
pub trait ElasticUserTypeMapping<'a, T: 'a + Clone + Default>
where Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticUserTypeVisitor<'a, T>;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticUserTypeVisitor<'a, T>;

	/// The name of the user-defined type used in Elasticsearch.
	fn name() -> &'static str;
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticUserTypeProperties<'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<M>
}
impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> ElasticUserTypeProperties<'a, T, M> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticUserTypeProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> serde::Serialize for ElasticUserTypeProperties<'a, T, M> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping<T>>::PropertiesVisitor::new(&self.data))
	}
}

/// Visitor for the `ElasticUserTypeProperties` struct and given user-defined type.
/// 
/// The purpose of this trait is to serialise the mapping for each datatype on the user-defined type `T`.
/// To make this easier, the `DataMapper` can be used to infer the mapping for a field that implements `ElasticType`.
/// 
/// # Examples
/// 
/// Implement `ElasticUserTypeVisitor` for a user-defined type:
/// 
/// ```
/// //TODO: Implement
/// ```
pub trait ElasticUserTypeVisitor<'a, T: 'a + Clone + Default>
where Self: serde::ser::MapVisitor { 
	/// Create a new visitor from a borrowed user-defined type.
	fn new(data: &'a T) -> Self;
}