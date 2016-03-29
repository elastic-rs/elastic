//! Helper mappers for `ElasticType`.
//! 
//! Mapping for types is inferred from the generic mapping parameters on `ElasticType`.
//! There are two mappers provided:
//! 
//! - `TypeMapper` for mapping user-defined types for the [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html)
//! - `FieldMapper` for mapping fields on user-defined types. User-defined types as fields are mapped as [Nested objects](https://www.elastic.co/guide/en/elasticsearch/guide/current/nested-objects.html)

use std::marker::PhantomData;
use serde;
use ::mapping::{ ElasticType, ElasticTypeMapping, NullMapping };
use ::object::{ ElasticUserTypeMapping, ElasticObjectTypeVisitor };

/// Helper for mapping user-defined types.
/// 
/// This mapper is designed to take a given user-defined type and pass it around to various visitors to map fields.
pub struct TypeMapper<'a, T: 'a + ElasticType<M, ()> + Clone + Default, M: ElasticUserTypeMapping<'a, T>> {
	phantom_a: PhantomData<&'a ()>,
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>
}
impl <'a, T: 'a + ElasticType<M, ()> + Clone + Default, M: ElasticUserTypeMapping<'a, T>> TypeMapper<'a, T, M> {
	/// Map a user-defined type.
	/// 
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	pub fn map<S>(t: &'a T, serializer: &mut S) -> Result<(), S::Error> 
	where S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticTypeMapping<()>>::data_type(), 
			<M as ElasticUserTypeMapping<'a, T>>::Visitor::new(&t)
		)
	}
}

/// Helper for mapping data type fields.
/// 
/// The mapping is inferred from the given `ElasticType`.
#[derive(Default)]
pub struct FieldMapper<T: ElasticType<M, F>, M: ElasticTypeMapping<F> = NullMapping, F = ()> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T: ElasticType<M, F>, M: ElasticTypeMapping<F>, F> FieldMapper<T, M, F> {
	/// Infer the mapping of a data type and map using its `Visitor`.
	/// 
	/// The mapping is emitted as a json field, where the key is the name of the field on the type.
	pub fn map<S>(key: &'static str, _: &T, serializer: &mut S) -> Result<(), S::Error> 
	where S: serde::Serializer {
		serializer.serialize_struct_elt(key, M::default())
	}
}