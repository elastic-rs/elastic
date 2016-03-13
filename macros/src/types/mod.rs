extern crate chrono;

pub mod date_format;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ ExtCtxt, MacResult, DummyResult, MacEager };
use syntax::ext::build::AstBuilder;

pub fn expand_date_fmt(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	let mut fmt = String::new();

	for arg in args {
		let _fmt = match *arg {
			TokenTree::Token(_, token::Literal(token::Lit::Str_(s), _)) => s.to_string(),
			_ => {
				cx.span_err(sp, "argument should be a single identifier");
				return DummyResult::any(sp);
			}
		};

		fmt.push_str(&_fmt);
	}

	//Build up the token tree
	let tokens = date_format::to_tokens(&fmt);
	let token_expr = cx.expr_vec(sp, tokens.iter().map(|t| date_format::Formatter::to_stmt(t, cx)).collect());

	MacEager::expr(quote_expr!(cx, { $token_expr }))
}

//TODO: Add macros for codegenning Serialize for ElasticMapping