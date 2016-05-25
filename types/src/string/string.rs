use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticStringMapping, DefaultStringMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultStringMapping, ()> for String { }

/// An Elasticsearch `string` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a string with a mapping:
///
/// ```
/// use elastic_types::string::mapping::DefaultStringMapping;
/// use elastic_types::string::ElasticString;
///
/// let string = ElasticString::<DefaultStringMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	value: String,
	phantom: PhantomData<T>
}
impl <T> ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	/// Creates a new `ElasticString` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `ElasticString` from a `String`:
	///
	/// ```
	/// use elastic_types::string::mapping::DefaultStringMapping;
	/// use elastic_types::string::ElasticString;
	///
	/// let string = ElasticString::<DefaultStringMapping>::new(String::from("my string"));
	/// ```
	pub fn new<I>(string: I) -> ElasticString<T> where I: Into<String> {
		ElasticString {
			value: string.into(),
			phantom: PhantomData
		}
	}

	/// Get the value of the string.
	pub fn get(&self) -> &str {
		&self.value
	}

	/// Set the value of the string.
	pub fn set<I>(&mut self, string: I) where I: Into<String> {
		self.value = string.into()
	}

	/// Change the mapping of this string.
	///
	/// # Examples
	///
	/// Change the mapping for a given `ElasticString`:
	///
	/// ```
	/// # extern crate serde;
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::string::prelude::*;
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyStringMapping;
	/// # impl ElasticStringMapping for MyStringMapping {
	/// # 	fn boost() -> Option<f32> {
	/// #			Some(1.5)
	/// #		}
	/// # }
	/// # impl ElasticFieldMapping<()> for MyStringMapping {
	/// # 	type Visitor = ElasticStringMappingVisitor<MyStringMapping>;
	/// # 	type MultiFieldMapping = Self;
	/// # 	fn data_type() -> &'static str {
	/// # 		"string"
	/// # 	}
	/// # }
	/// # impl serde::Serialize for MyStringMapping {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	/// # 	where S: serde::Serializer {
	/// # 		serializer.serialize_struct("mapping", Self::get_visitor())
	/// # 	}
	/// # }
	/// let es_string = ElasticString::<DefaultStringMapping>::new(String::from("my string"));
	///
	/// let string: ElasticString<MyStringMapping> = es_string.remap();
	/// # }
	/// ```
	pub fn remap<TInto>(self) -> ElasticString<TInto> where
	TInto: ElasticFieldMapping<()> + ElasticStringMapping {
		ElasticString::<TInto>::new(self.value)
	}
}

impl <T> ElasticType<T, ()> for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping { }

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T> AsRef<str> for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl<'a, T> PartialEq<String> for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, T> PartialEq<ElasticString<T>> for String where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, T> PartialEq<&'a str> for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, T> PartialEq<ElasticString<T>> for &'a str where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}

	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

//Serialize elastic string
impl <T> Serialize for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <T> Deserialize for ElasticString<T> where
T: ElasticFieldMapping<()> + ElasticStringMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<T>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticStringVisitor<T> where
		T: ElasticFieldMapping<()> + ElasticStringMapping {
			phantom: PhantomData<T>
		}

		impl <T> serde::de::Visitor for ElasticStringVisitor<T> where
		T: ElasticFieldMapping<()> + ElasticStringMapping {
			type Value = ElasticString<T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<T>, E> where
			E: serde::de::Error {
				Ok(ElasticString::<T>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<T>::default())
	}
}
