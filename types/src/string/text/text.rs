use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticTextMapping, DefaultTextMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultTextMapping, ()> for String { }

/// An Elasticsearch `string` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a string with a mapping:
///
/// ```
/// use elastic_types::string::mapping::DefaultTextMapping;
/// use elastic_types::string::ElasticText;
///
/// let string = ElasticText::<DefaultTextMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	value: String,
	phantom: PhantomData<M>
}
impl <M> ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	/// Creates a new `ElasticText` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `ElasticText` from a `String`:
	///
	/// ```
	/// use elastic_types::string::mapping::DefaultTextMapping;
	/// use elastic_types::string::ElasticText;
	///
	/// let string = ElasticText::<DefaultTextMapping>::new(String::from("my string"));
	/// ```
	pub fn new<I>(string: I) -> ElasticText<M> where I: Into<String> {
		ElasticText {
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
	/// Change the mapping for a given `ElasticText`:
	///
	/// ```
	/// # extern crate serde;
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::string::prelude::*;
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyStringMapping;
	/// # impl ElasticTextMapping for MyStringMapping {
	/// # 	fn boost() -> Option<f32> {
	/// #			Some(1.5)
	/// #		}
	/// # }
	/// # impl ElasticFieldMapping<()> for MyStringMapping {
	/// # 	type Visitor = ElasticTextMappingVisitor<MyStringMapping>;
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
	/// let es_string = ElasticText::<DefaultTextMapping>::new(String::from("my string"));
	///
	/// let string: ElasticText<MyStringMapping> = es_string.remap();
	/// # }
	/// ```
	pub fn remap<MInto>(self) -> ElasticText<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticTextMapping {
		ElasticText::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, ()> for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping { }

impl From<String> for ElasticText<DefaultTextMapping> {
	fn from(string: String) -> Self {
		ElasticText::new(string)
	}
}

impl <M> AsRef<str> for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl<'a, M> PartialEq<String> for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<ElasticText<M>> for String where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn eq(&self, other: &ElasticText<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticText<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, M> PartialEq<&'a str> for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value, *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value, *other)
	}
}

impl<'a, M> PartialEq<ElasticText<M>> for &'a str where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn eq(&self, other: &ElasticText<M>) -> bool {
		PartialEq::eq(*self, &other.value)
	}

	fn ne(&self, other: &ElasticText<M>) -> bool {
		PartialEq::ne(*self, &other.value)
	}
}

//Serialize elastic string
impl <M> Serialize for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <M> Deserialize for ElasticText<M> where
M: ElasticFieldMapping<()> + ElasticTextMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticText<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticTextVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticTextMapping {
			phantom: PhantomData<M>
		}

		impl <M> serde::de::Visitor for ElasticTextVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticTextMapping {
			type Value = ElasticText<M>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticText<M>, E> where
			E: serde::de::Error {
				Ok(ElasticText::<M>::new(v))
			}
		}

		deserializer.deserialize(ElasticTextVisitor::<M>::default())
	}
}
