use syntax;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ ExtCtxt, MacResult, MacEager };
use syntax::ext::build::AstBuilder;
use ::parse::*;

//Parse a token tree to a json `str` at compile time.
pub fn expand_json(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	let json_raw = syntax::print::pprust::tts_to_string(&args);
	let mut json = String::with_capacity(json_raw.len());
	sanitise(json_raw.as_bytes(), &mut json);

	let str_lit = cx.expr_str(sp, token::intern_and_get_ident(&json));
	MacEager::expr(P(quote_expr!(cx, $str_lit).unwrap()))
}

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