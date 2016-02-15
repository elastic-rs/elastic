//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;
use ::gen::rust::{ ty, TyPathOpts };

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
pub fn url_fmt_decl(url: &str, param_parts: Vec<Ident>) -> (Ident, Stmt) {
	let ident = token::str_to_ident("url_fmtd");

	//Build up the macro arguments
	let mut args = vec![
		//The url format
		TokenTree::Token(
			DUMMY_SP, token::Token::Literal(
				token::Lit::Str_(token::intern(url)),
				None
			)
		),
		TokenTree::Token(
			DUMMY_SP, token::Token::Comma
		),
	];

	for part in param_parts {
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
		node: StmtKind::Decl(
			P(Decl {
				node: DeclKind::Local(
					P(Local {
						pat: P(Pat {
							id: DUMMY_NODE_ID,
							node: PatIdent(
								BindingMode::ByValue(Mutability::Immutable),
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
								node: ExprKind::Mac(Spanned {
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

/// Generate a series of statements to compose a url.
pub fn url_push_decl(url_parts: Vec<String>, param_parts: Vec<Ident>) -> (Ident, Vec<Stmt>) {
	let ident = token::str_to_ident("url_fmtd");

	//Get the string literal params
	let _url_parts: Vec<TokenTree> = url_parts.iter().map(|p| TokenTree::Token(
			DUMMY_SP, token::Token::Literal(
				token::Lit::Str_(token::intern(&p)),
				None
			)
		)
	)
	.collect();

	let url_decl = Stmt {
		node: StmtKind::Decl(
			P(Decl {
				node: DeclKind::Local(
					P(Local {
						pat: P(Pat {
							id: DUMMY_NODE_ID,
							node: PatIdent(
								BindingMode::ByValue(Mutability::Mutable),
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
								node: ExprKind::Call(
									P(Expr {
										id: DUMMY_NODE_ID,
										node: ExprKind::Path(
											None,
											Path {
												span: DUMMY_SP,
												global: false,
												segments: vec![
													PathSegment {
														identifier: token::str_to_ident("String"),
														parameters: PathParameters::none()
													},
													PathSegment {
														identifier: token::str_to_ident("with_capacity"),
														parameters: PathParameters::none()
													}
												]
											}
										),
										span: DUMMY_SP,
										attrs: None
									}),
									//TODO: Single param; the length of each param
									//This is just a shim to demo getting the length of params
									param_parts.iter().map(|item| get_item_length(*item)).collect()
								),
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

	(ident, vec![url_decl])
}

fn get_item_length(item: Ident) -> P<Expr> {
	P(Expr {
		id: DUMMY_NODE_ID,
		node: ExprKind::MethodCall(
			Spanned {
				span: DUMMY_SP,
				node: token::str_to_ident("len")
			},
			Vec::new(),
			vec![
				P(Expr {
					id: DUMMY_NODE_ID,
					node: ExprKind::Path(
						None,
						Path {
							span: DUMMY_SP,
							global: false,
							segments: vec![
								PathSegment {
									identifier: item,
									parameters: PathParameters::none()
								}
							]
						}
					),
					span: DUMMY_SP,
					attrs: None
				})
			]
		),
		span: DUMMY_SP,
		attrs: None
	})
}