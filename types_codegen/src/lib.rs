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
			Item::Literal(c) => quote_expr!(cx, Item::Literal($c)),
			Item::Numeric(Numeric::Year, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Year, Pad::Zero)),
			Item::Numeric(Numeric::Month, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Month, Pad::Zero)),
			Item::Numeric(Numeric::Day, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Day, Pad::Zero)),
			Item::Numeric(Numeric::Hour, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Hour, Pad::Zero)),
			Item::Numeric(Numeric::Minute, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Minute, Pad::Zero)),
			Item::Numeric(Numeric::Second, Pad::Zero) => quote_expr!(cx, Item::Numeric(Numeric::Second, Pad::Zero)),
			Item::Fixed(Fixed::Nanosecond3) => quote_expr!(cx, Item::Fixed(Fixed::Nanosecond3)),
			_ => quote_expr!(cx, Item::Literal(""))
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