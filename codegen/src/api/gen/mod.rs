use syntax::ast;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Span, Spanned, DUMMY_SP };
use syntax::ptr::P;

/// Generates function signatures for the Elasticsearch API
pub struct RustApiFnGen;
impl RustApiFnGen {
	//TODO: Fill in this method properly
	pub fn gen(name: &str) -> (FnDecl, Unsafety, Constness, Ident, Generics) {
		//function
		let mut fun = FnDecl {
			inputs: vec![
			//arg1: &mut i32
			Arg {
				//&mut i32
				ty: P(Ty {
					id: DUMMY_NODE_ID,
					//&mut
					node: Ty_::TyRptr(
						None, 
						MutTy { 
							//i32
							ty: P(Ty {
								id: DUMMY_NODE_ID,
								node: Ty_::TyPath(None, Path {
									span: DUMMY_SP,
									global: false,
									segments: vec![ 
									PathSegment { 
										identifier: token::str_to_ident("i32"), 
										parameters: PathParameters::none()
									} 
									]
								}),
								span: DUMMY_SP,
							}),
							//mut
							mutbl: Mutability::MutMutable
						}
						),
					span: DUMMY_SP,
				}),
				//arg1
				pat: P(Pat {
					id: DUMMY_NODE_ID,
					node: PatIdent(
						BindingMode::ByValue(Mutability::MutImmutable), 
						Spanned { 
							span: DUMMY_SP, 
							node: token::str_to_ident("arg1") 
						}, 
						None
						),
					span: DUMMY_SP
				}),
				id: DUMMY_NODE_ID
			}
			],
			//No return
			output: FunctionRetTy::DefaultReturn(DUMMY_SP),
			variadic: false
		};

		let mut generics = Generics::default();

		(fun, Unsafety::Normal, Constness::NotConst, token::str_to_ident(name), generics)
	}
}