use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "bulk_delete")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

const ID: &'static str = "1";

fn doc() -> Doc {
    Doc { id: ID.to_owned() }
}

test! {
    const description: &'static str = "index and delete a document";

    type Response = BulkResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Index a document, get it, delete it, then try get it again
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let index_res = client
            .document()
            .index(doc())
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let delete_res = client.bulk().push(bulk::<Doc>().delete(ID)).send();

        Box::new(index_res.and_then(|_| delete_res))
    }

    // Ensure the document was found before deleting but not found after deleting
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let deleted = res.iter().next().unwrap().unwrap();

        deleted.action() == BulkAction::Delete && deleted.deleted()
    }
}
