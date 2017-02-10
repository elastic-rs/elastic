use ops::Client;
use std::io::Error as IoError;
use serde_json::{Value, Error as JsonError};
use elastic::client::requests::{Body, IndicesExistsRequest, IndicesCreateRequest};
use elastic::error::Error as ResponseError;

use model;

pub trait EnsureBankIndexExists {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError>;
}

impl EnsureBankIndexExists for Client {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError> {
        let exists = self.io.request(exists()).send()?;

        match exists.status() {
            // Success, do nothing
            200 => (),
            // Not found, create the index
            404 => {
                let body = model::index::body()?;

                self.io.request(put(body))
                    .send()
                    .and_then(|res| res.response::<Value>())?;
            },
            // Some other response, deserialise
            _ => {
                exists.response::<Value>()?;
            }
        }

        Ok(())
    }
}

fn exists() -> IndicesExistsRequest<'static> {
    IndicesExistsRequest::for_index(model::index::name())
}

fn put<B>(body: B) -> IndicesCreateRequest<'static>
    where B: Into<Body<'static>>
{
    IndicesCreateRequest::for_index(model::index::name(), body)
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn put_request_url() {
        let req = put(vec![]);

        assert_eq!("/bank-sample", req.url.as_ref());
    }
}
