use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(ty = "document", index = "bulk_index_get")]
pub struct Doc {
    #[elastic(id)]
    id: String,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

test! {
    const description: &'static str = "index and then get";

    type Response = GetResponse<Doc>;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Index some bulk documents
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let ops = (0..10).into_iter().map(|i| {
            bulk().index(Doc {
                id: i.to_string(),
                title: "A document title".to_owned(),
                timestamp: Date::build(2017, 03, 24, 13, 44, 0, 0),
            })
        });

        let bulk_res = client
            .bulk()
            .extend(ops)
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let get_res = client.document().get("4").send();

        Box::new(bulk_res.and_then(|_| get_res))
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.document().map(|doc| &*doc.id) == Some("4")
    }
}
