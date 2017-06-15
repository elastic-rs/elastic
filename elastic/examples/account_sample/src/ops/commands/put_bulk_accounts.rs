use std::io::{Result as IoResult, Error as IoError};
use std::fs::File;
use std::path::Path;
use ops::Client;
use elastic::client::into_response;
use elastic::client::requests::{IntoBody, BulkRequest};
use elastic::client::responses::{BulkErrorsResponse, BulkItemError};
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
        let req = put(body);

        let res = self.io
            .request(req)
            .params(|params| params.url_param("refresh", true))
            .send()?
            .into_response::<BulkErrorsResponse>()?;

        if res.is_err() {
            return Err(res.items.into());
        }

        Ok(())
    }
}

fn put<B>(body: B) -> BulkRequest<'static, B>
    where B: IntoBody
{
    BulkRequest::for_index_ty(model::index::name(), model::account::name(), body)
}

fn bulk_body<P>(path: P) -> IoResult<File>
    where P: AsRef<Path>
{
    File::open(path)
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
        Bulk(err: Vec<BulkItemError>) {
            from()
            display("failed to put bulk accounts: {:?}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_request_url() {
        let req = put(vec![]);

        assert_eq!("/bank-sample/account/_bulk", req.url.as_ref());
    }
}
