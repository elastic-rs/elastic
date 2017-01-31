use model;
use super::EnsureSuccess;

pub trait EnsureBankIndexExists {
    fn ensure_index_existsensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError>;
}

impl EnsureBankIndexExists for Client {
    fn ensure_bank_index_exists(&self) -> Result<(), EnsureBankIndexExistsError> {
        let exists = self.request(exists()).send()?;

        match exists.status() {
            StatusCode::NotFound => {
                let body = model::index::body()?;

                let res = self.request(put(body)).send()?;

                res.ensure_success()?
            }
            _ => (),
        }

        exists.ensure_success()?
    }
}

fn exists() -> IndicesExistsRequest<'static> {
    IndicesExistsRequest::for_index(model::index::name())
}

fn put<B>(body: B) -> IndicesCreateRequest<'static>
    where B: Into<Body<'static>>
{
    Ok(IndicesCreateRequest::for_index(model::index::name(), body))
}

#[cfg(test)]
mod tests {
    #[test]
    fn put_request_url() {
        let req = put(vec![]);

        assert_eq!("/bank-sample", req.url.as_ref());
    }
}
