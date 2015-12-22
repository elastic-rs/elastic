use aster;
use ::ast::*;

pub trait Gen<T> {
	fn gen(ast: SyntaxTree) -> Result<T, &'static str>;
}

pub struct RustApiGen;

impl Gen<String> for RustApiGen {
	fn gen(ast: SyntaxTree) -> Result<String, &'static str> {
		let bldr = aster::AstBuilder::new();

		panic!("implement");
	}
}