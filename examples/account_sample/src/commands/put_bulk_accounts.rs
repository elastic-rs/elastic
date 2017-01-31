use std::io::{Read, Result as IoResult};
use std::fs::File;
use std::path::Path;
use elastic::client::requests::BulkRequest;

use model;
use super::EnsureSuccess;

pub trait PutBulkAccounts {
    fn put_bulk_accounts<P>(&self, path: P) -> Result<(), PutBulkAccountsError> where P: AsRef<Path>;
}

impl PutBulkAccounts for Client {
    fn put_bulk_accounts<P>(&self, path: P) -> Result<(), PutBulkAccountsError>
        where P: AsRef<Path>
    {
        let body = bulk_body(path)?;
        let res = client.request(put(body)).send()?;

        res.ensure_success()?
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

#[cfg(test)]
mod tests {
    #[test]
    fn put_request_url() {
        let req = put(vec![]);

        assert_eq!("/bank-sample/account", req.url.as_ref());
    }
}
