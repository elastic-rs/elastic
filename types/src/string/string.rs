use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticStringType, ElasticStringMapping, DefaultStringMapping };
use ::mapping::{ ElasticTypeMapping, ElasticType };

impl ElasticType<DefaultStringMapping, ()> for String { }
impl ElasticStringType<DefaultStringMapping> for String { }

/// An Elasticsearch `string` with a mapping.
/// 
/// Where the mapping isn't custom, you can use the standard library `String` instead.
/// 
/// # Examples
/// Defining a string with a format:
/// 
/// ```
/// use elastic_types::string::mapping::DefaultStringMapping;
/// use elastic_types::string::ElasticString;
/// 
/// let string = ElasticString::<DefaultStringMapping>::new("my string value");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ElasticString<T: ElasticTypeMapping<()> + ElasticStringMapping> {
	value: String,
	phantom: PhantomData<T>
}
impl <T: ElasticTypeMapping<()> + ElasticStringMapping> ElasticString<T> {
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
	pub fn new<I: Into<String>>(string: I) -> ElasticString<T> {
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
	pub fn set<I: Into<String>>(&mut self, string: I) {
		self.value = string.into()
	}

	/// Change the mapping of this string.
	pub fn into<TInto: ElasticTypeMapping<()> + ElasticStringMapping>(self) -> ElasticString<TInto> {
		ElasticString::<TInto>::new(self.value)
	}
}

impl <T: ElasticTypeMapping<()> + ElasticStringMapping> ElasticType<T, ()> for ElasticString<T> { }
impl <T: ElasticTypeMapping<()> + ElasticStringMapping> ElasticStringType<T> for ElasticString<T> { }

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T: ElasticTypeMapping<()> + ElasticStringMapping> AsRef<str> for ElasticString<T> {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl <T: ElasticTypeMapping<()> + ElasticStringMapping> Into<String> for ElasticString<T> {
	fn into(self) -> String {
		self.value
	}
}

impl<'a, T: ElasticTypeMapping<()> + ElasticStringMapping> PartialEq<String> for ElasticString<T> {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, T: ElasticTypeMapping<()> + ElasticStringMapping> PartialEq<ElasticString<T>> for String {
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, T: ElasticTypeMapping<()> + ElasticStringMapping> PartialEq<&'a str> for ElasticString<T> {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, T: ElasticTypeMapping<()> + ElasticStringMapping> PartialEq<ElasticString<T>> for &'a str {
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}

	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

//Serialize elastic string
impl <T: ElasticTypeMapping<()> + ElasticStringMapping> Serialize for ElasticString<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <T: ElasticTypeMapping<()> + ElasticStringMapping> Deserialize for ElasticString<T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<T>, D::Error> where D: Deserializer {
		#[derive(Default)]
		struct ElasticStringVisitor<T: ElasticTypeMapping<()> + ElasticStringMapping> {
			phantom: PhantomData<T>
		}

		impl <T: ElasticTypeMapping<()> + ElasticStringMapping> serde::de::Visitor for ElasticStringVisitor<T> {
			type Value = ElasticString<T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<T>, E> where E: serde::de::Error {
				Ok(ElasticString::<T>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<T>::default())
	}
}