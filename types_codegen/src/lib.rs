//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//!
//! # Date Formatting
//!
//! The `date_fmt!` macro will take a literal date format and parse it to a more efficient `Vec<Item>`.
//! This is used by date formatters.
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_types_codegen)]
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
//! # #![plugin(elastic_types_codegen)]
//! # extern crate chrono;
//! # fn main() {
//! let my_fmt = date_fmt!("%Y%m%dT%H%M%S%.3fZ");
//! # }
//! ```
//! 
//! # Links
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types_codegen/")]

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote, vec_push_all)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate chrono;

pub mod parsers;
pub mod date;
pub mod json;

use std::ops::Deref;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{ Stmt };
use syntax::ptr::P;
use syntax::ext::quote;
use syntax::ast::{ TokenTree, Expr };
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;
use chrono::format::{ Item, Fixed, Numeric, Pad };

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
	let tokens = date::to_tokens(&fmt[..]);
	let token_expr = cx.expr_vec(
		sp, 
		tokens.iter().map(|t| match *t {
			Item::Literal(c) => quote_expr!(cx, chrono::format::Item::Literal($c)),
			Item::Numeric(Numeric::Year, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Year, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Month, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Month, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Day, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Day, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Hour, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Hour, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Minute, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Minute, chrono::format::Pad::Zero)),
			Item::Numeric(Numeric::Second, Pad::Zero) => quote_expr!(cx, chrono::format::Item::Numeric(chrono::format::Numeric::Second, chrono::format::Pad::Zero)),
			Item::Fixed(Fixed::Nanosecond3) => quote_expr!(cx, chrono::format::Item::Fixed(chrono::format::Fixed::Nanosecond3)),
			_ => quote_expr!(cx, chrono::format::Item::Literal(""))
		}).collect()
	);

	MacEager::expr(quote_expr!(cx, {
		$token_expr
	}))
}

//TODO: Clean up
fn expand_json(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	//Get idents first, separated by commas
	let mut idents: Vec<syntax::ast::Ident> = Vec::new();

	//We expect a comma delimited list of idents
	//If none are found, we move on

	//TODO: Use BTreeMap instead of Vec. Doesn't support params out of order or reuse
	let mut ac = 0;
	for arg in args {
		match *arg {
			TokenTree::Token(_, token::Ident(s, _)) => {
				idents.push(s);
				ac += 1;
			},
			TokenTree::Token(_, token::Token::Comma) => ac += 1,
	        _ => break
		}
	}

	let json = syntax::print::pprust::tts_to_string(&args[ac..]);

	//If there are no idents, just emit the json string
	if ac == 0 {
		return MacEager::expr(cx.expr_str(sp, token::intern_and_get_ident(&json)))
	}
	
	let json_bytes = json.as_bytes();

	let mut tree = Vec::new();
	let json_parts = json::parse_to_replacement(json_bytes, &mut tree);

	let mut stmts: Vec<P<Stmt>> = Vec::new();
	let mut push_stmts: Vec<P<Stmt>> = Vec::new();

	stmts.push(quote_stmt!(cx, let mut c = 0).unwrap());

	let mut c = 0;
	let mut ic = 0;

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
			//TODO: replace `to_string` with `serde_json::to_str`
			json::JsonPart::Replacement(_, part) => {
				let name = idents[ic];
				let jn = format!("jrepl_{}", c);
				let jname = token::str_to_ident(&jn);
				
				match part {
					//For keys, emit the value surrounded by quotes
					//This may no longer be needed when using serde
					json::ReplacementPart::Key => {
						stmts.push(quote_stmt!(cx, let $jname = {
							let mut tmpstr = String::with_capacity(&$name.len() + 2);
							tmpstr.push('\"');
							let tmpjval = $name.to_string();
							tmpstr.push_str(&tmpjval);
							tmpstr.push('\"');

							tmpstr
						}).unwrap());
					},
					//For values, just emit the string value
					json::ReplacementPart::Value => {
						stmts.push(quote_stmt!(cx, let $jname = $name.to_string()).unwrap());
					}
				}

				stmts.push(quote_stmt!(cx, c += $jname.len()).unwrap());
				push_stmts.push(quote_stmt!(cx, jval.push_str(&$jname)).unwrap());

				ic += 1;
			}
		}

		c += 1;
	};

	stmts.push(quote_stmt!(cx, let mut jval = String::with_capacity(c)).unwrap());
	stmts.push_all(&mut push_stmts);

	MacEager::expr(cx.expr_block(cx.block(sp, stmts, Some(quote_expr!(cx, jval)))))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("date_fmt", expand_date_fmt);
    reg.register_macro("json", expand_json);
}
