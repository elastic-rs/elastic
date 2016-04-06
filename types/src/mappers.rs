//! Helper mappers for `ElasticType`.
//!
//! Mapping for types is inferred from the generic mapping parameters on `ElasticType`.
//! There are two mappers provided:
//!
//! - `TypeMapper` for mapping user-defined types for the [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html)
//! - `FieldMapper` for mapping fields on user-defined types. User-defined types as fields are mapped as [Nested objects](https://www.elastic.co/guide/en/elasticsearch/guide/current/nested-objects.html)

use std::error::Error;
use std::marker::PhantomData;
use serde;
use serde::ser::Error as SerError;
use serde_json;
use ::mapping::{ ElasticType, ElasticTypeMapping, NullMapping };
use ::object::{ ElasticUserTypeMapping, ElasticObjectTypeVisitor };

/// Helper for mapping user-defined types.
///
/// This mapper is designed to take a given user-defined type and pass it around to various visitors to map fields.
pub struct TypeMapper<'a, T, M> where
T: 'a + ElasticType<M, ()> + Clone + Default,
M: ElasticUserTypeMapping<'a, T> {
	phantom_a: PhantomData<&'a ()>,
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>
}
impl <'a, T, M> TypeMapper<'a, T, M> where
T: 'a + ElasticType<M, ()> + Clone + Default,
M: ElasticUserTypeMapping<'a, T> {
	/// Map a user-defined type with a given `Serializer`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	pub fn map<S>(t: &'a T, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticTypeMapping<()>>::data_type(),
			<M as ElasticUserTypeMapping<'a, T>>::Visitor::new(&t)
		)
	}

	/// Map a user-defined type to a `String`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	pub fn map_str(t: &'a T) -> Result<String, serde_json::Error> {
		let mut writer = Vec::new();
		{
			let mut ser = serde_json::Serializer::new(&mut writer);
			let _ = try!(Self::map(&t, &mut ser));
		}

		String::from_utf8(writer).map_err(|e| serde_json::Error::custom(e.description()))
	}

	/// Map a user-defined type to a `serde_json::Value`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	pub fn map_val(t: &'a T) -> Result<serde_json::Value, serde_json::Error> {
		let mut ser = serde_json::value::Serializer::new();
		let _ = try!(Self::map(&t, &mut ser));

		Ok(ser.unwrap())
	}
}

/// Helper for mapping data type fields.
///
/// The mapping is inferred from the given `ElasticType`.
#[derive(Default)]
pub struct FieldMapper<T, M = NullMapping, F = ()> where
T: ElasticType<M, F>,
M: ElasticTypeMapping<F> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T, M, F> FieldMapper<T, M, F> where
T: ElasticType<M, F>,
M: ElasticTypeMapping<F> {
	/// Infer the mapping of a data type and map using its `Visitor`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the field on the type.
	pub fn map<S>(key: &'static str, _: &T, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct_elt(key, M::default())
	}
}
