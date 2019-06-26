use elastic::{
    error::Error,
    prelude::*,
};
use futures::{
    future,
    Future,
};

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "sql_select_all_idx")]
pub struct Doc {
    #[elastic(id(expr = "id.to_string()"))]
    id: i32,
    field1: String,
}

fn doc(i: i32) -> Doc {
    Doc {
        id: i,
        field1: "field1".to_owned(),
    }
}

test! {
    const description: &'static str = "select all documents";

    type Response = SqlResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(Doc::static_index()).delete().send();

        let index_reqs = future::join_all((0..10).into_iter().map(move |i| {
            client
                .document()
                .index(doc(i))
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
        let res = client
            .sql_query(&format!("select * from {}", Doc::static_index()))
            .send();

        Box::new(res)
    }

    // Ensure the response contains documents
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let correct_columns = vec!["id", "field1"].sort()
            == res
                .columns()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<&str>>()
                .sort();

        let correct_length = res.rows().len() == 10;

        correct_columns && correct_length
    }
}
