use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticStringType, ElasticStringMapping, StringFormat, DefaultStringMapping, DefaultStringFormat };
use ::mapping::{ ElasticMapping, ElasticDataType, TypeEllision, TypeEllisionKind };

impl TypeEllision for String {
	fn get_ellision() -> TypeEllisionKind {
		TypeEllisionKind::Ellided
	}
}

impl ElasticDataType<DefaultStringMapping<DefaultStringFormat>, DefaultStringFormat> for String { }
impl ElasticStringType<DefaultStringMapping<DefaultStringFormat>, DefaultStringFormat> for String { }

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
pub struct ElasticString<T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat = DefaultStringFormat> {
	value: String,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}
impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> ElasticString<T, F> {
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
	pub fn new<I: Into<String>>(string: I) -> ElasticString<T, F> {
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
	pub fn into<TInto: ElasticMapping<F> + ElasticStringMapping<F>>(self) -> ElasticString<TInto, F> {
		ElasticString::<TInto, F>::new(self.value)
	}
}

impl <F: StringFormat, T: ElasticMapping<F> + ElasticStringMapping<F>> TypeEllision for ElasticString<T, F> {
	fn get_ellision() -> TypeEllisionKind {
		TypeEllisionKind::Ellided
	}
}

impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> ElasticDataType<T, F> for ElasticString<T, F> { }
impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> ElasticStringType<T, F> for ElasticString<T, F> { }

impl <F: StringFormat> From<String> for ElasticString<DefaultStringMapping<F>, F> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> AsRef<str> for ElasticString<T, F> {
	fn as_ref(&self) -> &str {
		&self.value
	}
}

impl <T: ElasticMapping<DefaultStringFormat> + ElasticStringMapping<DefaultStringFormat>> Into<String> for ElasticString<T, DefaultStringFormat> {
	fn into(self) -> String {
		self.value
	}
}

impl<'a, T: ElasticMapping<DefaultStringFormat> + ElasticStringMapping<DefaultStringFormat>> PartialEq<String> for ElasticString<T, DefaultStringFormat> {
	fn eq(&self, other: &String) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &String) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, T: ElasticMapping<DefaultStringFormat> + ElasticStringMapping<DefaultStringFormat>> PartialEq<ElasticString<T, DefaultStringFormat>> for String {
	fn eq(&self, other: &ElasticString<T, DefaultStringFormat>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticString<T, DefaultStringFormat>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl<'a, T: ElasticMapping<DefaultStringFormat> + ElasticStringMapping<DefaultStringFormat>> PartialEq<&'a str> for ElasticString<T, > {
	fn eq(&self, other: & &'a str) -> bool {
		PartialEq::eq(&self.value[..], *other)
	}

	fn ne(&self, other: & &'a str) -> bool {
		PartialEq::ne(&self.value[..], *other)
	}
}

impl<'a, T: ElasticMapping<DefaultStringFormat> + ElasticStringMapping<DefaultStringFormat>> PartialEq<ElasticString<T, DefaultStringFormat>> for &'a str {
	fn eq(&self, other: &ElasticString<T, DefaultStringFormat>) -> bool {
		PartialEq::eq(*self, &other.value[..])
	}

	fn ne(&self, other: &ElasticString<T, DefaultStringFormat>) -> bool {
		PartialEq::ne(*self, &other.value[..])
	}
}

//Serialize elastic string
impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> Serialize for ElasticString<T, F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.value)
	}
}

//Deserialize elastic string
impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> Deserialize for ElasticString<T, F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticString<T, F>, D::Error> where D: Deserializer {
		#[derive(Default)]
		struct ElasticStringVisitor<T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> {
			phantom_f: PhantomData<F>,
			phantom_t: PhantomData<T>
		}

		impl <T: ElasticMapping<F> + ElasticStringMapping<F>, F: StringFormat> serde::de::Visitor for ElasticStringVisitor<T, F> {
			type Value = ElasticString<T, F>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticString<T, F>, E> where E: serde::de::Error {
				Ok(ElasticString::<T, F>::new(v))
			}
		}

		deserializer.deserialize(ElasticStringVisitor::<T, F>::default())
	}
}