//! Elasticsearch Emitter Numerical Implementations

pub mod i {
	//! Elasticsearch Emitter Signed Int Implementations

	use ::emit::{ Emit, EmitError };

	impl Emit<(), EmitError> for i8 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for i16 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for i32 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for i64 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for isize {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}
}

pub mod u {
	//! Elasticsearch Emitter Unsigned Int Implementations

	use ::emit::{ Emit, EmitError };

	impl Emit<(), EmitError> for u8 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for u16 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for u32 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for u64 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for usize {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}
}

pub mod f {
	//! Elasticsearch Emitter Float Implementations

	use ::emit::{ Emit, EmitError };

	impl Emit<(), EmitError> for f32 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}

	impl Emit<(), EmitError> for f64 {
		fn emit(&self, _: ()) -> Result<String, EmitError> {
			Ok(self.to_string())
		}
	}
}