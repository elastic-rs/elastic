use std::ops::Deref;
use syntax::ast::*;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::codemap::{ Spanned, DUMMY_SP };
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::{ ty, ty_ptr, TyPathOpts };

/// A representation of a Rust fn.
pub struct Fn {
	/// The name of the fn.
	pub identifier: Ident,
	/// The fn header with arguments and return type.
	pub decl: FnDecl,
	/// The lifetimes and generic params for the fn.
	pub generics: Generics,
	/// The body of the fn.
	pub body: Block,
	/// Whether or not the fn is unsafe.
	pub unsafety: Unsafety,
	/// Whether or not the fn is constant.
	pub constness: Constness
}

impl Fn {
	//TODO: add_arg and add_args

	/// Append a lifetime to the function generics.
	pub fn add_lifetime<'a>(&'a mut self, lifetime: Lifetime) -> &'a mut Fn {
		self.generics.lifetimes.push(LifetimeDef {
			lifetime: lifetime,
			bounds: Vec::new()
		});

		self
	}

	/// Set the return type of the function.
	pub fn set_return<'a, T>(&'a mut self) -> &'a mut Fn {
		self.decl.output = FunctionRetTy::Ty(P(ty::<T>(TyPathOpts::default())));

		self
	}

	/// Append a statement to the function body.
	pub fn add_body_stmt<'a>(&'a mut self, stmt: Stmt) -> &'a mut Fn {
		self.body.stmts.push(stmt);

		self
	}

	/// Append a collection of statements to the function body.
	pub fn add_body_stmts<'a>(&'a mut self, stmts: Vec<Stmt>) -> &'a mut Fn {
		let mut _stmts = stmts.clone();
		self.body.stmts.append(&mut _stmts);

		self
	}

	/// Append the body to existing statements.
	/// 
	/// This will update the return expression if the function declaration has a return type set.
	pub fn add_body_block<'a>(&'a mut self, body: P<Block>) -> &'a mut Fn {
		let _body = body.deref();

		//Append the body statements
		let mut body_stmts = _body.stmts.clone();
		self.body.stmts.append(&mut body_stmts);

		//Set the return type if the function takes one
		match self.decl.output {
			FunctionRetTy::Ty(_) => self.body.expr = _body.expr.clone(),
			_ => ()
		}
		
		self
	}

	/// Set the function body.
	pub fn set_body<'a>(&'a mut self, body: P<Block>) -> &'a mut Fn {
		self.body = body.deref().clone();

		self
	}
}

impl ToString for Fn {
	/// Outputs the fn declaration and body as Rust source.
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

/// Generate a function.
/// 
/// # Examples
/// 
/// ```
/// # #![feature(rustc_private)]
/// # extern crate syntax;
/// # extern crate elastic_codegen;
/// # fn main() {
/// # struct MyStruct;
/// use syntax::ast::*;
/// use elastic_codegen::gen::rust::*;
/// 
/// //Define a lifetime 'a
/// let lifetime = lifetime("'a");
/// 
/// //Build a function signature
/// let mut fun = build_fn("my_fun", vec![
/// 	arg::<MyStruct>("arg1"),
/// 	arg_ptr::<i32>("arg2", Mutability::Mutable, Some(lifetime)),
/// 	build_arg("arg3", build_ty_ptr("str", Mutability::Immutable, Some(lifetime)))
/// ]);
/// fun.add_lifetime(lifetime);
/// 
/// //Print the results: 'fn my_fun<'a>(arg1: MyStruct, arg2: &'a mut i32, arg3: &'a str){ }'
/// println!("{}", fun.to_string());
/// # }
/// ```
pub fn build_fn(name: &str, inputs: Vec<Arg>) -> Fn {
	Fn {
		identifier: token::str_to_ident(name),
		decl: FnDecl {
			inputs: inputs,
			output: FunctionRetTy::Default(DUMMY_SP),
			variadic: false
		},
		generics: Generics::default(),
		body: Block {
			stmts: Vec::new(),
			expr: None,
			id: DUMMY_NODE_ID,
			rules: BlockCheckMode::Default,
			span: DUMMY_SP
		},
		unsafety: Unsafety::Normal,
		constness: Constness::NotConst
	}
}

/// Generate a function arg with a type.
pub fn build_arg(name: &str, ty: Ty) -> Arg {
	build_arg_ident(token::str_to_ident(name), ty)
}

/// Generate a function arg with a type and existing ident.
pub fn build_arg_ident(name: Ident, ty: Ty) -> Arg {
	Arg {
		ty: P(ty),
		pat: P(Pat {
			id: DUMMY_NODE_ID,
			node: PatIdent(
				BindingMode::ByValue(Mutability::Immutable),
				Spanned {
					span: DUMMY_SP,
					node: name
				},
				None
				),
			span: DUMMY_SP
		}),
		id: DUMMY_NODE_ID
	}
}

/// Generate a lifetime parameter.
pub fn lifetime(name: &str) -> Lifetime {
	Lifetime {
		id: DUMMY_NODE_ID,
		span: DUMMY_SP,
		name: intern(name)
	}
}

/// Generate an arg with the specified type.
pub fn arg<T>(name: &str) -> Arg {
	build_arg(name, ty::<T>(TyPathOpts::default()))
}

/// Generate an arg using the specified type and existing ident
pub fn arg_ident<T>(name: Ident) -> Arg {
	build_arg_ident(name, ty::<T>(TyPathOpts::default()))
}

/// Generate a potentially mutable arg with the specified type.
pub fn arg_ptr<T>(name: &str, mutbl: Mutability, lifetime: Option<Lifetime>) -> Arg {
	build_arg(name, ty_ptr::<T>(mutbl, lifetime, TyPathOpts::default()))
}

/// Generate a potentially mutable arg with the specified type.
pub fn arg_ptr_ident<T>(name: Ident, mutbl: Mutability, lifetime: Option<Lifetime>) -> Arg {
	build_arg_ident(name, ty_ptr::<T>(mutbl, lifetime, TyPathOpts::default()))
}
