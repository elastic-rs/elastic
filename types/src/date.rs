use std::marker::PhantomData;
use std::str::FromStr;
use chrono;
use chrono::offset::TimeZone;
use chrono::{ UTC };

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
				Err(e) => ()
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

pub fn now<T: Format>() -> DateTime<T> {
	DateTime::<T>::default()
}

pub trait Format {
	fn fmt() -> Vec<&'static str>;
}

#[derive(Clone)]
pub struct StrictDateOptionalTime;
impl Format for StrictDateOptionalTime {
	fn fmt() -> Vec<&'static str> {
		vec!["%Y-%m-%dT%H:%M:%SZ"]
	}
}