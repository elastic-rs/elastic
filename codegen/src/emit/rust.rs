//! Rust Emitter Helpers
//! 
//! Contains implementations of `emit` for the `libsyntax` crate and other `gen::rust` modules.

use syntax::ext::base::ExtCtxt;
use syntax::ext::quote::rt::ToTokens;
use syntax::print::pprust;
use ::gen::rust;
use super::{ Emit, Emitter, EmitResult, EmitError };

/// Emit a Rust AST
impl <'a, T> Emit<&'a ExtCtxt<'a>, EmitError> for T where T: ToTokens {
	fn emit(&self, cx: &ExtCtxt) -> EmitResult {
		Ok(pprust::tts_to_string(&self.to_tokens(cx)[..]))
	}
}

/// Emit a Rust Function
impl <'a> Emit<&'a ExtCtxt<'a>, EmitError> for rust::Fn {
	fn emit(&self, _: &ExtCtxt) -> EmitResult {
		Ok(self.to_string())
	}
}

/// Emitter for Rust AST
/// 
/// # Examples
/// 
/// Emit a Rust AST token:
/// 
/// ```
/// # #![feature(rustc_private)]
/// # extern crate syntax;
/// # extern crate elastic_codegen;
/// # fn main() {
/// use syntax::parse::token;
/// use syntax::parse::ParseSess;
/// use syntax::feature_gate::GatedCfgAttr;
/// use syntax::ext::base::ExtCtxt;
/// use syntax::ext::expand::ExpansionConfig;
/// use syntax::print::pprust;
/// use elastic_codegen::emit::*;
/// use elastic_codegen::emit::rust::*;
/// 
/// //Create an ExtCtxt to use in the emitter
/// let sess = ParseSess::new();
/// let mut attrs: Vec<GatedCfgAttr> = Vec::new();
/// let cx = ExtCtxt::new(
/// 	&sess, 
/// 	Vec::new(), 
/// 	ExpansionConfig::default("".to_string()), 
/// 	&mut attrs
/// );
/// 
/// //Create an emitter
/// let emitter = RustEmitter::new(cx);
/// let mut buf: Vec<u8> = Vec::new();
/// 
/// //Emit a token
/// let token = token::str_to_ident("some_ident");
/// let _ = emitter.emit(&token, &mut buf).unwrap();
/// # }
/// ```
/// 
/// Emit a Rust Function:
/// 
/// ```
/// # #![feature(rustc_private)]
/// # extern crate syntax;
/// # extern crate elastic_codegen;
/// # fn main() {
/// use syntax::ast::*;
/// use syntax::parse::ParseSess;
/// use syntax::feature_gate::GatedCfgAttr;
/// use syntax::ext::base::ExtCtxt;
/// use syntax::ext::expand::ExpansionConfig;
/// use syntax::print::pprust;
/// use elastic_codegen::emit::*;
/// use elastic_codegen::emit::rust::*;
/// use elastic_codegen::gen::rust::*;
/// 
/// //Create an ExtCtxt to use in the emitter
/// let sess = ParseSess::new();
/// let mut attrs: Vec<GatedCfgAttr> = Vec::new();
/// let cx = ExtCtxt::new(
/// 	&sess, 
/// 	Vec::new(), 
/// 	ExpansionConfig::default("".to_string()), 
/// 	&mut attrs
/// );
/// 
/// //Create an emitter
/// let emitter = RustEmitter::new(cx);
/// let mut buf: Vec<u8> = Vec::new();
/// 
/// //Emit a function signature
/// let lifetime = lifetime("'a");
/// let mut fun = build_fn("my_fun", vec![
/// 	arg_ptr::<i32>("arg1", Mutability::MutMutable, Some(lifetime)),
/// 	build_arg("arg2", build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
/// ]);
/// 
/// let _ = emitter.emit(&fun, &mut buf).unwrap();
/// # }
/// ```
pub struct RustEmitter<'a> {
	cx: ExtCtxt<'a>
}

impl <'a> RustEmitter<'a> {
	/// Create a new emitter with the provided `ExtCtxt`.
	pub fn new(cx: ExtCtxt<'a>) -> RustEmitter<'a> {
		RustEmitter {
			cx: cx
		}
	}
}

impl <'a> Emitter<'a> for RustEmitter<'a> {
	type Ctxt = ExtCtxt<'a>;
	
	fn get_cx(&self) -> &'a Self::Ctxt {
		&self.cx
	}
}