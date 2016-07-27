use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticTextMapping, DefaultTextMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultTextMapping, ()> for String { }

/// An Elasticsearch `text` field with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `String` instead.
///
/// # Examples
///
/// Defining a `text` field with a mapping:
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
	/// let string = ElasticText::<DefaultTextMapping>::new("my string");
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
	pub fn remap<MInto>(self) -> ElasticText<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticTextMapping {
		ElasticText::<MInto>::new(self.value)
	}
}

impl_string_type!(ElasticText, ElasticTextMapping, DefaultTextMapping);