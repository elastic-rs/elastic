use chomp::*;

pub struct DateParseResult {
	pub year: Vec<u8>,
	pub month: Vec<u8>,
	pub day: Vec<u8>,
	pub hour: Vec<u8>,
	pub min: Vec<u8>,
	pub sec: Vec<u8>,
	pub msec: Vec<u8>
}

impl DateParseResult {
	pub fn is_valid(&self) -> bool {
		self.date_is_valid() && (self.time_is_valid() || self.no_time_specified())
	}

	pub fn date_is_valid(&self) -> bool {
		self.year_is_valid() && self.month_is_valid() && self.day_is_valid()
	}

	pub fn time_is_valid(&self) -> bool {
		self.hour_is_valid() && self.min_is_valid() && self.sec_is_valid()
	}

	pub fn no_time_specified(&self) -> bool {
		self.hour.len() == 0 && self.min.len() == 0 && self.sec.len() == 0 && self.msec.len() == 0
	}

	pub fn year_is_valid(&self) -> bool {
		self.year.len() == 4
	}

	pub fn month_is_valid(&self) -> bool {
		let len = self.month.len();
		len == 2 || (len == 0 && self.day.len() == 3)
	}

	pub fn day_is_valid(&self) -> bool {
		let len = self.day.len();
		len == 2 || (self.month.len() == 0 && len == 3)
	}

	pub fn hour_is_valid(&self) -> bool {
		self.hour.len() == 2
	}

	pub fn min_is_valid(&self) -> bool {
		self.min.len() == 2
	}

	pub fn sec_is_valid(&self) -> bool {
		self.sec.len() == 2
	}

	pub fn msec_is_valid(&self) -> bool {
		self.msec.len() == 3
	}
}

fn es_year(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'y');
		take_while1(|c| c == b'y')
	}
}

fn es_month(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'M');
		take_while1(|c| c == b'M')
	}
}

fn es_day(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'd');
		take_while1(|c| c == b'd')
	}
}

fn es_hour(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'H');
		take_while1(|c| c == b'H')
	}
}

fn es_min(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'm');
		take_while1(|c| c == b'm')
	}
}

fn es_sec(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b's');
		take_while1(|c| c == b's')
	}
}

fn es_msec(i: Input<u8>) -> U8Result<&[u8]> {
	parse!{i;
		take_while(|c| c != b'S');
		take_while1(|c| c == b'S')
	}
}

//TODO: This could be improved a lot.
//Should run in a single pass
//Pass in transform to output, so we can use to generate chrono format, or validate result
pub fn parse(fmt: &str) -> DateParseResult {
	let mut res = DateParseResult {
		year: Vec::new(),
		month: Vec::new(),
		day: Vec::new(),
		hour: Vec::new(),
		min: Vec::new(),
		sec: Vec::new(),
		msec: Vec::new()
	};

	if let Ok(d) = parse_only(es_year, fmt.as_bytes()) {
		res.year = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_month, fmt.as_bytes()) {
		res.month = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_day, fmt.as_bytes()) {
		res.day = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_hour, fmt.as_bytes()) {
		res.hour = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_min, fmt.as_bytes()) {
		res.min = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_sec, fmt.as_bytes()) {
		res.sec = d.iter().cloned().collect();
	}
	if let Ok(d) = parse_only(es_msec, fmt.as_bytes()) {
		res.msec = d.iter().cloned().collect();
	}

	res
}