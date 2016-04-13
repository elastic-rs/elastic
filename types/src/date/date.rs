use std::marker::PhantomData;
use chrono;
use chrono::Weekday;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::{ DT, DefaultFormat };
use super::format::{ DateFormat, ParseError };
use super::mapping::{ ElasticDateMapping, DefaultDateMapping };
use ::mapping::{ ElasticTypeMapping, ElasticType };

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
/// use elastic_types::date::ElasticDate;
///
/// let date: ElasticDate = ElasticDate::now();
/// ```
///
/// Defining a date using a named format:
///
/// ```
/// use elastic_types::date::{ ElasticDate, BasicDateTime };
///
/// let date = ElasticDate::<BasicDateTime>::now();
/// ```
///
/// Defining a date using a custom mapping:
///
/// ```
/// use elastic_types::date::mapping::DefaultDateMapping;
/// use elastic_types::date::{ ElasticDate, BasicDateTime };
///
/// let date = ElasticDate::<BasicDateTime, DefaultDateMapping>::now();
/// ```
///
/// Accessing the values of a date:
///
/// ```
/// use elastic_types::date::{ ElasticDate, Datelike, Timelike, BasicDateTime };
///
/// let date = ElasticDate::<BasicDateTime>::now();
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
pub struct ElasticDate<F = DefaultFormat, T = DefaultDateMapping<F>> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	value: DT,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	/// Creates a new `ElasticDate` from the given `chrono::DateTime<UTC>`.
	///
	/// This function will consume the provided `chrono` date.
	///
	/// # Examples
	///
	/// Create a `ElasticDate` from the current date:
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
	/// //Give it to the ElasticDate struct
	/// let esDate: ElasticDate = ElasticDate::new(chronoDate);
	/// # }
	/// ```
	pub fn new(date: DT) -> ElasticDate<F, T> {
		ElasticDate {
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
	/// let date: ElasticDate = ElasticDate::now();
	/// ```
	pub fn now() -> ElasticDate<F, T> {
		ElasticDate {
			value: chrono::UTC::now(),
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}

	/// Parse the date and time from a string.
	///
	/// The format of the string must match the given `DateFormat`.
	///
	/// # Examples
	///
	/// Parsing from a specified `DateFormat`.
	///
	/// ```
	/// use elastic_types::date::{ ElasticDate, BasicDateTime };
	///
	/// let date = ElasticDate::<BasicDateTime>::parse("20151126T145543.778Z").unwrap();
	/// ```
	pub fn parse(date: &str) -> Result<ElasticDate<F, T>, ParseError> {
		F::parse(date).map(ElasticDate::new)
	}

	/// Format the date and time as a string.
	///
	/// The format of the string is specified by the given `DateFormat`.
	///
	/// # Examples
	///
	/// ```
	/// use elastic_types::date::{ ElasticDate, BasicDateTime };
	///
	/// let date: ElasticDate = ElasticDate::now();
	/// let fmt = date.format();
	///
	/// //eg: 20151126T145543.778Z
	/// println!("{}", fmt);
	/// ```
	pub fn format(&self) -> String {
		F::format(&self.value)
	}

	/// Change the format/mapping of this date.
	///
	/// # Examples
	///
	/// ```
	/// use elastic_types::date::{ ElasticDate, BasicDateTime, EpochMillis };
	///
	/// //Get the current datetime formatted as basic_date_time
	/// let date: ElasticDate<BasicDateTime> = ElasticDate::now();
	///
	/// //Change the format to epoch_millis
	/// let otherdate: ElasticDate<EpochMillis> = date.into();
	/// ```
	pub fn into<FInto, TInto>(self) -> ElasticDate<FInto, TInto> where
	FInto: DateFormat,
	TInto: ElasticTypeMapping<FInto> + ElasticDateMapping<FInto> {
		ElasticDate::<FInto, TInto>::new(self.value)
	}
}

impl <F, T> ElasticType<T, F> for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {

}

impl <F, T> Default for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn default() -> ElasticDate<F, T> {
		ElasticDate::<F, T>::now()
	}
}

impl <F, T> From<DT> for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn from(dt: DT) -> ElasticDate<F, T> {
		ElasticDate::<F, T>::new(dt)
	}
}

impl <F, T> Datelike for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn year(&self) -> i32 { self.value.year() }
	fn month(&self) -> u32 { self.value.month() }
	fn month0(&self) -> u32 { self.value.month0() }
	fn day(&self) -> u32 { self.value.day() }
	fn day0(&self) -> u32 { self.value.day0() }
	fn ordinal(&self) -> u32 { self.value.ordinal() }
	fn ordinal0(&self) -> u32 { self.value.ordinal0() }
	fn weekday(&self) -> Weekday { self.value.weekday() }
	fn isoweekdate(&self) -> (i32, u32, Weekday) { self.value.isoweekdate() }

	fn with_year(&self, year: i32) -> Option<ElasticDate<F, T>> {
		match self.value.with_year(year) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_month(&self, month: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_month(month) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_month0(&self, month0: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_month0(month0) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_day(&self, day: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_day(day) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_day0(&self, day0: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_day0(day0) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_ordinal(&self, ordinal: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_ordinal(ordinal) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_ordinal0(&self, ordinal0: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_ordinal0(ordinal0) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}
}

impl <F, T> Timelike for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn hour(&self) -> u32 { self.value.hour() }
	fn minute(&self) -> u32 { self.value.minute() }
	fn second(&self) -> u32 { self.value.second() }
	fn nanosecond(&self) -> u32 { self.value.nanosecond() }

	fn with_hour(&self, hour: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_hour(hour) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_minute(&self, min: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_minute(min) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_second(&self, sec: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_second(sec) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}

	fn with_nanosecond(&self, nano: u32) -> Option<ElasticDate<F, T>> {
		match self.value.with_nanosecond(nano) {
			Some(dt) => Some(ElasticDate::new(dt)),
			None => None
		}
	}
}

//Serialize date
impl <F, T> Serialize for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.format())
	}
}

//Deserialize date
impl <F, T> Deserialize for ElasticDate<F, T> where
F: DateFormat,
T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticDate<F, T>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct DateTimeVisitor<F, T> where
		F: DateFormat,
		T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
			phantom_f: PhantomData<F>,
			phantom_t: PhantomData<T>
		}

		impl <F, T> serde::de::Visitor for DateTimeVisitor<F, T> where
		F: DateFormat,
		T: ElasticTypeMapping<F> + ElasticDateMapping<F> {
			type Value = ElasticDate<F, T>;

			fn visit_str<E>(&mut self, v: &str) -> Result<ElasticDate<F, T>, E> where
			E: serde::de::Error {
				let result = ElasticDate::<F, T>::parse(v);
				result.map_err(|err| serde::de::Error::custom(format!("{}", err)))
			}

			fn visit_i64<E>(&mut self, v: i64) -> Result<ElasticDate<F, T>, E> where
			E: serde::de::Error {
				let result = ElasticDate::<F, T>::parse(&v.to_string());
				result.map_err(|err| serde::de::Error::custom(format!("{}", err)))
			}

			fn visit_u64<E>(&mut self, v: u64) -> Result<ElasticDate<F, T>, E> where
			E: serde::de::Error {
				let result = ElasticDate::<F, T>::parse(&v.to_string());
				result.map_err(|err| serde::de::Error::custom(format!("{}", err)))
			}
		}

		deserializer.deserialize(DateTimeVisitor::<F, T>::default())
	}
}

impl ElasticType<DefaultDateMapping, DefaultFormat> for DT {

}
