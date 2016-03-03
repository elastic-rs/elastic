use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticStringType, ElasticStringMapping, DefaultStringMapping };
use ::mapping::{ ElasticMapping, ElasticType };

/// An Elasticsearch `string` with a mapping.
/// 
/// Where the mapping isn't custom, you can use the standard library `String` instead.
/// 
/// # Examples
/// Defining a string with a format:
/// 
/// ```
/// use elastic_types::string::{ ElasticString, DefaultStringMapping };
/// 
/// let string = ElasticString::<DefaultStringMapping>::new("my string value");
/// ```
#[derive(Debug, Clone)]
pub struct ElasticString<T: ElasticMapping + ElasticStringMapping> {
	value: String,
	phantom: PhantomData<T>
}
impl <T: ElasticMapping + ElasticStringMapping> ElasticString<T> {
	/// Creates a new `ElasticString` with the given mapping.
	/// 
	/// # Examples
	/// 
	/// Create a new `ElasticString` from a `String`:
	/// 
	/// ```
	/// use elastic_types::string::{ ElasticString, DefaultStringMapping };
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
	pub fn get<'a>(&'a self) -> &'a str {
		&self.value
	}

	/// Set the value of the string.
	pub fn set<I: Into<String>>(&mut self, string: I) {
		self.value = string.into()
	}

	/// Change the mapping of this string.
	pub fn into<TInto: ElasticMapping + ElasticStringMapping>(self) -> ElasticString<TInto> {
		ElasticString::<TInto>::new(self.value)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> ElasticType<T, ()> for ElasticString<T> { }
impl <T: ElasticMapping + ElasticStringMapping> ElasticStringType<T> for ElasticString<T> { }

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> AsRef<str> for ElasticString<T> {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl <T: ElasticMapping + ElasticStringMapping> Into<String> for ElasticString<T> {
	fn into(self) -> String {
		self.value
	}
}

impl<'a, T: ElasticMapping + ElasticStringMapping> PartialEq<String> for ElasticString<T> {
	#[inline(always)]
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}
	#[inline(always)]
	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, T: ElasticMapping + ElasticStringMapping> PartialEq<ElasticString<T>> for String {
	#[inline(always)]
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(self, &other.value)
	}
	#[inline(always)]
	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, T: ElasticMapping + ElasticStringMapping> PartialEq<&'a str> for ElasticString<T> {
	#[inline(always)]
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}
	#[inline(always)]
	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, T: ElasticMapping + ElasticStringMapping> PartialEq<ElasticString<T>> for &'a str {
	#[inline(always)]
	fn eq(&self, other: &ElasticString<T>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}
	#[inline(always)]
	fn ne(&self, other: &ElasticString<T>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

impl ElasticType<DefaultStringMapping, ()> for String { }
impl ElasticStringType<DefaultStringMapping> for String { }

//Serialize elastic string
impl <T: ElasticMapping + ElasticStringMapping> Serialize for ElasticString<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <T: ElasticMapping + ElasticStringMapping> Deserialize for ElasticString<T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<T>, D::Error> where D: Deserializer {
		struct ElasticStringVisitor<T: ElasticMapping + ElasticStringMapping> {
			phantom: PhantomData<T>
		}

		impl <T: ElasticMapping + ElasticStringMapping> Default for ElasticStringVisitor<T> {
			fn default() -> ElasticStringVisitor<T> {
				ElasticStringVisitor::<T> {
					phantom: PhantomData
				}
			}
		}

		impl <T: ElasticMapping + ElasticStringMapping> serde::de::Visitor for ElasticStringVisitor<T> {
			type Value = ElasticString<T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<T>, E> where E: serde::de::Error {
				Ok(ElasticString::<T>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<T>::default())
	}
}