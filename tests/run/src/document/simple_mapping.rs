use elastic::error::Error;
use elastic::prelude::*;
use futures::Future;
use run_tests::IntegrationTest;

use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct SimpleMapping;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(ty = "document", index = "simple_mapping")]
pub struct Doc {
    #[elastic(id)]
    id: String,
    timestamp: Date<DefaultDateMapping>,
}

impl IntegrationTest for SimpleMapping {
    type Response = Value;

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "simple mapping"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Put the document mapping, then get it back from Elasticsearch
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let create_index = client.index(Doc::static_index()).create().send().map(|_| ());

        let put_mapping = client
            .document::<Doc>()
            .put_mapping()
            .send();

        let get_mapping = client
            .request(IndicesGetMappingRequest::for_index_ty(Doc::static_index(), Doc::static_ty()))
            .send()
            .and_then(|res| res.into_response::<Value>());

        Box::new(create_index.and_then(|_| put_mapping).and_then(|_| get_mapping))
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let expected = json!({
            "simple_mapping": {
                "mappings": {
                    "document": {
                        "properties": {
                            "id": {
                                "type": "text",
                                "fields": {
                                    "keyword": {
                                        "type": "keyword",
                                        "ignore_above": 256
                                    }
                                }
                            },
                            "timestamp": {
                                "type": "date",
                                "format": "basic_date_time"
                            }
                        }
                    }
                }
            }
        });

        res.to_string() == expected.to_string()
    }
}
