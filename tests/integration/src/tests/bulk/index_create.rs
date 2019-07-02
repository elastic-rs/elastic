use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    #[elastic(id)]
    id: String,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

const INDEX: &'static str = "bulk_index_create";
const ID: &'static str = "1";

fn doc() -> Doc {
    Doc {
        id: ID.to_owned(),
        title: "A document title".to_owned(),
        timestamp: Date::build(2017, 03, 24, 13, 44, 0, 0),
    }
}

test! {
    const description: &'static str = "create a doc along with its index";

    type Response = BulkResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Index a document, then get it
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let bulk_res = client.bulk().push(bulk().create(doc())).send();

        Box::new(bulk_res)
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let created = res.iter().next().unwrap().unwrap();

        created.action() == BulkAction::Create && created.created()
    }
}
