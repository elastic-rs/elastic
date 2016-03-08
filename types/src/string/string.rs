use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticStringType, ElasticStringMapping, StringFormat, DefaultStringMapping };
use ::mapping::{ ElasticMapping, ElasticDataType, TypeEllision, TypeEllisionKind };

impl TypeEllision for String {
	fn get_ellision() -> TypeEllisionKind {
		TypeEllisionKind::Ellided
	}
}

impl <F: StringFormat> ElasticDataType<DefaultStringMapping<F>, F> for String { }
impl <F: StringFormat> ElasticStringType<F, DefaultStringMapping<F>> for String { }

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
pub struct ElasticString<F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> {
	value: String,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}
impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> ElasticString<F, T> {
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
	pub fn new<I: Into<String>>(string: I) -> ElasticString<F, T> {
		ElasticString {
			value: string.into(),
			phantom_f: PhantomData,
			phantom_t: PhantomData
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
	pub fn into<TInto: ElasticMapping<F> + ElasticStringMapping<F>>(self) -> ElasticString<F, TInto> {
		ElasticString::<F, TInto>::new(self.value)
	}
}

impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> TypeEllision for ElasticString<F, T> {
	fn get_ellision() -> TypeEllisionKind {
		TypeEllisionKind::Ellided
	}
}

impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> ElasticDataType<T, F> for ElasticString<F, T> { }
impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> ElasticStringType<F, T> for ElasticString<F, T> { }

impl <F: StringFormat> From<String> for ElasticString<F, DefaultStringMapping<F>> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> AsRef<str> for ElasticString<F, T> {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> Into<String> for ElasticString<F, T> {
	fn into(self) -> String {
		self.value
	}
}

impl<'a, F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> PartialEq<String> for ElasticString<F, T> {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> PartialEq<ElasticString<F, T>> for String {
	fn eq(&self, other: &ElasticString<F, T>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticString<F, T>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> PartialEq<&'a str> for ElasticString<F, T> {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> PartialEq<ElasticString<F, T>> for &'a str {
	fn eq(&self, other: &ElasticString<F, T>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}

	fn ne(&self, other: &ElasticString<F, T>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

//Serialize elastic string
impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> Serialize for ElasticString<F, T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> Deserialize for ElasticString<F, T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<F, T>, D::Error> where D: Deserializer {
		#[derive(Default)]
		struct ElasticStringVisitor<F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> {
			phantom_f: PhantomData<F>,
			phantom_t: PhantomData<T>
		}

		impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> serde::de::Visitor for ElasticStringVisitor<F, T> {
			type Value = ElasticString<F, T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<F, T>, E> where E: serde::de::Error {
				Ok(ElasticString::<F, T>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<F, T>::default())
	}
}