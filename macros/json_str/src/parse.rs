use std::str;

//TODO: Should take json state. Don't check for special values if parsing key
pub fn sanitise(remainder: &[u8], current: &mut String) {
	//Parse to a change of state, sending in the remainder and current
	if remainder.len() == 0 {
		return;
	}

	match remainder[0] {
		//Key
		b'"'|b'\'' => {
			let quote_byte = remainder[0];
			let (rest, key) = take_while1(&remainder[1..], |c| c != quote_byte);

			current.push('"');
			current.push_str(key);
			current.push('"');

			sanitise(&rest[1..], current)
		},
		//Start of item
		b'{'|b'['|b':' => {
			let (rest, c) = take_first(remainder, |_| true);
			current.push(c as char);

			sanitise(&rest[1..], current)
		},
		//Replacements
		b'$' => {
			let (rest, key) = take_while1(&remainder[1..], |c|
				(c as char).is_alphanumeric() ||
				c == b'_' ||
				c == b'.'
			);

			current.push('$');
			current.push_str(key);

			sanitise(&rest[1..], current)
		},
		//Unquoted strings
		b if (b as char).is_alphabetic() => {
			let (rest, key) = take_while1(&remainder, |c|
				(c as char).is_alphabetic() ||
				c == b'_' ||
				c == b'.'
			);

			//Check if the string is a special value; true, false or null
			//For special values, push them as straight unquoted values. Otherwise, quote them
			match key {
				"true"|"false"|"null" =>
					current.push_str(key),
				_ => {
					current.push('"');
					current.push_str(key);
					current.push('"');
				}
			}

			sanitise(rest, current)
		},
		//Trim whitespace
		b' '|b'\r'|b'\n'|b'\t' => {
			sanitise(&remainder[1..], current)
		},
		//Other chars
		_ => {
			let (rest, c) = take_first(remainder, |_| true);
			current.push(c as char);

			sanitise(&rest[1..], current)
		}
	}
}

pub fn shift_while<F>(i: &[u8], f: F) -> &[u8] where F: Fn(u8) -> bool {
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

pub fn take_while1<F>(i: &[u8], f: F) -> (&[u8], &str) where F: Fn(u8) -> bool {
	let mut ctr = 0;

	for c in i {
		if f(*c) || ctr == 0 {
			ctr += 1;
		}
		else {
			break;
		}
	}

	(&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn take_first<F>(i: &[u8], f: F) -> (&[u8], u8) where F: Fn(u8) -> bool {
	let size = i.len();

	let mut ctr = 0;

	for c in i {
		if f(*c) || ctr == size - 1 {
			break;
		}
		else {
			ctr += 1;
		}
	}

	(&i[ctr..], i[ctr])
}

pub fn shift(i: &[u8], c: usize) -> &[u8] {
	match c {
		c if c >= i.len() => &[],
		_ => &i[c..]
	}
}
