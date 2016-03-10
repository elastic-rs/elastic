use syntax::ptr::P;
use syntax::ast::Expr;
use syntax::ext::base::ExtCtxt;

use chrono::format::{ Item, Fixed, Numeric, Pad };
use super::parse::*;

pub fn to_tokens(fmt: &str) -> Vec<Item> {
	let mut res = Vec::<Item>::new();
	parse_all(fmt.as_bytes(), &mut res);

	res
}

pub fn to_chrono_format(fmt: Vec<Item>) -> String {
	format_tokens(fmt, |i| Formatter::to_chrono_string(i))
}

pub fn to_es_format(fmt: Vec<Item>) -> String {
	format_tokens(fmt, |i| Formatter::to_es_string(i))
}

fn format_tokens<'a, F>(fmt: Vec<Item<'a>>, f: F) -> String where F: FnMut(&Item<'a>) -> String {
	let f: Vec<String> = fmt.iter().map(f).collect();

	f.join("")
}

pub struct Formatter;
impl Formatter {
	pub fn to_es_string(item: &Item) -> String {
		match *item {
			Item::Literal(c) => c.to_string(),
			Item::Numeric(Numeric::Year, Pad::Zero) => "yyyy".to_string(),
			Item::Numeric(Numeric::Month, Pad::Zero) => "MM".to_string(),
			Item::Numeric(Numeric::Day, Pad::Zero) => "dd".to_string(),
			Item::Numeric(Numeric::Hour, Pad::Zero) => "HH".to_string(),
			Item::Numeric(Numeric::Minute, Pad::Zero) => "mm".to_string(),
			Item::Numeric(Numeric::Second, Pad::Zero) => "ss".to_string(),
			Item::Fixed(Fixed::Nanosecond3) => ".SSS".to_string(),
			_ => "".to_string()
		}
	}

	pub fn to_chrono_string(item: &Item) -> String {
		match *item {
			Item::Literal(c) => c.to_string(),
			Item::Numeric(Numeric::Year, Pad::Zero) => "%Y".to_string(),
			Item::Numeric(Numeric::Month, Pad::Zero) => "%m".to_string(),
			Item::Numeric(Numeric::Day, Pad::Zero) => "%d".to_string(),
			Item::Numeric(Numeric::Hour, Pad::Zero) => "%H".to_string(),
			Item::Numeric(Numeric::Minute, Pad::Zero) => "%M".to_string(),
			Item::Numeric(Numeric::Second, Pad::Zero) => "%S".to_string(),
			Item::Fixed(Fixed::Nanosecond3) => "%.3f".to_string(),
			_ => "".to_string()
		}
	}

	pub fn to_stmt(item: &Item, cx: &ExtCtxt) -> P<Expr> {
		match *item {
			Item::Literal(c) => quote_expr!(cx, chrono::format::Item::Literal($c)),
			Item::Numeric(Numeric::Year, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Year, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Month, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Month, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Day, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Day, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Hour, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Hour, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Minute, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Minute, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Second, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Second, chrono::format::Pad::Zero)),
			Item::Fixed(Fixed::Nanosecond3) => quote_expr!(cx, chrono::format::Item::Fixed(chrono::format::Fixed::Nanosecond3)),
			_ => quote_expr!(cx, chrono::format::Item::Literal(""))
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
const ES_MSEC_PRE: u8 = b'.';
const CR_PREFIX: u8 = b'%';
const CR_YEAR: u8 = b'Y';
const CR_MONTH: u8 = b'm';
const CR_DAY: u8 = b'd';
const CR_HOUR: u8 = b'H';
const CR_MIN: u8 = b'M';
const CR_SEC: u8 = b'S';
const CR_MSEC_PRE: u8 = b'.';

fn not_date_token(c: u8) -> bool {
	match c {
		ES_YEAR => false,
		ES_MONTH => false,
		ES_DAY => false,
		ES_HOUR => false,
		ES_MIN => false,
		ES_SEC => false,
		ES_MSEC => false,
		ES_MSEC_PRE => false,
		CR_PREFIX => false,
		_ => true
	}
}

fn parse_all<'a, 'b>(i: &'a [u8], r: &'b mut Vec<Item<'a>>) {
	let (k, res) = parse(i);

	match res {
		Some(res) => {
			r.push(res);
			parse_all(k, r);
		},
		None => ()
	}
}

fn parse<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	let l = i.len();
	if l == 0 {
		(i, None)
	}
	else {
		let (i0, i1) = if l == 1 {
			(i[0], 0)
		}
		else {
			(i[0], i[1])
		};

		match (i0, i1) {
			//yy* | %Y
			(ES_YEAR, ES_YEAR)|(CR_PREFIX, CR_YEAR) => parse_year(i),
			//MM* | %m
			(ES_MONTH, ES_MONTH)|(CR_PREFIX, CR_MONTH) => parse_month(i),
			//dd* | %d
			(ES_DAY, ES_DAY)|(CR_PREFIX, CR_DAY) => parse_day(i),
			//HH* | %H
			(ES_HOUR, ES_HOUR)|(CR_PREFIX, CR_HOUR) => parse_hour(i),
			//mm* | %M
			(ES_MIN, ES_MIN)|(CR_PREFIX, CR_MIN) => parse_min(i),
			//ss* | %S
			(ES_SEC, ES_SEC)|(CR_PREFIX, CR_SEC) => parse_sec(i),
			//SS* | %.
			(ES_MSEC, ES_MSEC)|(CR_PREFIX, CR_MSEC_PRE) => parse_msec(i),
			//.S*
			(ES_MSEC_PRE, ES_MSEC) => parse_msec(i),
			//.*
			_ => parse_chars(i)
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

fn parse_year<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b'y', Some(Item::Numeric(Numeric::Year, Pad::Zero)))
}

fn parse_month<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b'M', Some(Item::Numeric(Numeric::Month, Pad::Zero)))
}

fn parse_day<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b'd', Some(Item::Numeric(Numeric::Day, Pad::Zero)))
}

fn parse_hour<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b'H', Some(Item::Numeric(Numeric::Hour, Pad::Zero)))
}

fn parse_min<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b'm', Some(Item::Numeric(Numeric::Minute, Pad::Zero)))
}

fn parse_sec<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	parse_token!(i, b's', Some(Item::Numeric(Numeric::Second, Pad::Zero)))
}

fn parse_msec<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	match i[0] {
		b'.' => {
			parse_msec(shift(i, 1))
		},
		b'S' => {
			(shift_while(i, |c| c == b'S'), Some(Item::Fixed(Fixed::Nanosecond3)))
		},
		b'%' => {
			let (k, r) = (shift(i, 4), Some(Item::Fixed(Fixed::Nanosecond3)));
			(k, r)
		},
		_ => panic!("unexpected symbol")
	}
}

fn parse_chars<'a>(i: &'a [u8]) -> (&'a [u8], Option<Item<'a>>) {
	let (k, s) = take_while1(i, |c| not_date_token(c));
	(k, Some(Item::Literal(s)))
}