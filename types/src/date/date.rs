use std::marker::PhantomData;
use chrono;
use chrono::{ UTC, Weekday, NaiveDateTime, NaiveDate, NaiveTime };
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::DT;
use super::format::{ DateFormat, ParseError };
use super::formats::ChronoFormat;
use super::mapping::{ ElasticDateMapping, DefaultDateMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

pub use chrono::{ Datelike, Timelike };

impl ElasticType<DefaultDateMapping<ChronoFormat>, ChronoFormat> for DT {

}

/// An Elasticsearch `date` type with a required `time` component.
///
/// The [format](format/index.html) is provided as a generic parameter.
/// This struct wraps up a `chrono::DateTime<UTC>` struct, meaning storing time in `UTC` is required.
///
/// # Examples
///
/// Defining a date using the default format:
///
/// ```
/// use elastic_types::date::{ ElasticDate, DefaultDateFormat };
///
/// let date: ElasticDate<DefaultDateFormat> = ElasticDate::now();
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
/// let date: ElasticDate<BasicDateTime, DefaultDateMapping<_>> = ElasticDate::now();
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
///
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Debug, Clone)]
pub struct ElasticDate<F, T = DefaultDateMapping<F>> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	value: DT,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	/// Creates a new `ElasticDate` from the given `chrono::DateTime<UTC>`.
	///
	/// This function will consume the provided `chrono` date.
	///
	/// # Examples
	///
	/// Create an `ElasticDate` from the given `chrono::DateTime`:
	///
	/// ```
	/// # extern crate elastic_types;
	/// # extern crate chrono;
	/// # fn main() {
	/// use chrono::UTC;
	/// use elastic_types::date::{ ElasticDate, DefaultDateFormat };
	///
	/// //Create a chrono DateTime struct
	/// let chronoDate = UTC::now();
	///
	/// //Give it to the ElasticDate struct
	/// let esDate: ElasticDate<DefaultDateFormat> = ElasticDate::new(chronoDate);
	/// # }
	/// ```
	pub fn new(date: DT) -> ElasticDate<F, T> {
		ElasticDate {
			value: date,
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}

	/// Creates an `ElasticDate` from the given UTC primitives:
	///
	/// ```
	/// use elastic_types::date::{ ElasticDate, DefaultDateFormat };
	///
	/// let esDate: ElasticDate<DefaultDateFormat> = ElasticDate::build(
	/// 	2015,
	/// 	5,
	/// 	14,
	/// 	16,
	/// 	45,
	/// 	8,
	/// 	886
	/// );
	/// ```
	pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> ElasticDate<F, T> {
		ElasticDate {
			value: DT::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd(year, month, day),
					NaiveTime::from_hms_milli(hour, minute, second, milli)
				),
				UTC
			),
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}

	/// Gets the current system time.
	///
	/// # Examples
	///
	/// ```
	/// use elastic_types::date::{ ElasticDate, DefaultDateFormat };
	///
	/// let date: ElasticDate<DefaultDateFormat> = ElasticDate::now();
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
	/// let date: ElasticDate<BasicDateTime> = ElasticDate::now();
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
	/// let otherdate: ElasticDate<EpochMillis> = date.remap();
	/// ```
	pub fn remap<FInto, TInto>(self) -> ElasticDate<FInto, TInto> where
	FInto: DateFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticDateMapping<FInto> {
		ElasticDate::<FInto, TInto>::new(self.value)
	}
}

impl <F, T> ElasticType<T, F> for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {

}

impl <F, T> Default for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	fn default() -> ElasticDate<F, T> {
		ElasticDate::<F, T>::now()
	}
}

impl <F, T> From<DT> for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	fn from(dt: DT) -> ElasticDate<F, T> {
		ElasticDate::<F, T>::new(dt)
	}
}

impl <F, T> Datelike for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
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
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
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

impl <F, T> Serialize for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.format())
	}
}

impl <F, T> Deserialize for ElasticDate<F, T> where
F: DateFormat,
T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticDate<F, T>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct DateTimeVisitor<F, T> where
		F: DateFormat,
		T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
			phantom_f: PhantomData<F>,
			phantom_t: PhantomData<T>
		}

		impl <F, T> serde::de::Visitor for DateTimeVisitor<F, T> where
		F: DateFormat,
		T: ElasticFieldMapping<F> + ElasticDateMapping<F> {
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
