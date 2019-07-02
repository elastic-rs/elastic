use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "simple_mapping")]
pub struct Doc {
    #[elastic(id)]
    id: String,
    timestamp: Date<DefaultDateMapping>,
}

test! {
    const description: &'static str = "put simple derived mapping";

    type Response = Value;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Put the document mapping, then get it back from Elasticsearch
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let create_index = client
            .index(Doc::static_index())
            .create()
            .send()
            .map(|_| ());

        let put_mapping = client.document::<Doc>().put_mapping().send();

        let get_mapping = client
            .request(IndicesGetMappingRequest::for_index(
                Doc::static_index(),
            ))
            .send()
            .and_then(|res| res.into_response::<Value>());

        Box::new(
            create_index
                .and_then(|_| put_mapping)
                .and_then(|_| get_mapping),
        )
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let expected = json!({
            "simple_mapping": {
                "mappings": {
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
        });

        res.to_string() == expected.to_string()
    }
}
