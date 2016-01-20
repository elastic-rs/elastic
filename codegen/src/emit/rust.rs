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