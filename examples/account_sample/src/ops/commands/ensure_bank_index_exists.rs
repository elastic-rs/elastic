use elastic::{
    http::StatusCode,
    prelude::*,
    Error as ResponseError,
};
use ops::Client;
use serde_json::Error as JsonError;
use std::io::Error as IoError;

use model;

pub trait EnsureBankIndexExists {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError>;
}

impl EnsureBankIndexExists for Client {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError> {
        let exists = self
            .io
            .request(IndicesExistsRequest::for_index(model::index::name()))
            .send()?;

        match exists.status() {
            // Success, do nothing
            StatusCode::OK => (),
            // Not found, create the index
            StatusCode::NOT_FOUND => {
                self.io
                    .index(model::index::name())
                    .create()
                    .body(model::index::body().to_string())
                    .send()?;
            }
            // Some other response, deserialise
            _ => {
                exists.into_response::<CommandResponse>()?;
            }
        }

        Ok(())
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum EnsureBankIndexExistsError {
        Io(err: IoError) {
            from()
            display("failed to ensure index exists: {}", err)
        }
        Json(err: JsonError) {
            from()
            display("failed to ensure index exists: {}", err)
        }
        Response(err: ResponseError) {
            from()
            display("failed to ensure index exists: {}", err)
        }
    }
}
