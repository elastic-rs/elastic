use std::intrinsics::type_name;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;
use super::parse_path;

/// Generate a type with a specified name.
pub fn build_ty(name: &str) -> Ty {
	let segments = parse_path(name).iter().map(|seg| PathSegment {
		identifier: token::str_to_ident(&seg[..]),
		parameters: PathParameters::none()
	}).collect();

	Ty {
		id: DUMMY_NODE_ID,
		node: TyKind::Path(None, Path {
			span: DUMMY_SP,
			global: false,
			segments: segments
		}),
		span: DUMMY_SP
	}
}

/// Generate a potentially mutable type with a specified name.
pub fn build_ty_mut(name: &str, mutbl: Mutability) -> MutTy {
	MutTy {
		ty: P(build_ty(name)),
		mutbl: mutbl
	}
}

/// Generate a type pointer with a specified name.
pub fn build_ty_ptr(name: &str, mutbl: Mutability, lifetime: Option<Lifetime>) -> Ty {
	Ty {
		id: DUMMY_NODE_ID,
		node: TyKind::Rptr(
			lifetime,
			build_ty_mut(name, mutbl)
		),
		span: DUMMY_SP
	}
}

/// Generate a type.
pub fn ty<T>(opts: TyPathOpts) -> Ty {
	build_ty(&_type_of::<T>(opts)[..])
}

/// Generate a potentially mutable type.
pub fn ty_mut<T>(mutbl: Mutability, opts: TyPathOpts) -> MutTy {
	build_ty_mut(&_type_of::<T>(opts)[..], mutbl)
}

/// Generate a type pointer.
pub fn ty_ptr<T>(mutbl: Mutability, lifetime: Option<Lifetime>, opts: TyPathOpts) -> Ty {
	build_ty_ptr(&_type_of::<T>(opts)[..], mutbl, lifetime)
}

/// Get the full-path name of a type.
pub fn type_of<'a, T>() -> &'a str {
    let t =
        unsafe {
            type_name::<T>()
        };
    t
}

fn _type_of<T>(opts: TyPathOpts) -> String {
	match opts {
		TyPathOpts::Full => type_of::<T>().to_string(),
		TyPathOpts::NameOnly => {
			let mut parts = parse_path(type_of::<T>());
			parts.pop().unwrap_or(String::new())
		}
	}
}

/// Get the full-path name of a type inferred from the argument.
pub fn infer_type_of<T>(_: &T) -> &str {
    type_of::<T>()
}

/// The kind of path to use in the type Ident.
/// 
/// The default value is `NameOnly`.
pub enum TyPathOpts {
	/// Use just the name of the type.
	NameOnly,
	/// Use the full Rust path, including crates and modules.
	Full
}

impl Default for TyPathOpts {
	fn default() -> TyPathOpts {
		TyPathOpts::NameOnly
	}
}