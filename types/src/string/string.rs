use std::marker::PhantomData;
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
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::ElasticMapping;
	/// use elastic_types::string::{ ElasticString, ElasticStringMapping, DefaultStringMapping };
	/// 
	/// //Define a custom mapping that adjusts the boost value
	/// struct MyStringMapping;
	/// impl ElasticStringMapping for MyStringMapping { }
	/// impl ElasticMapping for MyStringMapping {
	/// 	fn get_boost() -> Option<f32> {
	/// 		Some(2.0)
	/// 	}
	/// }
	/// 
	/// //Use the default format, where boost is given no value.
	/// let string1 = ElasticString::<DefaultStringMapping>::new("my string");
	/// 
	/// let string2: ElasticString<MyStringMapping> = string1.into();
	/// ```
	pub fn into<TInto: ElasticMapping + ElasticStringMapping>(self) -> ElasticString<TInto> {
		ElasticString::<TInto>::new(self.value)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> ElasticType<T> for ElasticString<T> { }
impl <T: ElasticMapping + ElasticStringMapping> ElasticStringType<T> for ElasticString<T> { }

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> Into<String> for ElasticString<T> {
	fn into(self) -> String {
		self.value
	}
}

impl ElasticType<DefaultStringMapping> for String { }
impl ElasticStringType<DefaultStringMapping> for String { }