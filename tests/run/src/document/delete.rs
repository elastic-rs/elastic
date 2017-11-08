use futures::Future;
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct Delete;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    id: i32,
}

const INDEX: &'static str = "delete_doc_idx";
const ID: i32 = 1;

impl IntegrationTest for Delete {
    type Response = (GetResponse<Doc>, DeleteResponse, GetResponse<Doc>);

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "delete"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index_delete(index(INDEX)).send().map(|_| ());

        Box::new(delete_res)
    }

    // Index a document, get it, delete it, then try get it again
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let index_res = client
            .document_index(index(INDEX), id(ID), Doc { id: ID })
            .params(|p| p.url_param("refresh", true))
            .send();

        let pre_delete_res = client.document_get(index(INDEX), id(ID)).send();

        let delete_res = client
            .document_delete::<Doc>(index(INDEX), id(ID))
            .params(|p| p.url_param("refresh", true))
            .send();

        let post_delete_res = client.document_get(index(INDEX), id(ID)).send();

        Box::new(
            index_res
                .and_then(|_| pre_delete_res)
                .and_then(|pre| delete_res.map(|del| (pre, del)))
                .and_then(|(pre, del)| post_delete_res.map(|post| (pre, del, post))),
        )
    }

    // Ensure the document was found before deleting but not found after deleting
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let pre = &res.0;
        let delete = &res.1;
        let post = &res.2;

        pre.found() && delete.deleted() && !post.found()
    }
}
