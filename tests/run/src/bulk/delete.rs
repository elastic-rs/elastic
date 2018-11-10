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

const INDEX: &'static str = "bulk_delete";
const ID: i32 = 1;

impl IntegrationTest for Delete {
    type Response = BulkResponse;

    fn kind() -> &'static str {
        "bulk"
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
            .document_index(index(INDEX), Doc { id: ID })
            .id(ID)
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let delete_res = client
            .bulk()
            .push(bulk_delete()
                .index(INDEX)
                .ty(Doc::name())
                .id(ID))
            .send();

        Box::new(
            index_res
                .and_then(|_| delete_res)
        )
    }

    // Ensure the document was found before deleting but not found after deleting
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let deleted = res.iter().next().unwrap().unwrap();

        deleted.action() == BulkAction::Delete && deleted.found()
    }
}
