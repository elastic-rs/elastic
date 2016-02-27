use std::marker::PhantomData;
use chrono;
use chrono::{ UTC, Weekday };
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::{ Format, ParseError };
use super::BasicDateTime;

/// A re-export of the `chrono::DateTime` struct with `UTC` timezone.
pub type DT = chrono::DateTime<UTC>;
pub use chrono::{ Datelike, Timelike };

/// The default DateTime format.
pub type DefaultFormat = BasicDateTime;

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
/// use elastic_types::date::{ DateTime, BasicDateTime };
/// 
/// let date = DateTime::<BasicDateTime>::now();
/// ```
/// 
/// Accessing the values of a date:
/// 
/// ```
/// use elastic_types::date::{ DateTime, Datelike, Timelike, BasicDateTime };
/// 
/// let date = DateTime::<BasicDateTime>::now();
/// 
/// //eg: 2010/04/30 13:56:59.372
/// println!("{}/{}/{} {}:{}:{}.{}", 
///		date.year(), 
/// 	date.month(), 
/// 	date.day(), 
/// 	date.hour(), 
/// 	date.minute(),
/// 	date.second(), 
/// 	date.nanosecond() / 1000000
/// );
/// ```
/// 
/// For a full list of available date and time functions on `date` see [Datelike](trait.Datelike.html) and [Timelike](trait.Timelike.html).
/// 
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Clone)]
pub struct DateTime<T: Format = DefaultFormat> {
	/// The date and time value
	value: DT,
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
	/// use elastic_types::date::*;
	/// 
	/// //Create a chrono DateTime struct
	/// let chronoDate = UTC::now();
	/// 
	/// //Give it to the elastic DateTime struct
	/// let esDate: DateTime = DateTime::new(chronoDate);
	/// # }
	/// ```
	pub fn new(date: DT) -> DateTime<T> {
		DateTime {
			value: date,
			phantom: PhantomData
		}
	}

	/// Gets the current system time.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::date::*;
	/// 
	/// let date: DateTime = DateTime::now();
	/// ```
	pub fn now() -> DateTime<T> {
		DateTime {
			value: chrono::UTC::now(),
			phantom: PhantomData
		}
	}

	/// Parse the date and time from a string.
	/// 
	/// The format of the string must match the given `Format`.
	/// 
	/// # Examples
	/// 
	/// Parsing from a specified `Format`.
	/// 
	/// ```
	/// use elastic_types::date::{ DateTime, BasicDateTime };
	/// 
	/// let date = DateTime::<BasicDateTime>::parse("20151126T145543.778Z").unwrap();
	/// ```
	pub fn parse(date: &str) -> Result<DateTime<T>, ParseError> {
		T::parse(date).map(|r| DateTime::new(r))
	}

	/// Format the date and time as a string.
	/// 
	/// The format of the string is specified by the given `Format`.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::date::{ DateTime, BasicDateTime };
	/// 
	/// let date: DateTime = DateTime::now();
	/// let fmt = date.format();
	/// 
	/// //eg: 20151126T145543.778Z
	/// println!("{}", fmt);
	/// ```
	pub fn format<'a>(&self) -> String {
		T::format(&self.value)
	}

	/// Change the format of this date.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::date::{ DateTime, BasicDateTime, EpochMillis };
	/// 
	/// //Get the current datetime formatted as basic_date_time
	/// let date: DateTime<BasicDateTime> = DateTime::now();
	/// 
	/// //Change the format to epoch_millis
	/// let otherdate: DateTime<EpochMillis> = date.into();
	/// ```
	pub fn into<TInto: Format>(self) -> DateTime<TInto> {
		DateTime::<TInto>::new(self.value)
	}
}

impl <T: Format> Default for DateTime<T> {
	fn default() -> DateTime<T> {
		DateTime::<T>::now()
	}
}

impl <T: Format> From<DT> for DateTime<T> {
	fn from(dt: DT) -> DateTime<T> {
		DateTime::<T>::new(dt)
	}
}

impl <T: Format> Datelike for DateTime<T> {
	fn year(&self) -> i32 { self.value.year() }
	fn month(&self) -> u32 { self.value.month() }
	fn month0(&self) -> u32 { self.value.month0() }
	fn day(&self) -> u32 { self.value.day() }
	fn day0(&self) -> u32 { self.value.day0() }
	fn ordinal(&self) -> u32 { self.value.ordinal() }
	fn ordinal0(&self) -> u32 { self.value.ordinal0() }
	fn weekday(&self) -> Weekday { self.value.weekday() }
	fn isoweekdate(&self) -> (i32, u32, Weekday) { self.value.isoweekdate() }

	fn with_year(&self, year: i32) -> Option<DateTime<T>> {
		match self.value.with_year(year) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_month(&self, month: u32) -> Option<DateTime<T>> {
		match self.value.with_month(month) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_month0(&self, month0: u32) -> Option<DateTime<T>> {
		match self.value.with_month0(month0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_day(&self, day: u32) -> Option<DateTime<T>> {
		match self.value.with_day(day) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_day0(&self, day0: u32) -> Option<DateTime<T>> {
		match self.value.with_day0(day0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_ordinal(&self, ordinal: u32) -> Option<DateTime<T>> {
		match self.value.with_ordinal(ordinal) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTime<T>> {
		match self.value.with_ordinal0(ordinal0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}
}

impl <T: Format> Timelike for DateTime<T> {
	fn hour(&self) -> u32 { self.value.hour() }
	fn minute(&self) -> u32 { self.value.minute() }
	fn second(&self) -> u32 { self.value.second() }
	fn nanosecond(&self) -> u32 { self.value.nanosecond() }

	fn with_hour(&self, hour: u32) -> Option<DateTime<T>> {
		match self.value.with_hour(hour) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_minute(&self, min: u32) -> Option<DateTime<T>> {
		match self.value.with_minute(min) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_second(&self, sec: u32) -> Option<DateTime<T>> {
		match self.value.with_second(sec) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_nanosecond(&self, nano: u32) -> Option<DateTime<T>> {
		match self.value.with_nanosecond(nano) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}
}

//Serialize
impl <T: Format> Serialize for DateTime<T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.format()[..])
	}
}

//Deserialize
impl <T: Format> Deserialize for DateTime<T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<DateTime<T>, D::Error> where D: Deserializer,
	{
		deserializer.deserialize(DateTimeVisitor::<T>::default())
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
		result.map_err(|err| serde::de::Error::custom(format!("{}", err)))
	}
}