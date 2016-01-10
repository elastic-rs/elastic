//! Elasticsearch API Rust Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to Rust source code.

use syntax::ast::*;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::parse::token::intern;
use syntax::ptr::P;
use ::gen::rust::*;

/// Generate a statement to replace the params in a url.
/// 
/// Generates statements of the form `let url_fmtd = format!(url, parts[0], ..., parts[n]);`. 
/// Returns the `Ident` for the formatted string and the `Stmt` that declares it.
pub fn url_fmt_dec(url: &str, parts: Vec<Ident>) -> (Ident, Stmt) {
	let ident = token::str_to_ident("url_fmtd");
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
						//TODO: format!(url, parts[0], ..., parts[n])
						init: None,
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