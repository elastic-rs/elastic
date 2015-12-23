pub mod parse;

//Date formats
pub trait Format {
	fn fmt() -> Vec<&'static str>;
	fn es_fmt() -> &'static str;
}

#[derive(Clone)]
pub struct EpochMillis;
impl Format for EpochMillis {
	fn fmt() -> Vec<&'static str> {
		vec![]
	}
	fn es_fmt() -> &'static str {
		"epoch_millis"
	}
}

pub const EPOCH_SECOND: &'static str = "%s";
#[derive(Clone)]
pub struct EpochSecond;
impl Format for EpochSecond {
	fn fmt() -> Vec<&'static str> {
		vec![EPOCH_SECOND]
	}
	fn es_fmt() -> &'static str {
		"epoch_second"
	}
}

pub const BASIC_DATE_TIME_NO_MILLIS: &'static str = "%Y%m%dT%H%M%SZ";
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