use super::ast::*;
use aster::AstBuilder;

/// Generic codegen trait
pub trait Gen<'a, T> {
	fn gen(&'a mut self, ast: Endpoint) -> Result<T, &'static str>;
}

/// Generates function signatures for the Elasticsearch API
pub struct RustFnSigGen {
	bldr: AstBuilder
}
impl RustFnSigGen {
	pub fn new() -> RustFnSigGen {
		RustFnSigGen {
			bldr: AstBuilder::new()
		}
	}
	pub fn from(bldr: AstBuilder) -> RustFnSigGen {
		RustFnSigGen {
			bldr: bldr
		}
	}
}

impl <'a> Gen<'a, &'a mut AstBuilder> for RustFnSigGen {
	fn gen(&'a mut self, ast: Endpoint) -> Result<&'a mut AstBuilder, &'static str> {
		//TODO: Determine fn params, whether required or optional. Single url per HTTP Verb
		Ok(&mut self.bldr)
	}
}