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
pub struct ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	value: String,
	phantom: PhantomData<M>
}
impl <M> ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
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
	pub fn new<I>(string: I) -> ElasticString<M> where I: Into<String> {
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
	pub fn remap<MInto>(self) -> ElasticString<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticStringMapping {
		ElasticString::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, ()> for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping { }

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <M> AsRef<str> for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl<'a, M> PartialEq<String> for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<ElasticString<M>> for String where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &ElasticString<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticString<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, M> PartialEq<&'a str> for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, M> PartialEq<ElasticString<M>> for &'a str where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn eq(&self, other: &ElasticString<M>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}

	fn ne(&self, other: &ElasticString<M>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

//Serialize elastic string
impl <M> Serialize for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <M> Deserialize for ElasticString<M> where
M: ElasticFieldMapping<()> + ElasticStringMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticStringVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticStringMapping {
			phantom: PhantomData<M>
		}

		impl <M> serde::de::Visitor for ElasticStringVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticStringMapping {
			type Value = ElasticString<M>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<M>, E> where
			E: serde::de::Error {
				Ok(ElasticString::<M>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<M>::default())
	}
}
