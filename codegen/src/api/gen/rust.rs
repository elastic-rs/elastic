//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use std::collections::BTreeMap;
use std::error;
use std::fmt;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;
use ::api::ast as api;
use ::gen::rust::{ ty, build_ty, TyPathOpts };
use super::{ parse_path_params, parse_mod_path, ApiParseError };

/// A single function for a url path.
pub struct UrlFn<'a> {
	/// The name of the function.
	pub name: String,
	/// The HTTP verb for the fn url
    pub method: &'a api::HttpVerb,
    /// The url path for the function.
	pub path: &'a str,
	/// The replacement parts in the url path.
	pub parts: BTreeMap<String, &'a api::Part>
}

#[derive(Debug)]
enum ApiGenErrorKind {
	Parse(ApiParseError),
	Other(String)
}

/// Represents an error encountered during parsing.
/// 
/// This could include errors while reading the file or deserialising the contents.
#[derive(Debug)]
pub struct ApiGenError {
	kind: ApiGenErrorKind
}

impl fmt::Display for ApiGenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.kind {
			ApiGenErrorKind::Parse(ref err) => write!(f, "Parse error: {:?}", err),
			ApiGenErrorKind::Other(ref err) => write!(f, "Error: {}", err)
		}
	}
}

impl error::Error for ApiGenError {
	fn description(&self) -> &str {
		match self.kind {
			ApiGenErrorKind::Parse(_) => "Error parsing API data",
			ApiGenErrorKind::Other(ref err) => &err[..]
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match self.kind {
			ApiGenErrorKind::Parse(_) => None,
			ApiGenErrorKind::Other(_) => None
		}
	}
}

impl From<String> for ApiGenError {
	fn from(err: String) -> ApiGenError {
		ApiGenError {
			kind: ApiGenErrorKind::Other(err)
		}
	}
}

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
									token::intern(&format!("//! {}", self.documentation))
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

	/// Gets the name of the Endpoint if it's set or returns an empty string.
	pub fn get_name<'a>(&'a self) -> &'a str {
		match self.name {
			Some(ref n) => n,
			None => ""
		}
	}
	
	/// Get the module name for all functions in this endpoint.
	/// 
	/// The name is a hierarchy of modules, which are determined by splitting the endpoint name on '.'.
	/// So for example, the `indices.shard_stores` endpoint has the following module path:
	/// 
	/// - `indices`
	/// - `shard_stores`
	pub fn get_mod_path(&self) -> Result<Vec<String>, ApiGenError> {
		parse_mod_path(self.get_name()).map_err(|e| e.into())
	}

	/// Get the function definitions for this endpoint.
	/// 
	/// Each possible url path is considered a function.
	/// Names take the (rather verbose) form: `{http_verb}_{endpoint_name}_{param_1}_{param_n}`.
	/// So for example, the `count` endpoint will produce the following `fn` names:
	/// 
	/// - `get`
	/// - `post`
	/// - `get_index`
	/// - `post_index`
	/// - `get_index_type`
	/// - `post_index_type`
	/// 
	/// This is to try and prevent collisions with the names where not a lot of info about each endpoint is available.
	pub fn get_fns<'a>(&'a self) -> Result<Vec<UrlFn<'a>>, ApiGenError> {
		let mut fns = Vec::new();
		for path in &self.url.paths {
			//Parse the params used by this path
			let mut fn_parts = BTreeMap::new();
			let params = try!(parse_path_params(&path));

			for param in params.iter() {
				let param = param.to_owned();
				match self.url.parts.get(&param) {
					Some(part) => {
						let _ = fn_parts.insert(param, part);
					},
					None => ()
				};
			}

			//Return a function for each method on the url
			for method in &self.methods {
				let (method_name, m) = match *method {
					api::HttpVerb::Head => ("head", method),
					api::HttpVerb::Post => ("post", method),
					api::HttpVerb::Put => ("put", method),
					api::HttpVerb::Patch => ("patch", method),
					api::HttpVerb::Delete => ("delete", method),
					_ => ("get", method)
				};

				let path_name = match params.len() {
					//No params, return default name
					0 => String::new(),
					//If params are set, use to build up a unique fn name
					_ => format!("_{}", params.to_vec().join("_"))
				};

				let name = format!("{}{}", method_name, path_name);

				//Names take the (rather verbose) form {http_verb}_{endpoint_name}_{param_1}_{param_n}
				fns.push(UrlFn {
					name: name,
                    method: m,
					path: &path,
					parts: fn_parts.clone()
				})
			}
		}

		Ok(fns)
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
/// let mut url_fmtd = String::with_capacity(base.len() + 1 + 8 + index.len() + name.len());
/// 
/// url_fmtd.push_str(base);
/// url_fmtd.push_str("/");
/// url_fmtd.push_str(&index);
/// url_fmtd.push_str("/_alias/");
/// url_fmtd.push_str(name);
/// 
/// url_fmtd
/// ```
/// 
/// Right now, this method expects that param idents are `&str`s. This will be updated in the future.
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
		let url_part_ids: Vec<Ident> = url_parts
			.into_iter()
			.map(|p| token::str_to_ident(&(format!("\"{}\"", p))))
			.collect();

		//Get the length expression
		//This is the sum of all parts and args

		//Sum the url parts
		let mut url_iter = url_part_ids
			.iter()
			.map(|&ident| ident);

		let mut add_expr = len_add(
			len_expr(ident_expr(url_base)), 
			len_of_ident_expr(url_iter.next().unwrap())
		);
		for url_part in url_iter {
			add_expr = len_add(add_expr, len_of_ident_expr(url_part));
		}

		//Sum the url params
		let mut param_part_ids = Vec::new();
		for url_param in param_parts {
			param_part_ids.push(url_param.clone());
			add_expr = len_add(add_expr, len_expr(ident_expr(url_param)));
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

		let mut stmts = Vec::new();

		//Push the declaration statement and the base url
		stmts.push(url_decl);
		stmts.push(push_stmt(ident, ident_expr(url_base)));

		//Thread through each url part and param, pushing a part, then a param to keep the url in order
		let (mut part_iter, mut param_iter) = (
			url_part_ids.iter().map(|&ident| ident),
			param_part_ids.iter().map(|&ident| ident)
		);
		let mut cont = true;
		while cont {
			let part = part_iter.next();
			let param = param_iter.next();

			match (part, param) {
				//If there's a part and a param, push the part first
				(Some(part), Some(param)) => {
					stmts.push(push_stmt(ident, ident_expr(part)));
					stmts.push(push_stmt(ident, ident_expr(param)));
				},
				(Some(part), None) => stmts.push(push_stmt(ident, ident_expr(part))),
				(None, Some(param)) => stmts.push(push_stmt(ident, ident_expr(param))),
				//If there are no url parts or params left, stop iterating
				(None, None) => cont = false
			}
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

/// Gets the length of an ident string as a literal int
/// 
/// This method expects the ident is quoted, so 2 is subtracted from the length.
fn len_of_ident_expr(item: Ident) -> P<Expr> {
    let len = match item.name.as_str().len() {
        x if x <= 2 => 0,
        x => x - 2  
    };
    
    P(Expr {
		id: DUMMY_NODE_ID,
		node: ExprKind::Lit(
			P(Spanned {
                span: DUMMY_SP,
                node: LitKind::Int(len as u64, LitIntType::Unsuffixed)
            })
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
			to_add
		),
		span: DUMMY_SP,
		attrs: None
	})
}
