use std::intrinsics::type_name;
use syntax::ast;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::{ Span, Spanned, DUMMY_SP };
use syntax::ptr::P;

pub struct Fn {
	pub identifier: Ident,
	pub decl: FnDecl,
	pub generics: Generics,
	pub unsafety: Unsafety,
	pub constness: Constness
}

/// Generates function signatures for the Elasticsearch API
pub struct RustApiFnGen;
impl RustApiFnGen {
	//TODO: Fill in this method properly
	pub fn gen_fn(name: &str) -> Fn {
		//function
		let mut fun = FnDecl {
			inputs: vec![
			//arg1: &mut i32
			Arg {
				//&mut i32
				ty: P(RustApiFnGen::gen_ty_ptr::<i32>(Mutability::MutMutable)),
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

		Fn {
			identifier: token::str_to_ident(name),
			decl: fun, 
			generics: Generics::default(),
			unsafety: Unsafety::Normal, 
			constness: Constness::NotConst
		}
	}

	pub fn gen_ty<T>() -> Ty {
		Ty {
			id: DUMMY_NODE_ID,
			node: Ty_::TyPath(None, Path {
				span: DUMMY_SP,
				global: false,
				segments: vec![ 
				PathSegment { 
					identifier: token::str_to_ident(name_of::<T>()), 
					parameters: PathParameters::none()
				}]
			}),
			span: DUMMY_SP,
		}
	}

	pub fn gen_ty_from<T>(_: &T) -> Ty {
		RustApiFnGen::gen_ty::<T>()
	}

	pub fn gen_ty_mut<T>(mutbl: Mutability) -> MutTy {
		MutTy {
			ty: P(RustApiFnGen::gen_ty::<T>()),
			mutbl: mutbl
		}
	}

	pub fn gen_ty_mut_from<T>(mutbl: Mutability, _: &T) -> MutTy {
		RustApiFnGen::gen_ty_mut::<T>(mutbl)
	}

	pub fn gen_ty_ptr<T>(mutbl: Mutability) -> Ty {
		Ty {
			id: DUMMY_NODE_ID,
			node: Ty_::TyRptr(
				None, 
				RustApiFnGen::gen_ty_mut::<T>(mutbl)
			),
			span: DUMMY_SP
		}
	}

	pub fn gen_ty_ptr_from<T>(mutbl: Mutability, _: &T) -> Ty {
		RustApiFnGen::gen_ty_ptr::<T>(mutbl)
	}
}

pub fn name_of<'a, T>() -> &'a str {
    let t =
        unsafe {
            type_name::<T>()
        };
    t
}

pub fn type_of<T>(_: &T) -> &str {
    name_of::<T>()
}