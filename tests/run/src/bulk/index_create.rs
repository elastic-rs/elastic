use elastic::error::Error;
use elastic::prelude::*;
use futures::Future;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct IndexCreate;

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

impl IntegrationTest for IndexCreate {
    type Response = BulkResponse;

    fn kind() -> &'static str {
        "bulk"
    }
    fn name() -> &'static str {
        "index_create"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Index a document, then get it
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let bulk_res = client.bulk().push(bulk().create(doc())).send();

        Box::new(bulk_res)
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let created = res.iter().next().unwrap().unwrap();

        created.action() == BulkAction::Create && created.created()
    }
}
