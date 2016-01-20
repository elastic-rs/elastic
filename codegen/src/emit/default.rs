//! Elasticsearch Emitter Default Implementations
//! 
//! Sweeping defaults for emitting rust structs with a context-free emitter.
//! 
//! # Examples
//! 
//! Emit primitives that implement `ToString`:
//! 
//! ```
//! use std::io::Write;
//! use elastic_codegen::emit::*;
//! use elastic_codegen::emit::default::*;
//! 
//! let mut buf: Vec<u8> = Vec::new();
//! let emitter: CtxtFreeEmitter = CtxtFreeEmitter::new();
//! 
//! let item_i32 = 42;
//! let _ = emitter.emit(&item_i32, &mut buf).unwrap();
//! 
//! let item_str = "my item";
//! let _ = emitter.emit(&item_str, &mut buf).unwrap();
//! ```

use super::{ Emit, EmitError };

impl <T> Emit<(), EmitError> for T where T: ToString {
	fn emit(&self, _: ()) -> Result<String, EmitError> {
		Ok(self.to_string())
	}
}