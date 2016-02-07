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
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate chrono;

pub mod date;

use std::ops::Deref;
use syntax::codemap::Span;
use syntax::parse::token;
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

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("date_fmt", expand_date_fmt);
}
