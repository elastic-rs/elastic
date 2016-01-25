//! Elasticsearch Emitter Default Implementations
//!
//! The base implementations have been split up to try and prevent collisions with user implementations.
//! This module also contains a default context-free `Emitter` that's suitable for most scenarios.

pub mod num;
pub mod string;
pub mod boolean;

use std::marker::PhantomData;
use std::io::Write;
use super::{ Emit, EmitError, Emitter };

/// Default context-free Emitter
/// 
/// This emitter will emit items that implement `Emit<(), _>`
/// 
/// # Examples
/// 
/// ```
/// use std::io::Write;
/// use elastic_codegen::emit::*;
/// use elastic_codegen::emit::default::*;
/// 
/// let mut buf: Vec<u8> = Vec::new();
/// let emitter: CtxtFreeEmitter = CtxtFreeEmitter::new();
/// 
/// let item = "my emittable item";
/// let _ = emitter.emit(&item, &mut buf).unwrap();
/// ```
pub struct CtxtFreeEmitter<E = EmitError> where E: From<EmitError> {
	phantom: PhantomData<E>
}

impl <E> CtxtFreeEmitter<E> where E: From<EmitError> {
	/// Creates a new emitter.
	/// 
	/// # Examples
	/// 
	/// Create a default `CtxtFreeEmitter`:
	/// 
	/// ```
	/// use elastic_codegen::emit::*;
	/// use elastic_codegen::emit::default::*;
	/// 
	/// let emitter: CtxtFreeEmitter = CtxtFreeEmitter::new();
	/// ```
	/// 
	/// Create a `CtxtFreeEmitter` with the specified `Error` type:
	/// 
	/// ```
	/// use elastic_codegen::emit::*;
	/// use elastic_codegen::emit::default::*;
	/// 
	/// let emitter = CtxtFreeEmitter::<EmitError>::new();
	/// ```
	pub fn new() -> CtxtFreeEmitter<E> {
		CtxtFreeEmitter::<E> {
			phantom: PhantomData
		}
	}
}

impl <'a, E> Emitter<'a> for CtxtFreeEmitter<E> where E: From<EmitError> {
	type Ctxt = ();
	type CtxtBrw = ();
	type Error = E;

	fn get_cx(&self) {

	}

	fn emit<Emittable, EmError, W>(&self, e: &'a Emittable, writer: &'a mut W) -> Result<(), Self::Error> where 
		Emittable: Emit<Self::CtxtBrw, EmError>, 
		EmError: Into<EmitError>, 
		W: Write {
			let cx = self.get_cx();
			emit!(cx, e, writer)
	}

	fn emit_str<W>(&self, e: &str, writer: &mut W) -> Result<(), Self::Error> where W: Write {
		emit_str!(e, writer)
	}
}