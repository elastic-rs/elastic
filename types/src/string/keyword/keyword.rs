use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticKeywordMapping, DefaultKeywordMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

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
	/// let string = ElasticKeyword::<DefaultKeywordMapping>::new("my string");
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
	pub fn remap<MInto>(self) -> ElasticKeyword<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticKeywordMapping {
		ElasticKeyword::<MInto>::new(self.value)
	}
}

impl_string_type!(ElasticKeyword, ElasticKeywordMapping, DefaultKeywordMapping);