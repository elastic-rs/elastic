use std::io::{Read, Result as IoResult, Error as IoError};
use std::fs::File;
use std::path::Path;
use serde_json::Value;
use elastic::client::Client;
use elastic::client::requests::{Body, BulkRequest};
use elastic::error::Error as ResponseError;

use model;

pub trait PutBulkAccounts {
    fn put_bulk_accounts<P>(&self, path: P) -> Result<(), PutBulkAccountsError> where P: AsRef<Path>;
}

impl PutBulkAccounts for Client {
    fn put_bulk_accounts<P>(&self, path: P) -> Result<(), PutBulkAccountsError>
        where P: AsRef<Path>
    {
        let body = bulk_body(path)?;

        self.request(put(body))
            .send()
            .and_then(|res| res.response::<Value>())?;

        Ok(())
    }
}

fn put<B>(body: B) -> BulkRequest<'static>
    where B: Into<Body<'static>>
{
    BulkRequest::for_index_ty(model::index::name(), model::account::name(), body)
}

fn bulk_body<P>(path: P) -> IoResult<Vec<u8>>
    where P: AsRef<Path>
{
    let mut body = File::open(path)?;

    let mut buf = Vec::new();
    body.read_to_end(&mut buf)?;

    Ok(buf)
}

quick_error!{
    #[derive(Debug)]
    pub enum PutBulkAccountsError {
        Io(err: IoError) {
            from()
            display("failed to put bulk accounts: {}", err)
        }
        Response(err: ResponseError) {
            from()
            display("failed to put bulk accounts: {}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn put_request_url() {
        let req = put(vec![]);

        assert_eq!("/bank-sample/account", req.url.as_ref());
    }
}
