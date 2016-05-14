use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticBooleanMapping, DefaultBooleanMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultBooleanMapping, ()> for bool { }

/// An Elasticsearch `boolean` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `bool` instead.
///
/// # Examples
///
/// Defining a bool with a mapping:
///
/// ```
/// use elastic_types::boolean::mapping::DefaultBooleanMapping;
/// use elastic_types::boolean::ElasticBoolean;
///
/// let boolean = ElasticBoolean::<DefaultBooleanMapping>::new(true);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	value: bool,
	phantom: PhantomData<T>
}
impl <T> ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	/// Creates a new `ElasticBoolean` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `ElasticBoolean` from a `bool`:
	///
	/// ```
	/// use elastic_types::boolean::mapping::DefaultBooleanMapping;
	/// use elastic_types::boolean::ElasticBoolean;
	///
	/// let boolean = ElasticBoolean::<DefaultBooleanMapping>::new(false);
	/// ```
	pub fn new<I>(boolean: I) -> ElasticBoolean<T> where I: Into<bool> {
		ElasticBoolean {
			value: boolean.into(),
			phantom: PhantomData
		}
	}

	/// Get the value of the boolean.
	pub fn get(&self) -> bool {
		self.value
	}

	/// Set the value of the boolean.
	pub fn set<I>(&mut self, boolean: I) where I: Into<bool> {
		self.value = boolean.into()
	}

	/// Change the mapping of this boolean.
	///
	/// # Examples
	///
	/// Change the mapping for a given `ElasticBoolean`:
	///
	/// ```
	/// # extern crate serde;
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::boolean::prelude::*;
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyBooleanMapping;
	/// # impl ElasticBooleanMapping for MyBooleanMapping {
	/// # 	fn boost() -> Option<f32> {
	/// #			Some(1.5)
	/// #		}
	/// # }
	/// # impl ElasticFieldMapping<()> for MyBooleanMapping {
	/// # 	type Visitor = ElasticBooleanMappingVisitor<MyBooleanMapping>;
	/// # 	type MultiFieldMapping = Self;
	/// # 	fn data_type() -> &'static str {
	/// # 		"boolean"
	/// # 	}
	/// # }
	/// # impl serde::Serialize for MyBooleanMapping {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	/// # 	where S: serde::Serializer {
	/// # 		serializer.serialize_struct("mapping", Self::get_visitor())
	/// # 	}
	/// # }
	/// let es_boolean = ElasticBoolean::<DefaultBooleanMapping>::new(true);
	///
	/// let boolean: ElasticBoolean<MyBooleanMapping> = es_boolean.remap();
	/// # }
	/// ```
	pub fn remap<TInto>(self) -> ElasticBoolean<TInto> where
	TInto: ElasticFieldMapping<()> + ElasticBooleanMapping {
		ElasticBoolean::<TInto>::new(self.value)
	}
}

impl <T> ElasticType<T, ()> for ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping { }

impl From<bool> for ElasticBoolean<DefaultBooleanMapping> {
	fn from(boolean: bool) -> Self {
		ElasticBoolean::new(boolean)
	}
}

impl <T> Into<bool> for ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	fn into(self) -> bool {
		self.value
	}
}

impl<'a, T> PartialEq<bool> for ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	fn eq(&self, other: &bool) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &bool) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, T> PartialEq<ElasticBoolean<T>> for bool where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	fn eq(&self, other: &ElasticBoolean<T>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticBoolean<T>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

//Serialize elastic boolean
impl <T> Serialize for ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_bool(self.value)
	}
}

//Deserialize elastic boolean
impl <T> Deserialize for ElasticBoolean<T> where
T: ElasticFieldMapping<()> + ElasticBooleanMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticBoolean<T>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticBooleanVisitor<T> where
		T: ElasticFieldMapping<()> + ElasticBooleanMapping {
			phantom: PhantomData<T>
		}

		impl <T> serde::de::Visitor for ElasticBooleanVisitor<T> where
		T: ElasticFieldMapping<()> + ElasticBooleanMapping {
			type Value = ElasticBoolean<T>;

			fn visit_bool<E>(&mut self, v: bool) -> Result<ElasticBoolean<T>, E> where
			E: serde::de::Error {
				Ok(ElasticBoolean::<T>::new(v))
			}
		}

		deserializer.deserialize(ElasticBooleanVisitor::<T>::default())
	}
}
