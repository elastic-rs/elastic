use std::marker::PhantomData;
use chrono;
use chrono::Weekday;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::{ DT, Format, ParseError, ElasticDateMapping, ElasticDateType, DefaultDateMapping, DefaultFormat };
use ::mapping::{ ElasticMapping, ElasticType };

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
/// use elastic_types::date::{ DateTime, BasicDateTime };
/// 
/// let date = DateTime::<BasicDateTime>::now();
/// ```
/// 
/// Defining a date using a custom mapping:
/// 
/// ```
/// use elastic_types::date::{ DateTime, BasicDateTime, DefaultDateMapping };
/// 
/// let date = DateTime::<BasicDateTime, DefaultDateMapping>::now();
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
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Debug, Clone)]
pub struct DateTime<F: Format = DefaultFormat, T: ElasticMapping<F> + ElasticDateMapping<F> = DefaultDateMapping<F>> {
	/// The date and time value
	value: DT,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> DateTime<F, T> {
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
	pub fn new(date: DT) -> DateTime<F, T> {
		DateTime {
			value: date,
			phantom_f: PhantomData,
			phantom_t: PhantomData
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
	pub fn now() -> DateTime<F, T> {
		DateTime {
			value: chrono::UTC::now(),
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}

	/// Change the format/mapping of this date.
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
	pub fn into<FInto: Format, TInto: ElasticMapping<FInto> + ElasticDateMapping<FInto>>(self) -> DateTime<FInto, TInto> {
		DateTime::<FInto, TInto>::new(self.value)
	}
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> ElasticType<T, F> for DateTime<F, T> {

}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> ElasticDateType<F, T> for DateTime<F, T> {
	fn parse(date: &str) -> Result<DateTime<F, T>, ParseError> {
		F::parse(date).map(|r| DateTime::new(r))
	}

	fn format<'a>(&self) -> String {
		F::format(&self.value)
	}
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Default for DateTime<F, T> {
	fn default() -> DateTime<F, T> {
		DateTime::<F, T>::now()
	}
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> From<DT> for DateTime<F, T> {
	fn from(dt: DT) -> DateTime<F, T> {
		DateTime::<F, T>::new(dt)
	}
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Datelike for DateTime<F, T> {
	fn year(&self) -> i32 { self.value.year() }
	fn month(&self) -> u32 { self.value.month() }
	fn month0(&self) -> u32 { self.value.month0() }
	fn day(&self) -> u32 { self.value.day() }
	fn day0(&self) -> u32 { self.value.day0() }
	fn ordinal(&self) -> u32 { self.value.ordinal() }
	fn ordinal0(&self) -> u32 { self.value.ordinal0() }
	fn weekday(&self) -> Weekday { self.value.weekday() }
	fn isoweekdate(&self) -> (i32, u32, Weekday) { self.value.isoweekdate() }

	fn with_year(&self, year: i32) -> Option<DateTime<F, T>> {
		match self.value.with_year(year) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_month(&self, month: u32) -> Option<DateTime<F, T>> {
		match self.value.with_month(month) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_month0(&self, month0: u32) -> Option<DateTime<F, T>> {
		match self.value.with_month0(month0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_day(&self, day: u32) -> Option<DateTime<F, T>> {
		match self.value.with_day(day) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_day0(&self, day0: u32) -> Option<DateTime<F, T>> {
		match self.value.with_day0(day0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_ordinal(&self, ordinal: u32) -> Option<DateTime<F, T>> {
		match self.value.with_ordinal(ordinal) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTime<F, T>> {
		match self.value.with_ordinal0(ordinal0) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}
}

impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Timelike for DateTime<F, T> {
	fn hour(&self) -> u32 { self.value.hour() }
	fn minute(&self) -> u32 { self.value.minute() }
	fn second(&self) -> u32 { self.value.second() }
	fn nanosecond(&self) -> u32 { self.value.nanosecond() }

	fn with_hour(&self, hour: u32) -> Option<DateTime<F, T>> {
		match self.value.with_hour(hour) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_minute(&self, min: u32) -> Option<DateTime<F, T>> {
		match self.value.with_minute(min) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_second(&self, sec: u32) -> Option<DateTime<F, T>> {
		match self.value.with_second(sec) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}

	fn with_nanosecond(&self, nano: u32) -> Option<DateTime<F, T>> {
		match self.value.with_nanosecond(nano) {
			Some(dt) => Some(DateTime::new(dt)),
			None => None
		}
	}
}

//Serialize date
impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Serialize for DateTime<F, T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer
	{
		serializer.serialize_str(&self.format())
	}
}

//Deserialize date
impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Deserialize for DateTime<F, T> {
	fn deserialize<D>(deserializer: &mut D) -> Result<DateTime<F, T>, D::Error> where D: Deserializer {
		struct DateTimeVisitor<F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> {
			phantom_f: PhantomData<F>,
			phantom_t: PhantomData<T>
		}

		impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> Default for DateTimeVisitor<F, T> {
			fn default() -> DateTimeVisitor<F, T> {
				DateTimeVisitor::<F, T> {
					phantom_f: PhantomData,
					phantom_t: PhantomData
				}
			}
		}

		impl <F: Format, T: ElasticMapping<F> + ElasticDateMapping<F>> serde::de::Visitor for DateTimeVisitor<F, T> {
			type Value = DateTime<F, T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<DateTime<F, T>, E> where E: serde::de::Error {
				let result = DateTime::<F, T>::parse(v);
				result.map_err(|err| serde::de::Error::custom(format!("{}", err)))
			}
		}

		deserializer.deserialize(DateTimeVisitor::<F, T>::default())
	}
}