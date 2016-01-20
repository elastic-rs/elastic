//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, Span };
use syntax::ptr::P;

/// Generate a statement to replace the params in a url.
/// 
/// Generates statements of the form `let url_fmtd = format!(url, parts[0], ..., parts[n]);`. 
/// Returns the `Ident` for the formatted string and the `Stmt` that declares it.
pub fn url_fmt_dec(_url: &str, _parts: Vec<Ident>, sp: Span) -> (Ident, Stmt) {
	let ident = token::str_to_ident("url_fmtd");

	//Build up the macro arguments
	let mut args = vec![
		//The url format
		TokenTree::Token(
			sp, token::Token::Literal(
				token::Lit::Str_(token::intern(_url)),
				None
			)
		),
		TokenTree::Token(
			sp, token::Token::Comma
		),
	];

	for part in _parts {
		args.push(TokenTree::Token(
			sp, token::Token::Ident(
				part, 
				token::IdentStyle::Plain
			)
		));
		args.push(TokenTree::Token(
			sp, token::Token::Comma
		));
	}

	let stmt = Stmt {
		node: Stmt_::StmtDecl(
			P(Decl {
				node: Decl_::DeclLocal(
					P(Local {
						pat: P(Pat {
							id: DUMMY_NODE_ID,
							node: PatIdent(
								BindingMode::ByValue(Mutability::MutImmutable),
								Spanned {
									span: sp,
									node: ident
								},
								None
								),
							span: sp
						}),
						ty: None,
						init: Some(
							P(Expr {
								id: DUMMY_NODE_ID,
								node: Expr_::ExprMac(Spanned {
									span: sp,
									node: Mac_ {
										path: Path {
											span: sp,
											global: false,
											segments: vec![
												PathSegment {
													identifier: token::str_to_ident("format"),
													parameters: PathParameters::none()
												}
											]
										},
										tts: args,
										ctxt: SyntaxContext(0)
									}
								}),
								span: sp,
								attrs: None
							})
						),
						id: DUMMY_NODE_ID,
						span: sp,
						attrs: None
					})
				),
				span: sp
			}),
			DUMMY_NODE_ID
		),
		span: sp
	};

    (ident, stmt)
}