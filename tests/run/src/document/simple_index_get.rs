use futures::Future;
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct SimpleIndexGet;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

const INDEX: &'static str = "simple_index_get_idx";
const ID: i32 = 1;

fn doc() -> Doc {
    Doc {
        id: ID,
        title: "A document title".to_owned(),
        timestamp: Date::build(2017, 03, 24, 13, 44, 0, 0),
    }
}

impl IntegrationTest for SimpleIndexGet {
    type Response = GetResponse<Doc>;

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "simple_index_get"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client
            .index_delete(index(INDEX))
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Index a document, then get it
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let index_res = client.document_index(index(INDEX), id(ID), doc())
            .params(|p| p.url_param("refresh", true))
            .send();
        
        let get_res = client.document_get(index(INDEX), id(ID)).send();

        Box::new(index_res.and_then(|_| get_res))
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.document() == Some(&doc())
    }
}
