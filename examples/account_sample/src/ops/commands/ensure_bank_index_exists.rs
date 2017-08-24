use ops::Client;
use std::io::Error as IoError;
use serde_json::Error as JsonError;
use elastic::client::requests::IndicesExistsRequest;
use elastic::client::responses::CommandResponse;
use elastic::error::Error as ResponseError;

use model;

pub trait EnsureBankIndexExists {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError>;
}

impl EnsureBankIndexExists for Client {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError> {
        let exists = self.io
            .request(IndicesExistsRequest::for_index(model::index::name()))
            .send()?;

        match exists.status() {
            // Success, do nothing
            200 => (),
            // Not found, create the index
            404 => {
                self.io
                    .index_create(model::index::name())
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

quick_error!{
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
