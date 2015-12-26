use std::error::Error;
use std::marker::PhantomData;
use chrono;
use chrono::UTC;
use chrono::format::Parsed;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::Format;
use super::format::BasicDateTime;
use super::format::parse;

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

	//TODO: Add proper error type for accumulating format failures
    //TODO: Look at moving this into different struct with default for potential overloading
	pub fn parse(date: &str) -> Result<DateTime<T>, String> {
		let fmts = T::fmt();

		let mut errors: Vec<String> = Vec::with_capacity(fmts.len());
		let mut result: Result<DateTime<T>, String> = Err(String::new());

		for fmt in fmts {
			let f = parse::to_tokens(fmt);
			let mut parsed = Parsed::new();

			match chrono::format::parse(&mut parsed, date, f.into_iter())
			.map_err(|err| format!("{} : {}", fmt, err).to_string()) {
				Ok(_) => {
					//If the parsed result doesn't contain any time, set it to the default
					if parsed.hour_mod_12.is_none() {
						let _ = parsed.set_hour(0);
						let _ = parsed.set_minute(0);
					}

					//Set the DateTime result
					result = Ok(
						DateTime::new(
							chrono::DateTime::from_utc(
								parsed.to_naive_datetime_with_offset(0).unwrap(), 
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
