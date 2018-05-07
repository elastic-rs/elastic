use futures::{future, Future};
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct EmptyQuery;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    id: i32,
}

const INDEX: &'static str = "empty_query_idx";

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
        let delete_res = client.index_delete(index(INDEX)).send();

        let index_reqs = future::join_all((0..10).into_iter().map(move |i| {
            client
                .document_index(index(INDEX), Doc { id: i })
                .id(i)
                .params_fluent(|p| p.url_param("refresh", true))
                .send()
        }));

        Box::new(delete_res.then(|_| index_reqs.map(|_| ())))
    }

    // Execute a search request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.search().index(INDEX).send();

        Box::new(res)
    }

    // Ensure the response contains documents
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let correct_hits = res.hits()
            .all(|hit| hit.index() == INDEX && hit.ty() == Doc::name());
        let len_greater_than_0 = res.documents().count() > 0;

        correct_hits && len_greater_than_0
    }
}
