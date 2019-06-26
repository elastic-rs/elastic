use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "delete_doc_idx")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

const ID: &'static str = "1";

fn doc() -> Doc {
    Doc { id: ID.to_owned() }
}

test! {
    const description: &'static str = "delete existing document";

    type Response = (GetResponse<Doc>, DeleteResponse, GetResponse<Doc>);

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

        let pre_delete_res = client.document().get(ID).send();

        let delete_res = client
            .document::<Doc>()
            .delete(ID)
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let post_delete_res = client.document().get(ID).send();

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
