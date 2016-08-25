use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use serde::de::{ Visitor, Error };
use super::mapping::{ BooleanMapping, DefaultBooleanMapping, BooleanFormat };
use ::mapping::ElasticType;

impl ElasticType<DefaultBooleanMapping, BooleanFormat> for bool { }

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
/// use elastic_types::boolean::Boolean;
///
/// let boolean = Boolean::<DefaultBooleanMapping>::new(true);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Boolean<M> where
M: BooleanMapping {
	value: bool,
	_m: PhantomData<M>
}
impl <M> Boolean<M> where
M: BooleanMapping {
	/// Creates a new `Boolean` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `Boolean` from a `bool`:
	///
	/// ```
	/// use elastic_types::boolean::mapping::DefaultBooleanMapping;
	/// use elastic_types::boolean::Boolean;
	///
	/// let boolean = Boolean::<DefaultBooleanMapping>::new(false);
	/// ```
	pub fn new<I>(boolean: I) -> Boolean<M> where I: Into<bool> {
		Boolean {
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
	/// Change the mapping for a given `Boolean`:
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
	/// let es_boolean = Boolean::<DefaultBooleanMapping>::new(true);
	///
	/// let boolean: Boolean<MyBooleanMapping> = es_boolean.remap();
	/// # }
	/// ```
	pub fn remap<MInto>(self) -> Boolean<MInto> where
	MInto: BooleanMapping {
		Boolean::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, BooleanFormat> for Boolean<M> where
M: BooleanMapping { }

impl From<bool> for Boolean<DefaultBooleanMapping> {
	fn from(boolean: bool) -> Self {
		Boolean::new(boolean)
	}
}

impl<'a, M> PartialEq<bool> for Boolean<M> where
M: BooleanMapping {
	fn eq(&self, other: &bool) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &bool) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<Boolean<M>> for bool where
M: BooleanMapping {
	fn eq(&self, other: &Boolean<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &Boolean<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

//Serialize elastic boolean
impl <M> Serialize for Boolean<M> where
M: BooleanMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_bool(self.value)
	}
}

//Deserialize elastic boolean
impl <M> Deserialize for Boolean<M> where
M: BooleanMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<Boolean<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct BooleanVisitor<M> where
		M: BooleanMapping {
			_m: PhantomData<M>
		}

		impl <M> Visitor for BooleanVisitor<M> where
		M: BooleanMapping {
			type Value = Boolean<M>;

			fn visit_bool<E>(&mut self, v: bool) -> Result<Boolean<M>, E> where
			E: Error {
				Ok(Boolean::<M>::new(v))
			}
		}

		deserializer.deserialize(BooleanVisitor::<M>::default())
	}
}
