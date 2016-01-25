//! Elasticsearch Emitter Boolean Implementations

use ::emit::{ Emit, EmitError };

impl Emit<(), EmitError> for bool {
	fn emit(&self, _: ()) -> Result<String, EmitError> {
		Ok(self.to_string())
	}
}