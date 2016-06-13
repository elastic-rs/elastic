//! Rust Emitter Helpers
//!
//! Contains implementations of `emit` for the `libsyntax` crate and other `gen::rust` modules.

use std::marker::PhantomData;
use std::io::Write;
use syntax::ext::base::ExtCtxt;
use syntax::ext::quote::rt::ToTokens;
use syntax::print::pprust;
use ::gen::rust;
use super::{ Emit, Emitter, EmitResult, EmitError };

/// Emit a Rust AST
impl <'a, T> Emit<ExtCtxt<'a>, EmitError> for T where T: ToTokens {
	fn emit(&self, cx: &ExtCtxt) -> EmitResult {
		Ok(pprust::tts_to_string(&self.to_tokens(cx)[..]))
	}
}

/// Emit a Rust Function
impl <'a> Emit<ExtCtxt<'a>, EmitError> for rust::Fn {
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
/// use syntax::ext::base::{ ExtCtxt, DummyMacroLoader };
/// use syntax::ext::expand::ExpansionConfig;
/// use syntax::print::pprust;
/// use elastic_codegen::emit::*;
/// use elastic_codegen::emit::rust::*;
///
/// //Create an ExtCtxt to use in the emitter
/// let sess = ParseSess::new();
/// let mut attrs: Vec<GatedCfgAttr> = Vec::new();
/// let mut loader = DummyMacroLoader;
/// let mut cx = ExtCtxt::new(
/// 	&sess,
/// 	Vec::new(),
/// 	ExpansionConfig::default("".to_string()),
/// 	&mut attrs,
/// 	&mut loader
/// );
///
/// let emitter = RustEmitter::new();
///
/// let mut buf: Vec<u8> = Vec::new();
///
/// //Emit a token
/// let token = token::str_to_ident("some_ident");
/// let _ = emitter.emit(&token, &cx, &mut buf).unwrap();
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
/// use syntax::ext::base::{ ExtCtxt, DummyMacroLoader };
/// use syntax::ext::expand::ExpansionConfig;
/// use syntax::print::pprust;
/// use elastic_codegen::emit::*;
/// use elastic_codegen::emit::rust::*;
/// use elastic_codegen::gen::rust::*;
///
/// //Create an ExtCtxt to use in the emitter
/// let sess = ParseSess::new();
/// let mut attrs: Vec<GatedCfgAttr> = Vec::new();
/// let mut loader = DummyMacroLoader;
/// let mut cx = ExtCtxt::new(
/// 	&sess,
/// 	Vec::new(),
/// 	ExpansionConfig::default("".to_string()),
/// 	&mut attrs,
/// 	&mut loader
/// );
///
/// let emitter = RustEmitter::new();
///
/// let mut buf: Vec<u8> = Vec::new();
///
/// //Emit a function signature
/// let lifetime = lifetime("'a");
/// let mut fun = build_fn("my_fun", vec![
/// 	arg_ptr::<i32>("arg1", Mutability::Mutable, Some(lifetime)),
/// 	build_arg("arg2", build_ty_ptr("str", Mutability::Immutable, Some(lifetime)))
/// ]);
///
/// let _ = emitter.emit(&fun, &cx, &mut buf).unwrap();
/// # }
/// ```
pub struct RustEmitter<'a> {
	phantom: PhantomData<&'a ()>
}

impl <'a> RustEmitter<'a> {
	/// Create a new emitter with the provided `ExtCtxt`.
	pub fn new() -> RustEmitter<'a> {
		RustEmitter {
			phantom: PhantomData
		}
	}
}

impl <'a> Emitter for RustEmitter<'a> {
	type Ctxt = ExtCtxt<'a>;
	type Error = EmitError;

	fn emit<Emittable, EmError, W>(&self, e: &Emittable, cx: &ExtCtxt<'a>, writer: &mut W) -> Result<(), Self::Error> where
		Emittable: Emit<Self::Ctxt, EmError>,
		EmError: Into<EmitError>,
		W: Write {
			emit!(cx, e, writer)
	}

	/// Emit a string
	fn emit_str<W>(&self, e: &str, writer: &mut W) -> Result<(), Self::Error> where W: Write {
		emit_str!(e, writer)
	}
}
