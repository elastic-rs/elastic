use std::marker::PhantomData;
use chrono;
use chrono::UTC;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::Format;
use super::format::BasicDateTime;

/// A re-export of the `chrono::DateTime` struct with `UTC` timezone
pub type DT = chrono::DateTime<UTC>;
pub use chrono::{ Datelike, Timelike };

/// An Elasticsearch `date` type with a required `time` component. 
/// 
/// The [format](format/index.html) is provided as a generic parameter.
/// This struct wraps up a `chrono::DateTime<UTC>` struct, meaning storing time in `UTC` is required.
///
/// # Examples
/// Defining a date using the default format:
/// 
/// ```
/// use elastic_types::date::DateTime;
/// 
/// let date: DateTime = DateTime::now();
/// ```
/// 
/// Defining a date using a named format:
/// 
/// ```
/// use elastic_types::date::DateTime;
/// use elastic_types::date::format::BasicDateTime;
/// 
/// let date = DateTime::<BasicDateTime>::now();
/// ```
/// 
/// Accessing the values of a date:
/// 
/// ```
/// use elastic_types::date::{ DateTime, Datelike, Timelike };
/// use elastic_types::date::format::BasicDateTime;
/// 
/// let date = DateTime::<BasicDateTime>::now();
/// 
/// //eg: 2010/04/30 13:56
/// println!("{}/{}/{} {}:{}", 
///	date.value.year(), 
/// 	date.value.month(), 
/// 	date.value.day(), 
/// 	date.value.hour(), 
/// 	date.value.minute()
/// );
/// ```
/// For a full list of available date and time functions on `date.value` see [Datelike](trait.Datelike.html) and [Timelike](trait.Timelike.html).
/// 
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Clone)]
pub struct DateTime<T: Format = BasicDateTime> {
	/// The date and time value
	pub value: DT,
	phantom: PhantomData<T>
}

impl <T: Format> DateTime<T> {
	/// Creates a new `DateTime` from the given `chrono::DateTime<UTC>`.
	/// 
	/// This function will consume the provided `chrono` date.
	/// 
	/// # Examples
	/// 
	/// Create a `DateTime` from the current date:
	/// 
	/// ```
	/// # extern crate elastic_types;
	/// # extern crate chrono;
	/// # fn main() {
	/// use chrono::UTC;
	/// use elastic_types::date::DateTime;
	/// use elastic_types::date::format::BasicDateTime;
	/// 
	/// //Create a chrono DateTime struct
	/// let chronoDate = UTC::now();
	/// 
	/// //Give it to the elastic DateTime struct
	/// let esDate = DateTime::<BasicDateTime>::new(chronoDate);
	/// # }
	/// ```
	pub fn new(date: DT) -> DateTime<T> {
		DateTime {
			value: date,
			phantom: PhantomData
		}
	}

	pub fn now() -> DateTime<T> {
		DateTime::<T>::default()
	}

	pub fn parse(date: &str) -> Result<DateTime<T>, String> {
		T::parse(date).map(|r| DateTime::new(r))
	}
}

impl <T: Format> ToString for DateTime<T> {
	fn to_string(&self) -> String {
		//Get the format. We want the first item in the list
		let fmts = T::fmt();
		let fmt = &fmts
			.iter()
			.next()
			.unwrap();

		self.value.format(fmt).to_string()
	}
}

impl <T: Format> Default for DateTime<T> {
	fn default() -> DateTime<T> {
		DateTime {
			value: chrono::UTC::now(),
			phantom: PhantomData
		}
	}
}

//Serialize
impl <T: Format> Serialize for DateTime<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
    {
        serializer.visit_str(&self.to_string()[..])
    }
}

//Deserialize
impl <T: Format> Deserialize for DateTime<T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<DateTime<T>, D::Error> where D: Deserializer,
    {
        deserializer.visit_str(DateTimeVisitor::<T>::default())
    }
}

struct DateTimeVisitor<T: Format> {
	phantom: PhantomData<T>
}

impl <T: Format> Default for DateTimeVisitor<T> {
	fn default() -> DateTimeVisitor<T> {
		DateTimeVisitor::<T> {
			phantom: PhantomData
		}
	}
}

impl <T: Format> serde::de::Visitor for DateTimeVisitor<T> {
	type Value = DateTime<T>;

	fn visit_str<E>(&mut self, v: &str) -> Result<DateTime<T>, E> where E: serde::de::Error {
		let result = DateTime::<T>::parse(v);
		result.map_err(|err| E::syntax(&format!("{}", err)))
	}
}
