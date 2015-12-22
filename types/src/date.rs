use std::marker::PhantomData;
use chrono;
use chrono::offset::TimeZone;
use chrono::{ UTC };
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };

pub type DT = chrono::DateTime<UTC>;

#[derive(Clone)]
pub struct DateTime<T: Format = StrictDateOptionalTime> {
	pub value: DT,
	phantom: PhantomData<T>
}
impl <T: Format> DateTime<T> {
	pub fn new(date: DT) -> DateTime<T> {
		DateTime {
			value: date,
			phantom: PhantomData
		}
	}

	pub fn parse(date: &str) -> DateTime<T> {
		//Get the formats, we need to find the first positive match
		let fmts = T::fmt();

		let mut dt: Option<DT> = None;
		for fmt in fmts {
			match chrono::UTC.datetime_from_str(date, fmt) {
				Ok(parsed) => {
					dt = Some(
						chrono::DateTime::from_utc(
							parsed.naive_utc(), 
							chrono::UTC
						)
					);
					break;
				},
				Err(_) => ()
			}
		}

		match dt {
			Some(dt) => DateTime::<T>::new(dt),
			None => DateTime::<T>::default()
		}
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
		Ok(DateTime::<T>::parse(v))
	}
}

pub fn now<T: Format>() -> DateTime<T> {
	DateTime::<T>::default()
}

//Date formats
pub trait Format {
	fn fmt() -> Vec<&'static str>;
}

#[derive(Clone)]
pub struct EpochSecond;
impl Format for EpochSecond {
	fn fmt() -> Vec<&'static str> {
		vec!["%s"]
	}
}

#[derive(Clone)]
pub struct DateOptionalTime;
impl Format for DateOptionalTime {
	fn fmt() -> Vec<&'static str> {
		vec!["%Y-%m-%dT%H:%M:%SZ"]
	}
}

#[derive(Clone)]
pub struct StrictDateOptionalTime;
impl Format for StrictDateOptionalTime {
	fn fmt() -> Vec<&'static str> {
		vec!["%Y-%m-%dT%H:%M:%SZ"]
	}
}