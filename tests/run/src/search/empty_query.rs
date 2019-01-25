use futures::{future, Future};
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct EmptyQuery;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "empty_query_idx")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

fn doc() -> Doc {
    Doc {
        id: "1".to_owned(),
    }
}

impl IntegrationTest for EmptyQuery {
    type Response = SearchResponse<Doc>;

    fn kind() -> &'static str {
        "search"
    }
    fn name() -> &'static str {
        "empty_query"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
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
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.search().index(Doc::static_index()).send();

        Box::new(res)
    }

    // Ensure the response contains documents
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let correct_hits = res.hits()
            .all(|hit| hit.index() == Doc::static_index() && hit.ty() == Doc::static_ty());
        let len_greater_than_0 = res.documents().count() > 0;

        correct_hits && len_greater_than_0
    }
}
