use std::intrinsics::type_name;
use syntax::ast::{ 
	Ident, 
	FnDecl, 
	Generics, 
	Block, 
	BlockCheckMode,
	Unsafety, 
	Constness, 
	FunctionRetTy, 
	Pat, 
	PatIdent, 
	BindingMode, 
	Ty, 
	Ty_, 
	MutTy, 
	Arg,
	Path,
	PathParameters, 
	PathSegment, 
	DUMMY_NODE_ID 
};
use syntax::parse::token;
use syntax::print::pprust;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::ptr::P;

/// Rexport of the libsyntax Mutability enum
pub use syntax::ast::Mutability;

/// A representation of a Rust fn
pub struct Fn {
	/// The name of the fn
	pub identifier: Ident,
	/// The fn header with arguments and return type
	pub decl: FnDecl,
	/// The lifetimes and generic params for the fn
	pub generics: Generics,
	/// The body of the fn
	pub body: Block,
	/// Whether or not the fn is unsafe
	pub unsafety: Unsafety,
	/// Whether or not the fn is constant
	pub constness: Constness
}

impl ToString for Fn {
	/// Outputs the fn declaration and body as Rust source
	fn to_string(&self) -> String {
		let decl = pprust::fun_to_string(
			&self.decl, 
			self.unsafety, 
			self.constness, 
			self.identifier, 
			None, 
			&self.generics
		);

		let body = pprust::block_to_string(&self.body);

		decl + &body
	}
}

/// Generates function signatures for the Elasticsearch API
pub struct RustApiFnBldr;
impl RustApiFnBldr {
	/// Create a new builder
	pub fn new() -> RustApiFnBldr {
		RustApiFnBldr
	}

	/// Generate a function
	pub fn gen_fn(&self, name: &str, inputs: Vec<Arg>) -> Fn {
		Fn {
			identifier: token::str_to_ident(name),
			decl: FnDecl {
				inputs: inputs,
				output: FunctionRetTy::DefaultReturn(DUMMY_SP),
				variadic: false
			},
			generics: Generics::default(),
			body: Block {
				stmts: Vec::new(),
				expr: None,
				id: DUMMY_NODE_ID,
				rules: BlockCheckMode::DefaultBlock,
				span: DUMMY_SP
			},
			unsafety: Unsafety::Normal,
			constness: Constness::NotConst
		}
	}

	/// Generate a function arg with a type
	pub fn gen_arg(&self, name: &str, ty: Ty) -> Arg {
		Arg {
			ty: P(ty),
			pat: P(Pat {
				id: DUMMY_NODE_ID,
				node: PatIdent(
					BindingMode::ByValue(Mutability::MutImmutable),
					Spanned {
						span: DUMMY_SP,
						node: token::str_to_ident(name)
					},
					None
					),
				span: DUMMY_SP
			}),
			id: DUMMY_NODE_ID
		}
	}

	/// Generate a type with a specified name
	pub fn gen_ty_as(&self, name: &str) -> Ty {
		Ty {
			id: DUMMY_NODE_ID,
			node: Ty_::TyPath(None, Path {
				span: DUMMY_SP,
				global: false,
				segments: vec![
				PathSegment {
					identifier: token::str_to_ident(name),
					parameters: PathParameters::none()
				}]
			}),
			span: DUMMY_SP
		}
	}

	/// Generate a type
	pub fn gen_ty<T>(&self) -> Ty {
		self.gen_ty_as(name_of::<T>())
	}

	/// Generate a potentially mutable type with a specified name
	pub fn gen_ty_mut_as(&self, name: &str, mutbl: Mutability) -> MutTy {
		MutTy {
			ty: P(self.gen_ty_as(name)),
			mutbl: mutbl
		}
	}

	/// Generate a potentially mutable type
	pub fn gen_ty_mut<T>(&self, mutbl: Mutability) -> MutTy {
		MutTy {
			ty: P(self.gen_ty::<T>()),
			mutbl: mutbl
		}
	}

	/// Generate a type pointer with a specified name
	pub fn gen_ty_ptr_as<T>(&self, name: &str, mutbl: Mutability) -> Ty {
		Ty {
			id: DUMMY_NODE_ID,
			node: Ty_::TyRptr(
				None,
				self.gen_ty_mut_as(name, mutbl)
			),
			span: DUMMY_SP
		}
	}

	/// Generate a type pointer
	pub fn gen_ty_ptr<T>(&self, mutbl: Mutability) -> Ty {
		Ty {
			id: DUMMY_NODE_ID,
			node: Ty_::TyRptr(
				None,
				self.gen_ty_mut::<T>(mutbl)
			),
			span: DUMMY_SP
		}
	}
}

/// Get the full-path name of a type
pub fn name_of<'a, T>() -> &'a str {
    let t =
        unsafe {
            type_name::<T>()
        };
    t
}

/// Get the full-path name of a type inferred from the argument
pub fn type_of<T>(_: &T) -> &str {
    name_of::<T>()
}