extern crate aster;
extern crate elastic_codegen;

use self::aster::AstBuilder;
use self::elastic_codegen::gen::Gen;
use self::elastic_codegen::ast::*;

/// Generates Hyper implementation as a function body for the Elasticsearch API
pub struct HyperFnBodyGen {
	bldr: AstBuilder
}
impl HyperFnBodyGen {
	pub fn new() -> HyperFnBodyGen {
		HyperFnBodyGen {
			bldr: AstBuilder::new()
		}
	}
	pub fn from(bldr: AstBuilder) -> HyperFnBodyGen {
		HyperFnBodyGen {
			bldr: bldr
		}
	}
}

impl <'a> Gen<'a, &'a mut AstBuilder> for HyperFnBodyGen {
	fn gen(&'a mut self, ast: SyntaxTree) -> Result<&'a mut AstBuilder, &'static str> {
		//TODO: Examine fn params, build GET/POST/PUT with appropriate url
		Ok(&mut self.bldr)
	}
}