use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticKeywordMapping, DefaultKeywordMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultKeywordMapping, ()> for String { }

/// An Elasticsearch `string` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a string with a mapping:
///
/// ```
/// use elastic_types::string::mapping::DefaultKeywordMapping;
/// use elastic_types::string::ElasticKeyword;
///
/// let string = ElasticKeyword::<DefaultKeywordMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	value: String,
	phantom: PhantomData<M>
}
impl <M> ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	/// Creates a new `ElasticKeyword` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `ElasticKeyword` from a `String`:
	///
	/// ```
	/// use elastic_types::string::mapping::DefaultKeywordMapping;
	/// use elastic_types::string::ElasticKeyword;
	///
	/// let string = ElasticKeyword::<DefaultKeywordMapping>::new(String::from("my string"));
	/// ```
	pub fn new<I>(string: I) -> ElasticKeyword<M> where I: Into<String> {
		ElasticKeyword {
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
	/// Change the mapping for a given `ElasticKeyword`:
	///
	/// ```
	/// # extern crate serde;
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::string::prelude::*;
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyStringMapping;
	/// # impl ElasticKeywordMapping for MyStringMapping {
	/// # 	fn boost() -> Option<f32> {
	/// #			Some(1.5)
	/// #		}
	/// # }
	/// # impl ElasticFieldMapping<()> for MyStringMapping {
	/// # 	type Visitor = ElasticKeywordMappingVisitor<MyStringMapping>;
	/// # 	fn data_type() -> &'static str {
	/// # 		STRING_DATATYPE
	/// # 	}
	/// # }
	/// # impl serde::Serialize for MyStringMapping {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	/// # 	where S: serde::Serializer {
	/// # 		serializer.serialize_struct("mapping", Self::get_visitor())
	/// # 	}
	/// # }
	/// let es_string = ElasticKeyword::<DefaultKeywordMapping>::new(String::from("my string"));
	///
	/// let string: ElasticKeyword<MyStringMapping> = es_string.remap();
	/// # }
	/// ```
	pub fn remap<MInto>(self) -> ElasticKeyword<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticKeywordMapping {
		ElasticKeyword::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, ()> for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping { }

impl From<String> for ElasticKeyword<DefaultKeywordMapping> {
	fn from(string: String) -> Self {
		ElasticKeyword::new(string)
	}
}

impl <M> AsRef<str> for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl<'a, M> PartialEq<String> for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<ElasticKeyword<M>> for String where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn eq(&self, other: &ElasticKeyword<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticKeyword<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, M> PartialEq<&'a str> for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value, *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value, *other)
	}
}

impl<'a, M> PartialEq<ElasticKeyword<M>> for &'a str where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn eq(&self, other: &ElasticKeyword<M>) -> bool {
		PartialEq::eq(*self, &other.value)
	}

	fn ne(&self, other: &ElasticKeyword<M>) -> bool {
		PartialEq::ne(*self, &other.value)
	}
}

//Serialize elastic string
impl <M> Serialize for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <M> Deserialize for ElasticKeyword<M> where
M: ElasticFieldMapping<()> + ElasticKeywordMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticKeyword<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticKeywordVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticKeywordMapping {
			phantom: PhantomData<M>
		}

		impl <M> serde::de::Visitor for ElasticKeywordVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticKeywordMapping {
			type Value = ElasticKeyword<M>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticKeyword<M>, E> where
			E: serde::de::Error {
				Ok(ElasticKeyword::<M>::new(v))
			}
		}

		deserializer.deserialize(ElasticKeywordVisitor::<M>::default())
	}
}
