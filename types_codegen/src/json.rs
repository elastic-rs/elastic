use super::parsers::*;

#[derive(PartialEq)]
pub enum JsonPart {
	Literal(String),
	Replacement(String, ReplacementPart)
}

#[derive(PartialEq)]
pub enum ReplacementPart {
	Key,
	Value
}

impl ToString for JsonPart {
	fn to_string(&self) -> String {
		match *self {
			JsonPart::Literal(ref s) => s.clone(),
			JsonPart::Replacement(ref s, _) => s.clone()
		}
	}
}

pub fn sanitise(remainder: &[u8], current: &mut String) {
    //Parse to a change of state, sending in the remainder and current
    if remainder.len() == 0 {
        return;
    }
    
    match remainder[0] {
        //Key
        b'"' => {
            let (rest, key) = take_while1(&remainder[1..], |c| c != b'"');
            
            current.push('"');
            current.push_str(key);
            current.push('"');
            
            sanitise(&rest[1..], current)
        },
        b'\'' => {
            let (rest, key) = take_while1(&remainder[1..], |c| c != b'\'');
            
            current.push('"');
            current.push_str(key);
            current.push('"');
            
            sanitise(&rest[1..], current)
        },
        //Start of item
        b'{'|b'['|b':' => {
            let (rest, c) = take_first(remainder, |c| true);
            current.push(c as char);
            
            sanitise(&rest[1..], current)
        },
        //Replacements
        b'$' => {
            let (rest, key) = take_while1(&remainder[1..], |c| 
                (c as char).is_alphabetic() ||
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
            
            current.push('"');
            current.push_str(key);
            current.push('"');
            
            sanitise(rest, current)
        },
        //Trim whitespace
        b' '|b'\r'|b'\n'|b'\t' => {
            sanitise(&remainder[1..], current)
        },
        //Other chars
        _ => {
            let (rest, c) = take_first(remainder, |c| true);
            current.push(c as char);
            
            sanitise(&rest[1..], current)
        }
    }
}

pub fn parse_to_replacement(json: &[u8], parts: &mut Vec<JsonPart>) {
	if json.len() > 0 {
		let (a, part) = take_while1(json, |c| c != b'$');
		parts.push(JsonPart::Literal(part.to_string()));

		if a.len() > 0 {
			let (b, ident) = take_while1(&a[1..], |c| 
				c != b':' && 
				c != b'}' &&
				c != b']' &&
				c != b',' &&
				c != b' ' &&
				c != b'{' &&
				c != b'['
			);

			let (_, token) = take_first(a, |c|
				c == b':' || 
				c == b'"' ||
				c == b'\'' ||
				c == b',' ||
				c == b'{' ||
				c == b'['
			);

			let id = ident.to_string().replace(" ", "");
			match token {
				b':' => parts.push(JsonPart::Replacement(id, ReplacementPart::Key)),
				_ => parts.push(JsonPart::Replacement(id, ReplacementPart::Value))
			}
			parse_to_replacement(b, parts);
		}
	}
}
