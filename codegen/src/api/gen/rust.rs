//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;

/// Generate a statement to replace the params in a url.
/// 
/// Generates statements of the form `let url_fmtd = format!(url, parts[0], ..., parts[n]);`. 
/// Returns the `Ident` for the formatted string and the `Stmt` that declares it.
/// 
/// # Examples
/// 
/// ```
/// # #![feature(rustc_private)]
/// # extern crate syntax;
/// # extern crate elastic_codegen;
/// # fn main() {
/// use syntax::parse::token;
/// use syntax::print::pprust;
/// use elastic_codegen::api::gen::rust::*;
/// 
/// //Generate the statement
/// let (ident, stmt) = url_fmt_decl("/{}/_alias/{}", vec![
/// 	token::str_to_ident("index"),
/// 	token::str_to_ident("name")
/// ]);
/// 
/// //Print the result: 'let url_fmtd = format!("/{}/_alias/{}" , index , name ,);'
/// let result = pprust::stmt_to_string(&stmt);
/// println!("{}", result);
/// # }
/// ```
pub fn url_fmt_decl(_url: &str, _parts: Vec<Ident>) -> (Ident, Stmt) {
	let ident = token::str_to_ident("url_fmtd");

	//Build up the macro arguments
	let mut args = vec![
		//The url format
		TokenTree::Token(
			DUMMY_SP, token::Token::Literal(
				token::Lit::Str_(token::intern(_url)),
				None
			)
		),
		TokenTree::Token(
			DUMMY_SP, token::Token::Comma
		),
	];

	for part in _parts {
		args.push(TokenTree::Token(
			DUMMY_SP, token::Token::Ident(
				part, 
				token::IdentStyle::Plain
			)
		));
		args.push(TokenTree::Token(
			DUMMY_SP, token::Token::Comma
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
									span: DUMMY_SP,
									node: ident
								},
								None
								),
							span: DUMMY_SP
						}),
						ty: None,
						init: Some(
							P(Expr {
								id: DUMMY_NODE_ID,
								node: Expr_::ExprMac(Spanned {
									span: DUMMY_SP,
									node: Mac_ {
										path: Path {
											span: DUMMY_SP,
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
								span: DUMMY_SP,
								attrs: None
							})
						),
						id: DUMMY_NODE_ID,
						span: DUMMY_SP,
						attrs: None
					})
				),
				span: DUMMY_SP
			}),
			DUMMY_NODE_ID
		),
		span: DUMMY_SP
	};

    (ident, stmt)
}

fn url_push_decl(_url: &str, _parts: Vec<Ident>) -> (Ident, Vec<Stmt>) {
	//See: format_url_push bench
	panic!("implement")
}
