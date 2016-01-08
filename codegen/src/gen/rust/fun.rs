use syntax::ast::*;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::{ ty, ty_ptr };

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

impl Fn {
	pub fn add_lifetime<'a>(&'a mut self, lifetime: Lifetime) -> &'a mut Fn {
		self.generics.lifetimes.push(LifetimeDef {
			lifetime: lifetime,
			bounds: Vec::new()
		});

		self
	}
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

/// Generate a function
pub fn build_fn(name: &str, inputs: Vec<Arg>) -> Fn {
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
pub fn build_arg(name: &str, ty: Ty) -> Arg {
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

/// Generate a lifetime parameter
pub fn lifetime(name: &str) -> Lifetime {
	Lifetime {
		id: DUMMY_NODE_ID,
		span: DUMMY_SP,
		name: intern(name)
	}
}

/// Generate an arg with the specified type
pub fn arg<T>(name: &str) -> Arg {
	build_arg(name, ty::<T>())
}

/// Generate a potentially mutable arg with the specified type
pub fn arg_ptr<T>(name: &str, mutbl: Mutability, lifetime: Option<Lifetime>) -> Arg {
	build_arg(name, ty_ptr::<T>(mutbl, lifetime))
}