//! Elasticsearch Emitter
//! 
//! Utilities for emitting generated code to some output.

use std::io::Write;
use std::error;
use std::fmt;
use std::io::Error as IoError;

#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! emit {
		($cx:ident, $emittable:ident, $writer:ident) => {
			{
				let emitted = try!(
					$emittable.emit($cx).map_err(|e| e.into())
				);
				
				$writer.write_all(&emitted.into_bytes()[..]).map_err(|e| {
					let err: EmitError = e.into();
					err.into()
				})
			}
		}
	}

	#[macro_export]
	macro_rules! emit_str {
		($emittable:ident, $writer:ident) => {
			{
				$writer.write_all($emittable.as_bytes()).map_err(|e| {
					let err: EmitError = e.into();
					err.into()
				})
			}
		}
	}
}

pub mod default;
pub mod rust;

/// An emittable codegen item.
/// 
/// Takes in a context struct. This is necessary for rust `TokenTrees`, but may not be required in other cases.
/// 
/// # Examples
/// 
/// Implement `Emit` with no context:
/// 
/// ```
/// use elastic_codegen::emit::*;
/// 
/// struct MyEmittable;
/// 
/// impl Emit<(), EmitError> for MyEmittable {
/// 	fn emit(&self, _: ()) -> Result<String, EmitError> {
/// 		Ok("some result".to_string())
/// 	}
/// }
/// ```
pub trait Emit<T, E> where E: Into<EmitError> {
	/// Emit to a string
	fn emit(&self, cx: T) -> Result<String, E>;
}

/// Emitter for codegen items.
/// 
/// The `Emitter` takes compatible `Emit` structs and writes them to a destination.
pub trait Emitter<'a> {
	/// A context struct that's threaded through calls to `Emit::emit`.
	type Ctxt: 'a;
	/// The context passed to the implementors of `Emit`.
	type CtxtBrw: 'a = &'a Self::Ctxt;
	/// An error returned by `emit()`.
	type Error: From<EmitError> = EmitError;
	
	/// Gets the context struct.
	fn get_cx(&'a self) -> Self::CtxtBrw;
	
	/// Emit a codegen item to the provided writer.
	/// 
	/// This default implementation will attempt to emit results in-line, 
	/// so no extra characters, such as new lines or whitespace, will be emitted between calls to `emit`.
	fn emit<Emittable, EmError, W>(&'a self, e: &'a Emittable, writer: &'a mut W) -> Result<(), Self::Error> where 
		Emittable: Emit<Self::CtxtBrw, EmError>, 
		EmError: Into<EmitError>, 
		W: Write {
			let cx = self.get_cx();
			emit!(cx, e, writer)
	}

	/// Emit a string
	fn emit_str<W>(&self, e: &str, writer: &'a mut W) -> Result<(), Self::Error> where W: Write {
		emit_str!(e, writer)
	}
}

#[derive(Debug)]
enum EmitErrorKind {
	Io(IoError),
	Other(String)
}

/// Represents an error encountered during emission.
/// 
/// This could include errors while converting to string or writing.
#[derive(Debug)]
pub struct EmitError {
	kind: EmitErrorKind
}

impl fmt::Display for EmitError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.kind {
			EmitErrorKind::Io(ref err) => write!(f, "IO error: {}", err),
			EmitErrorKind::Other(ref err) => write!(f, "Error: {}", err)
		}
	}
}

impl error::Error for EmitError {
	fn description(&self) -> &str {
		match self.kind {
			EmitErrorKind::Io(ref err) => err.description(),
			EmitErrorKind::Other(ref err) => &err
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match self.kind {
			EmitErrorKind::Io(ref err) => Some(err),
			EmitErrorKind::Other(_) => None
		}
	}
}

impl From<IoError> for EmitError {
	fn from(err: IoError) -> EmitError {
		EmitError {
			kind: EmitErrorKind::Io(err)
		}
	}
}

impl From<String> for EmitError {
	fn from(err: String) -> EmitError {
		EmitError {
			kind: EmitErrorKind::Other(err)
		}
	}
}

/// The default result of emitting to a writer.
pub type EmitResult = Result<String, EmitError>;