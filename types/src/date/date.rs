use std::error::Error;
use std::marker::PhantomData;
use chrono;
use chrono::UTC;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::Format;
use super::format::BasicDateTime;

pub type DT = chrono::DateTime<UTC>;

#[derive(Clone)]
pub struct DateTime<T: Format = BasicDateTime> {
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
