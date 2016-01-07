use std::intrinsics::type_name;
use std::str;
use syntax::ast::*;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;
use chomp::*;

fn parse_path_segment(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		//Trim leading :
		let _ = take_while(|c| c == b':');
		let seg = take_while1(|c| c != b':');

		ret str::from_utf8(seg).unwrap().to_string()
	}
}

pub fn parse_path(path: &str) -> Vec<String> {
	parse_only(|i| many(i, |i| parse_path_segment(i)), path.as_bytes()).unwrap()
}

/// Generate a type with a specified name
pub fn build_ty(name: &str) -> Ty {
	let segments = parse_path(name).iter().map(|seg| PathSegment {
		identifier: token::str_to_ident(&seg[..]),
		parameters: PathParameters::none()
	}).collect();

	Ty {
		id: DUMMY_NODE_ID,
		node: Ty_::TyPath(None, Path {
			span: DUMMY_SP,
			global: false,
			segments: segments
		}),
		span: DUMMY_SP
	}
}

/// Generate a potentially mutable type with a specified name
pub fn build_ty_mut(name: &str, mutbl: Mutability) -> MutTy {
	MutTy {
		ty: P(build_ty(name)),
		mutbl: mutbl
	}
}

/// Generate a type pointer with a specified name
pub fn build_ty_ptr(name: &str, mutbl: Mutability, lifetime: Option<Lifetime>) -> Ty {
	Ty {
		id: DUMMY_NODE_ID,
		node: Ty_::TyRptr(
			lifetime,
			build_ty_mut(name, mutbl)
		),
		span: DUMMY_SP
	}
}

/// Generate a type
pub fn ty<T>() -> Ty {
	build_ty(name_of::<T>())
}

/// Generate a potentially mutable type
pub fn ty_mut<T>(mutbl: Mutability) -> MutTy {
	MutTy {
		ty: P(ty::<T>()),
		mutbl: mutbl
	}
}

/// Generate a type pointer
pub fn ty_ptr<T>(mutbl: Mutability, lifetime: Option<Lifetime>) -> Ty {
	Ty {
		id: DUMMY_NODE_ID,
		node: Ty_::TyRptr(
			lifetime,
			ty_mut::<T>(mutbl)
		),
		span: DUMMY_SP
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