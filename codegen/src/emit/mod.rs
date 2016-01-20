//! Elasticsearch Emitter
//! 
//! Utilities for emitting generated code to some output.

pub mod rust;

use std::marker::PhantomData;
use std::io::Write;
use std::error;
use std::fmt;
use std::io::Error as IoError;

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
	fn get_cx(&self) -> Self::CtxtBrw;
	
	/// Emit a codegen item to the provided writer.
	/// 
	/// This default implementation will attempt to emit results in-line, 
	/// so no extra characters, such as new lines or whitespace, will be emitted.
	fn emit<Em, Er, W>(&self, e: &'a Em, writer: &'a mut W) -> Result<(), Self::Error> where 
		Em: Emit<Self::CtxtBrw, Er>, 
		Er: Into<EmitError>, 
		W: Write {
			DefaultEmitter::emit::<Self::CtxtBrw, Self::Error, Em, Er, W>(self.get_cx(), e, writer)
	}
}

/// Default context-free Emitter
/// 
/// This emitter will emit items that implement `Emit<(), _>`
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
	/// 
	/// let emitter: CtxtFreeEmitter = CtxtFreeEmitter::new();
	/// ```
	/// 
	/// Create a `CtxtFreeEmitter` with the specified `Error` type:
	/// 
	/// ```
	/// use elastic_codegen::emit::*;
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

	fn emit<Em, Er, W>(&self, e: &'a Em, writer: &'a mut W) -> Result<(), Self::Error> where 
		Em: Emit<Self::CtxtBrw, Er>, 
		Er: Into<EmitError>, 
		W: Write {
			DefaultEmitter::emit::<Self::CtxtBrw, Self::Error, Em, Er, W>(self.get_cx(), e, writer)
	}
}

struct DefaultEmitter;
impl DefaultEmitter {
	pub fn emit<'a, Cb, E, Em, Er, W>(c: Cb, e: &'a Em, writer: &'a mut W) -> Result<(), E>  where 
		Em: Emit<Cb, Er>, 
		E: From<EmitError>, 
		Er: Into<EmitError>, 
		W: Write {
			let emitted = try!(
				e.emit(c).map_err(|e| e.into())
			);
			
			writer.write_all(&emitted.into_bytes()[..]).map_err(|e| {
				let emit_err: EmitError = e.into();
				emit_err.into()
			})
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
			EmitErrorKind::Other(ref err) => &err[..]
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