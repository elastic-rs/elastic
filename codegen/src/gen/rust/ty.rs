use std::intrinsics::type_name;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;
use super::parse::parse_path;

/// Generate a type with a specified name.
pub fn build_ty(name: &str) -> Ty {
	Ty {
		id: DUMMY_NODE_ID,
		node: TyKind::Path(None, build_path(name)),
		span: DUMMY_SP
	}
}

/// Build a path from a string.
pub fn build_path(name: &str) -> Path {
	let segments = parse_path(name).iter().map(|seg| PathSegment {
		identifier: token::str_to_ident(&seg[..]),
		parameters: PathParameters::none()
	}).collect();

	Path {
		span: DUMMY_SP,
		global: false,
		segments: segments
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

/// Build a type param.
pub fn build_ty_param(name: &str, trait_refs: Vec<&str>) -> TyParam {
	let bounds = trait_refs.iter().map(|t|
		build_ty_param_bound(t)
	);

	TyParam {
		ident: token::str_to_ident(name),
		id: DUMMY_NODE_ID,
		bounds: P::from_vec(bounds.collect()),
		default: None,
		span: DUMMY_SP
	}
}

/// Build a type param bound.
pub fn build_ty_param_bound(trait_ref: &str) -> TyParamBound {
	TyParamBound::TraitTyParamBound(
		PolyTraitRef {
			bound_lifetimes: Vec::new(),
			trait_ref: TraitRef {
				path: build_path(trait_ref),
				ref_id: DUMMY_NODE_ID
			},
			span: DUMMY_SP
		},
		TraitBoundModifier::None
	)
}

/// Get the full-path name of a type.
pub fn type_of<'a, T>() -> &'a str {
	unsafe {
		type_name::<T>()
	}
}

fn _type_of<T>(opts: TyPathOpts) -> String {
	match opts {
		TyPathOpts::Full => type_of::<T>().to_owned(),
		TyPathOpts::NameOnly => {
			let mut parts = parse_path(type_of::<T>());
			parts.pop().unwrap_or_default()
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
