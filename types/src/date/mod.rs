pub mod format;

pub use self::format::Format;

use std::error::Error;
use std::marker::PhantomData;
use chrono;
use chrono::UTC;
use chrono::offset::TimeZone;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };

pub type DT = chrono::DateTime<UTC>;

pub fn now<T: Format>() -> DateTime<T> {
	DateTime::<T>::default()
}

#[derive(Clone)]
pub struct DateTime<T: Format = self::format::BasicDateTime> {
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

	//TODO: Add proper error type for accumulating format failures
	pub fn parse(date: &str) -> Result<DateTime<T>, String> {
		let fmts = T::fmt();

		let mut errors: Vec<String> = Vec::with_capacity(fmts.len());
		let mut result: Result<DateTime<T>, String> = Err(String::new());

		for fmt in fmts {
			match chrono::UTC.datetime_from_str(date, fmt)
			.map_err(|err| format!("{} : {}", fmt, err).to_string()) {
				Ok(parsed) => {
					result = Ok(
						DateTime::<T>::new(
							chrono::DateTime::from_utc(
								parsed.naive_utc(), 
								chrono::UTC
							)
						)
					);
					break;
				},
				Err(e) => errors.push(e)
			}
		}

		result.map_err(|_| errors.join(", "))
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