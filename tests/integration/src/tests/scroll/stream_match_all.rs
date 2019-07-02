use elastic::{
    error::Error,
    prelude::*,
};
use futures::{
    future,
    Future,
};

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "stream_match_all")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

fn doc() -> Doc {
    Doc { id: "1".to_owned() }
}

test! {
    const description: &'static str = "scroll through all docs";

    type Response = SearchResponse<Doc>;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(Doc::static_index()).delete().send();

        let index_reqs = future::join_all((0..10).into_iter().map(move |_| {
            client
                .document()
                .index(doc())
                .params_fluent(|p| p.url_param("refresh", true))
                .send()
        }));

        Box::new(delete_res.then(|_| index_reqs.map(|_| ())))
    }

    // Execute a search request against that index
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let scroll = client
            .scroll("1m")
            .index(Doc::static_index())
            .body(json!({
                "size": 1,
                "query": {
                    "match_all": {}
                }
            }))
            .build();
        
        // TODO: Collect as `Stream`

        Box::new(res)
    }

    // Ensure the response contains a scroll id
    fn assert_ok(&self, res: &Self::Response) -> bool {
        false
    }
}
