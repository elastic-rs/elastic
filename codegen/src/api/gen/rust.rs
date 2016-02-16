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
/// let (ident, stmt) = url_fmt_decl(
/// 	"/{}/_alias/{}", 
/// 	token::str_to_ident("http://localhost:9200"), 
/// 	vec![
/// 		token::str_to_ident("index"),
/// 		token::str_to_ident("name")
/// 	]
/// );
/// 
/// //Print the result: 'let url_fmtd = format!("/{}/_alias/{}" , index , name ,);'
/// let result = pprust::stmt_to_string(&stmt);
/// println!("{}", result);
/// # }
/// ```
pub fn url_fmt_decl(url: &str, url_base: Ident, param_parts: Vec<Ident>) -> (Ident, Stmt) {
	let ident = token::str_to_ident("url_fmtd");

	//Build up the macro arguments
	let mut args = vec![
		//The url format
		TokenTree::Token(
			DUMMY_SP, token::Token::Literal(
				token::Lit::Str_(token::intern(&format!("{{}}{}", url))),
				None
			)
		),
		TokenTree::Token(
			DUMMY_SP, token::Token::Comma
		),
		//The url base
		TokenTree::Token(
			DUMMY_SP, token::Token::Ident(
				url_base, 
				token::IdentStyle::Plain
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
/// 
/// Generates a series of statements of the form:
/// 
/// ```text
/// let mut url_fmtd = String::with_capacity(base.len() + "/".len() + "/_alias/".len() + index.len() + name.len());
/// 
/// url_fmtd.push_str(&base);
/// url_fmtd.push_str("/");
/// url_fmtd.push_str(&index);
/// url_fmtd.push_str("/_alias/");
/// url_fmtd.push_str(&name);
/// 
/// url_fmtd
/// ```
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
/// let (ident, stmt) = url_push_decl(
/// 	token::str_to_ident("base"), 
/// 	vec![
/// 		"/_alias/".to_string(),
/// 		"/".to_string()
/// 	],
/// 	vec![
/// 		token::str_to_ident("index"),
/// 		token::str_to_ident("name")
/// 	]
/// );
/// # }
/// ```
pub fn url_push_decl(url_base: Ident, url_parts: Vec<String>, param_parts: Vec<Ident>) -> (Ident, Vec<Stmt>) {
	let ident = token::str_to_ident("url_fmtd");

	//Get the string literal params
	let url_part_idents: Vec<Ident> = url_parts
		.iter()
		.map(|p| token::str_to_ident(&(format!("\"{}\"", p))))
		.collect();
	let url_part_tokens: Vec<TokenTree> = url_parts.iter().map(|p| TokenTree::Token(
			DUMMY_SP, token::Token::Literal(
				token::Lit::Str_(token::intern(&p)),
				None
			)
		)
	)
	.collect();

	//Get the length expression
	//This is the sum of all parts and args

	//Sum the url parts
	let mut url_iter = url_part_idents.iter();

	let mut add_expr = len_add(
		len_expr(url_base), 
		url_iter.next().unwrap().clone()
	);
	for url_part in url_iter {
		add_expr = len_add(add_expr, url_part.clone());
	}

	//Sum the url params
	for url_param in param_parts {
		add_expr = len_add(add_expr, url_param);
	}

	//Get the declaration statement
	//let url_fmtd = String::with_capacity(url_base.len() + "/".len() + param1.len() + "/part/".len() + param2.len());
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
									vec![add_expr]
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

fn len_expr(item: Ident) -> P<Expr> {
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

fn len_add(current_add: P<Expr>, to_add: Ident) -> P<Expr> {
	P(Expr {
		id: DUMMY_NODE_ID,
		node: ExprKind::Binary(
			Spanned {
				span: DUMMY_SP,
				node: BinOpKind::Add
			},
			current_add,
			len_expr(to_add)
		),
		span: DUMMY_SP,
		attrs: None
	})
}