use super::syntax::ast;
use super::syntax::ast::*;
use super::syntax::parse::token;
use super::syntax::codemap::{ Span, Spanned, DUMMY_SP };
use super::syntax::ptr::P;

/// Generates function signatures for the Elasticsearch API
pub struct HyperApiFnGen;
impl HyperApiFnGen {
	pub fn gen(name: &str) -> (FnDecl, Unsafety, Constness, Ident, Generics) {
		panic!("implement")
	}
}