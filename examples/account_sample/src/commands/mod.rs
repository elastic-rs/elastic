mod ensure_bank_index_exists;
mod put_bulk_accounts;

pub use self::ensure_bank_index_exists::EnsureBankIndexExists;
pub use self::put_bulk_accounts::PutBulkAccounts;

trait EnsureSuccess {
	type Result;
	type Err: Error;

	fn ensure_success(self) -> Result<Self::Result, Self::Error>;
}

impl EnsureSuccess for ResponseBuilder {
	type Result = ();
	type Err = ResponseError;

	fn ensure_success(mut self) -> Result<(), ResponseError> {
		let success = self.status().is_success();

		match success {
			true => Ok(()),
			false => {
				let mut body = String::new();
	        	self.raw().read_to_string(&mut body)?;

	        	Err(ResponseError::Other(body))
			}
		}
	}
}