use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use serde::de::{ Visitor, Error };
use super::mapping::{ ElasticBooleanMapping, DefaultBooleanMapping, ElasticBooleanFormat };
use ::mapping::{ ElasticType };

impl ElasticType<DefaultBooleanMapping, ElasticBooleanFormat> for bool { }

/// An Elasticsearch `boolean` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `bool` instead.
///
/// # Examples
///
/// Defining a `bool` with a mapping:
///
/// ```
/// use elastic_types::boolean::mapping::DefaultBooleanMapping;
/// use elastic_types::boolean::ElasticBoolean;
///
/// let boolean = ElasticBoolean::<DefaultBooleanMapping>::new(true);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElasticBoolean<M> where
M: ElasticBooleanMapping {
	value: bool,
	_m: PhantomData<M>
}
impl <M> ElasticBoolean<M> where
M: ElasticBooleanMapping {
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
	pub fn new<I>(boolean: I) -> ElasticBoolean<M> where I: Into<bool> {
		ElasticBoolean {
			value: boolean.into(),
			_m: PhantomData
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
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::prelude::*;
	/// # boolean_mapping!(MyBooleanMapping {
	/// # 	fn boost() -> Option<f32> {
	///	# 		Some(1.5)
	///	# 	}
	/// # });
	/// let es_boolean = ElasticBoolean::<DefaultBooleanMapping>::new(true);
	///
	/// let boolean: ElasticBoolean<MyBooleanMapping> = es_boolean.remap();
	/// # }
	/// ```
	pub fn remap<MInto>(self) -> ElasticBoolean<MInto> where
	MInto: ElasticBooleanMapping {
		ElasticBoolean::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, ElasticBooleanFormat> for ElasticBoolean<M> where
M: ElasticBooleanMapping { }

impl From<bool> for ElasticBoolean<DefaultBooleanMapping> {
	fn from(boolean: bool) -> Self {
		ElasticBoolean::new(boolean)
	}
}

impl<'a, M> PartialEq<bool> for ElasticBoolean<M> where
M: ElasticBooleanMapping {
	fn eq(&self, other: &bool) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &bool) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<ElasticBoolean<M>> for bool where
M: ElasticBooleanMapping {
	fn eq(&self, other: &ElasticBoolean<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticBoolean<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

//Serialize elastic boolean
impl <M> Serialize for ElasticBoolean<M> where
M: ElasticBooleanMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_bool(self.value)
	}
}

//Deserialize elastic boolean
impl <M> Deserialize for ElasticBoolean<M> where
M: ElasticBooleanMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticBoolean<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticBooleanVisitor<M> where
		M: ElasticBooleanMapping {
			_m: PhantomData<M>
		}

		impl <M> Visitor for ElasticBooleanVisitor<M> where
		M: ElasticBooleanMapping {
			type Value = ElasticBoolean<M>;

			fn visit_bool<E>(&mut self, v: bool) -> Result<ElasticBoolean<M>, E> where
			E: Error {
				Ok(ElasticBoolean::<M>::new(v))
			}
		}

		deserializer.deserialize(ElasticBooleanVisitor::<M>::default())
	}
}
