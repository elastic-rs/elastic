use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use serde::de::{ Visitor, Error };
use super::mapping::{ KeywordMapping, KeywordFormat };
use ::mapping::ElasticType;

/// An Elasticsearch `keyword` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a `keyword` with a mapping:
///
/// ```
/// use elastic_types::string::mapping::DefaultKeywordMapping;
/// use elastic_types::string::Keyword;
///
/// let string = Keyword::<DefaultKeywordMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Keyword<M> where
M: KeywordMapping {
	value: String,
	_m: PhantomData<M>
}
impl <M> Keyword<M> where
M: KeywordMapping {
	/// Creates a new `Keyword` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `Keyword` from a `String`:
	///
	/// ```
	/// use elastic_types::string::mapping::DefaultKeywordMapping;
	/// use elastic_types::string::Keyword;
	///
	/// let string = Keyword::<DefaultKeywordMapping>::new("my string");
	/// ```
	pub fn new<I>(string: I) -> Keyword<M> where I: Into<String> {
		Keyword {
			value: string.into(),
			_m: PhantomData
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
	pub fn remap<MInto>(self) -> Keyword<MInto> where
	MInto: KeywordMapping {
		Keyword::<MInto>::new(self.value)
	}
}

impl_string_type!(Keyword, KeywordMapping, KeywordFormat);