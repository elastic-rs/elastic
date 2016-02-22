//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use std::collections::BTreeMap;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;
use ::api::ast as api;
use ::gen::rust::{ ty, build_ty, TyPathOpts };
use super::{ parse_path_params, parse_mod_path, ModPathParseError };

impl api::Endpoint {
	/// Get the Rust doc comment for this endpoint.
	/// 
	/// This is the `documentation` value formatted as a Rust doc comment.
	pub fn get_doc(&self) -> Attribute {
		Spanned {
			span: DUMMY_SP,
			node: Attribute_ {
				id: AttrId(0),
				style: AttrStyle::Inner,
				value: P(Spanned {
					span: DUMMY_SP,
					node: MetaItemKind::NameValue(
						token::InternedString::new(""),
						Spanned { 
							span: DUMMY_SP,
							node: LitKind::Str(
								token::InternedString::new_from_name(
									token::intern(&self.documentation)
								),
								StrStyle::Cooked
							)
						}
					)
				}),
				is_sugared_doc: true
			}
		}
	}
	
	/// Get the module name for all functions in this endpoint.
	pub fn get_mod_path(&self) -> Result<Vec<String>, ModPathParseError> {
		parse_mod_path(match self.name {
			Some(ref n) => n,
			None => ""
		})
	}
}

/// A single function for a url path.
pub struct UrlFn<'a> {
	/// The name of the function.
	pub name: &'a str,
	/// The url path for the function.
	pub path: &'a str,
	/// The replacement parts in the url path.
	pub parts: BTreeMap<String, &'a api::Part>
}

impl api::Url {
	/// Get the function definitions for this endpoint.
	/// 
	/// Each possible url path is considered a function.
	pub fn get_fns<'a>(&'a self) -> Vec<Option<UrlFn<'a>>> {
		self.paths.iter()
			.map(|p| {
				let mut parts = BTreeMap::new();
				
				let params = parse_path_params(p);
				if params.is_err() {
					return None
				}
				
				for param in params.unwrap() {
					match self.parts.get(&param) {
						Some(part) => {
							let _ = parts.insert(param, part);
						},
						None => ()
					};
				}

				//TODO: get name for the fn
				
				Some(UrlFn {
					name: "",
					path: p,
					parts: parts
				})
			})
		.collect()
	}
}

impl <'a> Into<Option<Ty>> for api::Type<'a> {
	fn into(self) -> Option<Ty> {
		match self {
			api::Type::Bool => Some(ty::<bool>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Long) => Some(ty::<i64>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Int) => Some(ty::<i32>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Short) => Some(ty::<i16>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Byte) => Some(ty::<u8>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Double) => Some(ty::<f32>(TyPathOpts::default())),
			api::Type::Number(api::NumberKind::Float) => Some(ty::<f32>(TyPathOpts::default())),
			api::Type::Str => Some(ty::<String>(TyPathOpts::default())),
			api::Type::Bin => Some(ty::<Vec<u8>>(TyPathOpts::default())),
			api::Type::Other(t) => Some(build_ty(t)),
			_ => None
		}
	}
}

/// Generate a statement to replace the params in a url.
/// 
/// Generates statements of the form `let url_fmtd = format!(url, base, parts[0], ..., parts[n]);`. 
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
/// 	"{}/{}/_alias/{}", 
/// 	token::str_to_ident("base"), 
/// 	vec![
/// 		token::str_to_ident("index"),
/// 		token::str_to_ident("name")
/// 	]
/// );
/// 
/// //Print the result: 'let url_fmtd = format!("{}/{}/_alias/{}" , base , index , name ,);'
/// let result = pprust::stmt_to_string(&stmt);
/// println!("{}", result);
/// # }
/// ```
pub fn url_fmt_decl<I>(url: &str, url_base: Ident, param_parts: I) -> (Ident, Stmt)
	where I: IntoIterator<Item=Ident> {
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
								node: PatKind::Ident(
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
/// 		"/_alias/",
/// 		"/"
/// 	],
/// 	vec![
/// 		token::str_to_ident("index"),
/// 		token::str_to_ident("name")
/// 	]
/// );
/// # }
/// ```
pub fn url_push_decl<'a, I, K>(url_base: Ident, url_parts: I, param_parts: K) -> (Ident, Vec<Stmt>) 
	where I: IntoIterator<Item=&'a str>, K: IntoIterator<Item=Ident> {
		let ident = token::str_to_ident("url_fmtd");

		//Get the string literal params
		let url_part_ids_tokens: Vec<(Ident, TokenTree)> = url_parts
			.into_iter()
			.map(|p| {(
				token::str_to_ident(&(format!("\"{}\"", p))), 
				TokenTree::Token(
					DUMMY_SP, token::Token::Literal(
						token::Lit::Str_(token::intern(p)),
						None
					)
				)
			)})
			.collect();

		//Get the length expression
		//This is the sum of all parts and args

		//Sum the url parts
		let mut url_iter = url_part_ids_tokens
			.iter()
			.map(|&(ident, _)| ident);

		let mut add_expr = len_add(
			len_expr(ident_expr(url_base)), 
			ident_expr(url_iter.next().unwrap())
		);
		for url_part in url_iter {
			add_expr = len_add(add_expr, ident_expr(url_part));
		}

		//Sum the url params
		for url_param in param_parts {
			add_expr = len_add(add_expr, ident_expr(url_param));
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
								node: PatKind::Ident(
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

		//TODO: Get str_push() stmts in order
		//base, (lit, param)*
		
		let mut stmts = Vec::new();
		stmts.push(url_decl);

		stmts.push(push_stmt(ident, ident_expr_brw(url_base)));

		for url_part in url_part_ids_tokens
			.iter()
			.map(|&(ident, _)| ident) {
				stmts.push(push_stmt(ident, ident_expr(url_part)));
		}

		(ident, stmts)
}

/// Gets an ident as a borrow
fn ident_expr_brw(item: Ident) -> P<Expr> {
	P(Expr {
		id: DUMMY_NODE_ID,
		node: ExprKind::AddrOf(
			Mutability::Immutable,
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
		),
		span: DUMMY_SP,
		attrs: None
	})
}

/// Gets an ident as an expression
fn ident_expr(item: Ident) -> P<Expr> {
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
}

/// Gets an expression of the form 'item.len()' where item is an ident or string literal.
fn len_expr(item: P<Expr>) -> P<Expr> {
	P(Expr {
		id: DUMMY_NODE_ID,
		node: ExprKind::MethodCall(
			Spanned {
				span: DUMMY_SP,
				node: token::str_to_ident("len")
			},
			Vec::new(),
			vec![item]
		),
		span: DUMMY_SP,
		attrs: None
	})
}

/// Gets an expression of the form 'url.push_str(item)' where item is an ident or string literal.
fn push_stmt(url_ident: Ident, item: P<Expr>) -> Stmt {
	Spanned {
		span: DUMMY_SP,
		node: StmtKind::Expr(
			P(Expr {
				id: DUMMY_NODE_ID,
				node: ExprKind::MethodCall(
					Spanned {
						span: DUMMY_SP,
						node: token::str_to_ident("push_str")
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
											identifier: url_ident,
											parameters: PathParameters::none()
										}
									]
								}
							),
							span: DUMMY_SP,
							attrs: None
						}),
						item
					]
				),
				span: DUMMY_SP,
				attrs: None
			}
		), 
		DUMMY_NODE_ID
		)
	}
}

/// Adds the result of `len_expr(to_add)` to the `current_add`, for chaining addition ops together.
fn len_add(current_add: P<Expr>, to_add: P<Expr>) -> P<Expr> {
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
