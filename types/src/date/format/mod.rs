pub mod parse;

#[macro_export]
macro_rules! date_fmt(
	($fmt:expr) => ({
		$crate::date::format::parse::items(
			$crate::date::format::parse::format($fmt)
		)
	});
);

//Date formats
pub trait Format {
	fn fmt() -> Vec<&'static str>;
	fn es_fmt() -> &'static str;
}

pub const BASIC_DATE_TIME_NO_MILLIS: &'static str = "%Y%m%d";

#[derive(Clone)]
pub struct BasicDateTimeNoMillis;
impl Format for BasicDateTimeNoMillis {
	fn fmt() -> Vec<&'static str> {
		vec![BASIC_DATE_TIME_NO_MILLIS]
	}
	fn es_fmt() -> &'static str {
		"basic_date_time_no_millis"
	}
}

pub const BASIC_DATE_TIME: &'static str = "%Y%m%dT%H%M%S%.3fZ";

#[derive(Clone)]
pub struct BasicDateTime;
impl Format for BasicDateTime {
	fn fmt() -> Vec<&'static str> {
		vec![BASIC_DATE_TIME]
	}
	fn es_fmt() -> &'static str {
		"basic_date_time"
	}
}