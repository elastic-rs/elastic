use std::str;
use chrono::format::{ Item, Fixed, Numeric, Pad };

#[derive(Clone, Debug, PartialEq)]
pub enum DateToken<'a> {
	Chars(&'a str),
	Year,
	Month,
	Day,
	Hour,
	Min,
	Sec,
	Msec
}

impl <'a> ToString for DateToken<'a> {
	fn to_string(&self) -> String {
		match *self {
			DateToken::Chars(c) => c.to_string(),
			DateToken::Year => "%Y".to_string(),
			DateToken::Month => "%m".to_string(),
			DateToken::Day => "%d".to_string(),
			DateToken::Hour => "%H".to_string(),
			DateToken::Min => "%M".to_string(),
			DateToken::Sec => "%S".to_string(),
			DateToken::Msec => "%.3f".to_string()
		}
	}
}

impl <'a> DateToken<'a> {
	fn to_es_string(&self) -> String {
		match *self {
			DateToken::Chars(c) => c.to_string(),
			DateToken::Year => "yyyy".to_string(),
			DateToken::Month => "MM".to_string(),
			DateToken::Day => "dd".to_string(),
			DateToken::Hour => "HH".to_string(),
			DateToken::Min => "mm".to_string(),
			DateToken::Sec => "ss".to_string(),
			DateToken::Msec => ".SSS".to_string()
		}
	}

	fn to_item(&self) -> Item<'a> {
		match *self {
			DateToken::Chars(c) => Item::Literal(c),
			DateToken::Year => Item::Numeric(Numeric::Year, Pad::Zero),
			DateToken::Month => Item::Numeric(Numeric::Month, Pad::Zero),
			DateToken::Day => Item::Numeric(Numeric::Day, Pad::Zero),
			DateToken::Hour => Item::Numeric(Numeric::Hour, Pad::Zero),
			DateToken::Min => Item::Numeric(Numeric::Minute, Pad::Zero),
			DateToken::Sec => Item::Numeric(Numeric::Second, Pad::Zero),
			DateToken::Msec => Item::Fixed(Fixed::Nanosecond3)
		}
	}
}

const ES_YEAR: u8 = b'y';
const ES_MONTH: u8 = b'M';
const ES_DAY: u8 = b'd';
const ES_HOUR: u8 = b'H';
const ES_MIN: u8 = b'm';
const ES_SEC: u8 = b's';
const ES_MSEC: u8 = b'S';
const CR_PREFIX: u8 = b'%';

fn not_date_token(c: u8) -> bool {
	match c {
		ES_YEAR => false,
		ES_MONTH => false,
		ES_DAY => false,
		ES_HOUR => false,
		ES_MIN => false,
		ES_SEC => false,
		ES_MSEC => false,
		CR_PREFIX => false,
		_ => true
	}
}

fn parse_all<'a, 'b>(i: &'a [u8], r: &'b mut Vec<DateToken<'a>>) {
	let (k, res) = parse(i);

	match res {
		Some(res) => {
			r.push(res);
			parse_all(k, r);
		},
		None => ()
	}
}

fn parse<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	if i.len() == 0 {
		(i, None)
	}
	else {
		match i[0] {
			b'y' => parse_year(i),
			b'M' => parse_month(i),
			b'd' => parse_day(i),
			b'H' => parse_hour(i),
			b'm' => parse_minute(i),
			b's' => parse_second(i),
			b'.' => parse_millisecond(i),
			b'%' => parse_chrono(i),
			_ => parse_chars(i)
		}
	}
}

fn parse_chrono<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	if i.len() < 2 {
		(i, None)
	}
	else {
		match i[1] {
			b'Y' => parse_year(i),
			b'm' => parse_month(i),
			b'd' => parse_day(i),
			b'H' => parse_hour(i),
			b'M' => parse_minute(i),
			b'S' => parse_second(i),
			b'.' => parse_millisecond(i),
			_ => panic!("unexpected symbol")
		}
	}
}

macro_rules! parse_token {
    ($i:ident, $m:expr, $r:expr) => ({
    	match $i[0] {
			$m => (shift_while($i, |c| c == $m), $r),
			b'%' => (shift($i, 2), $r),
			_ => panic!("unexpected symbol")
		}
    })
}

fn parse_year<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b'y', Some(DateToken::Year))
}

fn parse_month<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b'M', Some(DateToken::Month))
}

fn parse_day<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b'd', Some(DateToken::Day))
}

fn parse_hour<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b'H', Some(DateToken::Hour))
}

fn parse_minute<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b'm', Some(DateToken::Min))
}

fn parse_second<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	parse_token!(i, b's', Some(DateToken::Sec))
}

fn parse_millisecond<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	match i[0] {
		b'.' => {
			parse_millisecond(shift(i, 1))
		},
		b'S' => {
			(shift_while(i, |c| c == b'S'), Some(DateToken::Msec))
		},
		b'%' => {
			let (k, r) = (shift(i, 4), Some(DateToken::Msec));
			(k, r)
		},
		_ => panic!("unexpected symbol")
	}
}

fn parse_chars<'a>(i: &'a [u8]) -> (&'a [u8], Option<DateToken<'a>>) {
	let (k, s) = take_while(i, |c| not_date_token(c));
	(k, Some(DateToken::Chars(s)))
}

fn shift_while<F>(i: &[u8], f: F) -> &[u8] where F: Fn(u8) -> bool {
	let mut ctr = 0;
	for c in i {
		if f(*c) {
			ctr += 1;
		}
		else {
			break;
		}
	}

	&i[ctr..]
}

fn take_while<F>(i: &[u8], f: F) -> (&[u8], &str) where F: Fn(u8) -> bool {
	let mut ctr = 0;

	for c in i {
		if f(*c) {
			ctr += 1;
		}
		else {
			break;
		}
	}

	(&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

fn shift(i: &[u8], c: usize) -> &[u8] {
	match c {
		c if c >= i.len() => &[],
		_ => &i[c..]
	}
}

pub fn to_tokens(fmt: &str) -> Vec<DateToken> {
	let mut res = Vec::<DateToken>::new();
	parse_all(fmt.as_bytes(), &mut res);

	res
}

pub fn to_chrono_format(fmt: Vec<DateToken>) -> String {
	format_tokens(fmt, |i| i.to_string())
}

pub fn to_chrono_tokens(fmt: Vec<DateToken>) -> Vec<Item> {
	let f: Vec<Item> = fmt.iter().map(|c| c.to_item()).collect();
	f
}

pub fn to_es_format(fmt: Vec<DateToken>) -> String {
	format_tokens(fmt, |i| i.to_es_string())
}

fn format_tokens<'a, F>(fmt: Vec<DateToken<'a>>, f: F) -> String where F: FnMut(&DateToken<'a>) -> String {
	let f: Vec<String> = fmt.iter().map(f).collect();

	f.join("")
}