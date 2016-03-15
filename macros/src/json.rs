use std::collections::BTreeMap;
use syntax;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ast::{ Stmt };
use syntax::ast::TokenTree;
use syntax::ext::base::{ ExtCtxt, MacResult, MacEager, DummyResult };
use syntax::ext::build::AstBuilder;
use ::parse::*;

//TODO: Clean this up, it's an awful macro in this state. Look at using serde_json for intermediate representation
pub fn expand_json(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	//Get idents first, separated by commas
	//If none are found then we just continue on
	let mut idents: BTreeMap<String, syntax::ast::Ident> = BTreeMap::new();
	let mut ac = 0;
	for arg in args {
		match *arg {
			TokenTree::Token(_, token::Ident(s, _)) => {
				idents.insert(s.to_string(), s);
				ac += 1;
			},
			TokenTree::Token(_, token::Token::Comma) => ac += 1,
			_ => break
		}
	}

	//Parse the tokens to a string and sanitise the results
	let json = syntax::print::pprust::tts_to_string(&args[ac..]);
	let mut sanitised = String::with_capacity(json.len());
	sanitise(json.as_bytes(), &mut sanitised);
	
	//If there are no idents, just emit the json string
	if idents.len() == 0 {
		let str_lit = cx.expr_str(sp, token::intern_and_get_ident(&sanitised));
		return MacEager::expr(P(quote_expr!(cx, String::from($str_lit)).unwrap()))
	}
	
	//TODO: Split this out, so it's obvious that replacements are treated differently
	let mut tree = Vec::new();
	parse_to_replacement(sanitised.as_bytes(), &mut tree);

	let mut stmts: Vec<Stmt> = Vec::new();
	let mut push_stmts: Vec<Stmt> = Vec::new();

	stmts.push(quote_stmt!(cx, let mut c = 0).unwrap());

	let mut tcount = 0;
	for t in tree {
		match t {
			//For literals, emit the string value
			JsonPart::Literal(ref lit) => {
				let jname = token::str_to_ident(&format!("jlit_{}", tcount));
				let len = lit.len();

				stmts.push(quote_stmt!(cx, let $jname = $lit).unwrap());
				stmts.push(quote_stmt!(cx, c += $len).unwrap());

				push_stmts.push(quote_stmt!(cx, jval.push_str($jname)).unwrap());
			},
			//For replacements, convert the input first
			JsonPart::Replacement(ref ident, ref part) => {
				let name = match idents.get(ident) {
					Some(name) => name,
					None => {
						cx.span_err(sp, &format!("failed to find '{}' in the supplied replacement args", ident));
						return DummyResult::any(sp);
					}
				};

				let jname = token::str_to_ident(&format!("jrepl_{}", tcount));
				
				match *part {
					//For keys, emit the value surrounded by quotes
					ReplacementPart::Key => {
						stmts.push(quote_stmt!(cx, let $jname = {
							let mut tmpstr = String::with_capacity(&$name.len() + 2);
							tmpstr.push('\"');
							tmpstr.push_str(&$name.to_string());
							tmpstr.push('\"');

							tmpstr
						}).unwrap());
					},
					//For values, emit as inline json if the first non quote char is an object, emit inline, otherwise emit the serde string
					ReplacementPart::Value => {
						stmts.push(quote_stmt!(cx, let $jname = {
							let tmpstr = serde_json::to_string(&$name).unwrap();
							let len = tmpstr.len();
							let mut chars = tmpstr.chars();

							if len > 2 {
								let mut parsed = String::with_capacity(len);
								let char_quote = chars.next().unwrap();
								let char_obj = chars.next().unwrap();

								match char_obj {
									'{'|'[' => {
										parsed.push(char_obj);
										chars.next();
										for c in chars {
											match c {
												'\\' => (),
												_ => parsed.push(c)
											}
										}
										let _ = parsed.pop();
										parsed
									},
									_ => {
										parsed.push(char_quote);
										parsed.push(char_obj);
										parsed.push_str(chars.as_str());
										parsed
									}
								}
							}
							else {
							    String::from(chars.as_str())
							}
						}).unwrap());
					}
				}

				stmts.push(quote_stmt!(cx, c += $jname.len()).unwrap());
				push_stmts.push(quote_stmt!(cx, jval.push_str(&$jname)).unwrap());
			}
		}

		tcount += 1;
	};

	stmts.push(quote_stmt!(cx, let mut jval = String::with_capacity(c)).unwrap());
	stmts.extend_from_slice(&mut push_stmts);

	MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(quote_expr!(cx, jval)))))
}

#[derive(Debug, PartialEq)]
pub enum JsonPart {
	Literal(String),
	Replacement(String, ReplacementPart)
}

#[derive(Debug, PartialEq)]
pub enum ReplacementPart {
	Key,
	Value
}

impl ToString for JsonPart {
	fn to_string(&self) -> String {
		match *self {
			JsonPart::Literal(ref s) => 		s.clone(),
			JsonPart::Replacement(ref s, _) => 	s.clone()
		}
	}
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
				_ => 	parts.push(JsonPart::Replacement(id, ReplacementPart::Value))
			}
			parse_to_replacement(b, parts);
		}
	}
}
