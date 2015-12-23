use std::str;
use chomp::*;

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

const YEAR: u8 = b'y';
const MONTH: u8 = b'M';
const DAY: u8 = b'd';
const HOUR: u8 = b'H';
const MIN: u8 = b'm';
const SEC: u8 = b's';
const MSEC: u8 = b'S';

fn not_date_token(c: u8) -> bool {
	match c {
		YEAR => false,
		MONTH => false,
		DAY => false,
		HOUR => false,
		MIN => false,
		SEC => false,
		MSEC => false,
		_ => true
	}
}

//TODO: Wrap this in a match instead
fn parse(i: Input<u8>) -> U8Result<DateToken> {
	or(i, 
		//Year
		|i| parse!{i;
			take_while1(|c| c == YEAR);
			ret DateToken::Year
		},
		|i| or(i, 
		//Month
		|i| parse!{i;
			take_while1(|c| c == MONTH);
			ret DateToken::Month
		},
		|i| or(i, 
		//Day
		|i| parse!{i;
			take_while1(|c| c == DAY);
			ret DateToken::Day
		},
		|i| or(i, 
		//Hour
		|i| parse!{i;
			take_while1(|c| c == HOUR);
			ret DateToken::Hour
		},
		|i| or(i, 
		//Minute
		|i| parse!{i;
			take_while1(|c| c == MIN);
			ret DateToken::Min
		},
		|i| or(i, 
		//Second
		|i| parse!{i;
			take_while1(|c| c == SEC);
			ret DateToken::Sec
		},
		|i| or(i, 
		//Millisecond
		|i| parse!{i;
			take_while1(|c| c == MSEC);
			ret DateToken::Msec
		},
		//Other
		|i| parse!{i;
			let res = take_while1(not_date_token);
			ret DateToken::Chars(str::from_utf8(res).unwrap())
		}))))))
	)
}

fn has_time(fmt: &Vec<DateToken>) -> bool {
	fmt.iter().any(|i| match *i {
			DateToken::Hour => true,
			DateToken::Min => true,
			DateToken::Sec => true,
			DateToken::Msec => true,
			_ => false
		}
	)
}

pub fn parse_format(fmt: &str) -> Vec<DateToken> {
	parse_only(
		|i| many(i, |i| 
			parse(i)
		), fmt.as_bytes()).unwrap()
}

pub fn parse_tokens(fmt: &Vec<DateToken>) -> String {
	let f: Vec<String> = if has_time(fmt) {
		fmt.iter().map(|i| i.to_string()).collect()
	}
	else {
		let mut _fmt = fmt.clone();
		_fmt.push_all(&vec![
			DateToken::Chars("T"),
			DateToken::Hour,
			DateToken::Min,
			DateToken::Sec,
			DateToken::Msec,
			DateToken::Chars("Z")
		]);
		_fmt.iter().map(|i| i.to_string()).collect()
	};

	f.join("")
}