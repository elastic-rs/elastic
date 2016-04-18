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
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(elastic_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::ElasticDate;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = Vec::new();
	/// let mut ser = serde_json::Serializer::new(&mut writer);
	/// let _ = TypeMapper::map(&MyType::default(), &mut ser).unwrap();
	/// # }
	/// ```
	pub fn map<S>(t: &'a T, serializer: &mut S) -> Result<(), S::Error> where
	S: serde::Serializer {
		serializer.serialize_struct(
			<M as ElasticTypeMapping<()>>::data_type(),
			<M as ElasticUserTypeMapping<'a, T>>::Visitor::new(&t)
		)
	}

	/// Map a user-defined type to a `String`.
	///
	/// The mapping is emitted as a json field, where the key is the name of the type, as defined by `M::data_type()`.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(elastic_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::ElasticDate;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let _ = TypeMapper::map_str(&MyType::default()).unwrap();
	/// # }
	/// ```
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
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(elastic_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::ElasticDate;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let val = TypeMapper::map_val(&MyType::default()).unwrap();
	///
	/// let ty = val.lookup("properties.my_date.type");
	/// # }
	/// ```
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
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(elastic_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// # use serde::{ Serialize, Deserialize };
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::date::ElasticDate;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = Vec::new();
	/// let mut ser = serde_json::Serializer::new(&mut writer);
	///
	/// let mytype = MyType::default();
	/// let _ = FieldMapper::map("my_date", &mytype.my_date, &mut ser).unwrap();
	/// # }
	/// ```
	pub fn map<S>(key: &'static str, _: &T, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct_elt(key, M::default())
	}
}
