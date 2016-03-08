//! Elasticsearch Emitter String Implementations

use ::emit::{ Emit, EmitError };

impl Emit<(), EmitError> for AsRef<String> {
	fn emit(&self, _: ()) -> Result<String, EmitError> {
		Ok(self.as_ref().clone())
	}
}

impl <'a> Emit<(), EmitError> for &'a str {
	fn emit(&self, _: ()) -> Result<String, EmitError> {
		Ok((*self).to_owned())
	}
}

impl Emit<(), EmitError> for char {
	fn emit(&self, _: ()) -> Result<String, EmitError> {
		Ok(self.to_string())
	}
}