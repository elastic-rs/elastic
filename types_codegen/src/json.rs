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