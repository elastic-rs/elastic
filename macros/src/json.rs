use std::collections::BTreeMap;
use syntax;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ast::{ Stmt, Ident };
use syntax::ast::TokenTree;
use syntax::ext::base::{ ExtCtxt, MacResult, MacEager, DummyResult };
use syntax::ext::build::AstBuilder;
use ::parse::*;

//TODO: Clean this up, it's an awful macro in this state. Look at using serde_json for intermediate representation
pub fn expand_json(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	let (json, repl_args) = parse_tt(&args);
	
	//If there are no idents, just emit the json string
	if repl_args.len() == 0 {
		let str_lit = cx.expr_str(sp, token::intern_and_get_ident(&json));
		MacEager::expr(P(quote_expr!(cx, String::from($str_lit)).unwrap()))
	}
	else {
		expand_repl(cx, sp, &json, &repl_args)
	}
}

fn parse_tt(args: &[TokenTree]) -> (String, BTreeMap<String, Ident>) {
	let mut repl_args: BTreeMap<String, Ident> = BTreeMap::new();
	let mut ac = 0;
	for arg in args {
		match *arg {
			TokenTree::Token(_, token::Ident(s, _)) => {
				repl_args.insert(s.to_string(), s);
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

	(sanitised, repl_args)
}

fn expand_repl(cx: &mut ExtCtxt, sp: Span, json: &str, repl_args: &BTreeMap<String, Ident>) -> Box<MacResult+'static> {
	let mut repl_json = Vec::new();
	parse_to_replacement(json.as_bytes(), &mut repl_json);

	let mut stmts: Vec<Stmt> = Vec::new();
	let mut push_stmts: Vec<Stmt> = Vec::new();

	stmts.push(quote_stmt!(cx, let mut c = 0).unwrap());

	let mut repl_count = 0;
	for t in repl_json {
		match t {
			JsonPart::Literal(ref lit) => {
				match expand_repl_lit(cx, &mut stmts, &mut push_stmts, repl_count, lit) {
					Err(e) => {
						cx.span_err(sp, &e);
						return DummyResult::any(sp);
					},
					_ => ()
				}
			},
			JsonPart::Replacement(ref ident, ref part) => {
				match expand_repl_arg(cx, &mut stmts, &mut push_stmts, repl_count, repl_args, ident, part) {
					Err(e) => {
						cx.span_err(sp, &e);
						return DummyResult::any(sp);
					},
					_ => ()
				}
			}
		}

		repl_count += 1;
	};

	stmts.push(quote_stmt!(cx, let mut jval = String::with_capacity(c)).unwrap());
	stmts.extend_from_slice(&mut push_stmts);

	MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(quote_expr!(cx, jval)))))
}

fn expand_repl_lit(cx: &mut ExtCtxt, stmts: &mut Vec<Stmt>, push_stmts: &mut Vec<Stmt>, repl_count: usize, repl: &str) -> Result<(), String> {
	let arg_key = token::str_to_ident(&format!("jlit_{}", repl_count));
	let len = repl.len();

	stmts.push(quote_stmt!(cx, let $arg_key = $repl).unwrap());
	stmts.push(quote_stmt!(cx, c += $len).unwrap());

	push_stmts.push(quote_stmt!(cx, jval.push_str($arg_key)).unwrap());

	Ok(())
}

fn expand_repl_arg(cx: &mut ExtCtxt, stmts: &mut Vec<Stmt>, push_stmts: &mut Vec<Stmt>, repl_count: usize, repl_args: &BTreeMap<String, Ident>, ident: &str, part: &ReplacementPart) -> Result<(), String> {
	let arg_val = try!(repl_args.get(ident).ok_or(format!("failed to find '{}' in the supplied replacement args", ident)));
	let arg_key = token::str_to_ident(&format!("jrepl_{}", repl_count));

	match *part {
		ReplacementPart::Key => stmts.push(expand_repl_key(cx, arg_key, *arg_val).unwrap()),
		ReplacementPart::Value => stmts.push(expand_repl_value(cx, arg_key, *arg_val).unwrap())
	}

	stmts.push(quote_stmt!(cx, c += $arg_key.len()).unwrap());
	push_stmts.push(quote_stmt!(cx, jval.push_str(&$arg_key)).unwrap());

	Ok(())
}

fn expand_repl_key(cx: &mut ExtCtxt, arg_key: Ident, arg_val: Ident) -> Option<Stmt> {
	quote_stmt!(cx, let $arg_key = {
		let mut tmpstr = String::with_capacity(&$arg_val.len() + 2);
		tmpstr.push('\"');
		tmpstr.push_str(&$arg_val.to_string());
		tmpstr.push('\"');
		tmpstr
	})
}

fn expand_repl_value(cx: &mut ExtCtxt, arg_key: Ident, arg_val: Ident) -> Option<Stmt> {
	quote_stmt!(cx, let $arg_key = {
		let tmpstr = serde_json::to_string(&$arg_val).unwrap();
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
	})
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
