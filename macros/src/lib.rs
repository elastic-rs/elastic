//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//! 
//! # Json Parsing
//! 
//! The `json!` macro will take an inline token tree and serialise it as json:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let json = json!({
//! 	"query": {
//! 		"filtered": {
//! 			"query": {
//! 				"match_all": {}
//! 			},
//! 			"filter": {
//! 				"geo_distance": {
//! 					"distance": "20km",
//! 					"location": {
//! 						"lat": 37.776,
//! 						"lon": -122.41
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//! 
//! This will also work for unquoted keys for something a bit more `rusty`:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let json = json!({
//! 	query: {
//! 		filtered: {
//! 			query: {
//! 				match_all: {}
//! 			},
//! 			filter: {
//! 				geo_distance: {
//! 					distance: "20km",
//! 					location: {
//! 						lat: 37.776,
//! 						lon: -122.41
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//! 
//! Json values can be spliced in to the result if they implement `serde::Serialize`:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let query = "match_all";
//! let dist = "20km";
//! let lat = 37.776;
//! let lon = -122.41;
//! 
//! let json = json!(key, dist, lat, lon {
//! 	query: {
//! 		filtered: {
//! 			query: {
//! 				$query: {}
//! 			},
//! 			filter: {
//! 				geo_distance: {
//! 					distance: $dist,
//! 					location: {
//! 						lat: $lat,
//! 						lon: $lon
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//!
//! # Date Formatting
//!
//! The `date_fmt!` macro will take a literal date format and parse it to a more efficient `Vec<Item>`.
//! This is used by date formatters.
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # extern crate chrono;
//! # fn main() {
//! let my_fmt = date_fmt!("yyyyMMddTHHmmss.SSSZ");
//! # }
//! ```
//! 
//! This also works for `chrono` date formats:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # extern crate chrono;
//! # fn main() {
//! let my_fmt = date_fmt!("%Y%m%dT%H%M%S%.3fZ");
//! # }
//! ```
//! 
//! # Links
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_macros/")]

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote, plugin)]
#![plugin(serde_macros)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod parse;
//TODO: Feature gate: types
pub mod date_format;
pub mod json;

use std::collections::BTreeMap;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ast::{ Stmt };
use syntax::ast::TokenTree;
use syntax::ext::base::{ ExtCtxt, MacResult, DummyResult, MacEager };
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;
use chrono::format::{ Item, Fixed, Numeric, Pad };

//TODO: Feature gate: types
fn expand_date_fmt(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	let mut fmt = String::new();

	for arg in args {
		let _fmt = match *arg {
			TokenTree::Token(_, token::Literal(token::Lit::Str_(s), _)) => s.to_string(),
			_ => {
				cx.span_err(sp, "argument should be a single identifier");
				return DummyResult::any(sp);
			}
		};

		fmt.push_str(&_fmt[..]);
	}

	//Build up the token tree
	let tokens = date_format::to_tokens(&fmt[..]);
	let token_expr = cx.expr_vec(sp, tokens.iter().map(|t| date_format::Formatter::to_stmt(t, cx)).collect());

	MacEager::expr(quote_expr!(cx, {
		$token_expr
	}))
}

//TODO: Clean this up
fn expand_json(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
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
	json::sanitise(json.as_bytes(), &mut sanitised);
	
	//If there are no idents, just emit the json string
	if idents.len() == 0 {
        let str_lit = cx.expr_str(sp, token::intern_and_get_ident(&sanitised));
		return MacEager::expr(P(quote_expr!(cx, String::from($str_lit)).unwrap()))
	}
	
	let mut tree = Vec::new();
	json::parse_to_replacement(sanitised.as_bytes(), &mut tree);

	let mut stmts: Vec<Stmt> = Vec::new();
	let mut push_stmts: Vec<Stmt> = Vec::new();

	stmts.push(quote_stmt!(cx, let mut c = 0).unwrap());

	let mut c = 0;
	for t in tree {
		match t {
			//For literals, emit the string value
			json::JsonPart::Literal(ref lit) => {
				let s = lit.clone();

				let jn = format!("jlit_{}", c);
				let jname = token::str_to_ident(&jn);

				stmts.push(quote_stmt!(cx, let $jname = $s).unwrap());
				stmts.push(quote_stmt!(cx, c += $jname.len()).unwrap());

				push_stmts.push(quote_stmt!(cx, jval.push_str($jname)).unwrap());
			},
			//For replacements, convert the input first
			json::JsonPart::Replacement(ref ident, ref part) => {
				let name = idents.get(ident).unwrap();
				let jn = format!("jrepl_{}", c);
				let jname = token::str_to_ident(&jn);
				
				match *part {
					//For keys, emit the value surrounded by quotes
					//This may no longer be needed when using serde
					json::ReplacementPart::Key => {
						stmts.push(quote_stmt!(cx, let $jname = {
							let mut tmpstr = String::with_capacity(&$name.len() + 2);
							tmpstr.push('\"');
							let tmpjval = serde_json::to_string(&$name).unwrap();
							tmpstr.push_str(&tmpjval);
							tmpstr.push('\"');

							tmpstr
						}).unwrap());
					},
					//For values, just emit the string value
					json::ReplacementPart::Value => {
						stmts.push(quote_stmt!(cx, let $jname = serde_json::to_string(&$name).unwrap()).unwrap());
					}
				}

				stmts.push(quote_stmt!(cx, c += $jname.len()).unwrap());
				push_stmts.push(quote_stmt!(cx, jval.push_str(&$jname)).unwrap());
			}
		}

		c += 1;
	};

	stmts.push(quote_stmt!(cx, let mut jval = String::with_capacity(c)).unwrap());
	stmts.extend_from_slice(&mut push_stmts);

	MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(quote_expr!(cx, jval)))))
}

//TODO: Add macros for codegenning Serialize for ElasticMapping. Possibly feature-gate

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("date_fmt", expand_date_fmt);
    reg.register_macro("json", expand_json);
}