//! Send a bulk request and return the updated document at the same time.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates a bulk request.
//! There are a few extra dials you can tweak on bulk requests to get more
//! performance out of them.
//! See the docs for `BulkResponse` for more details.

use elastic::prelude::*;
use elastic_derive::ElasticType;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;

#[derive(Serialize, Deserialize, ElasticType)]
#[elastic(index = "bulk_with_source_sample_index")]
struct NewsArticle {
    id: i64,
    likes: i64,
}
#[derive(Serialize, Deserialize, ElasticType, Debug)]
#[elastic(index = "bulk_with_source_sample_index")]
struct UpdatedNewsArticle {
    id: i64,
    likes: i64,
}

fn run() -> Result<(), Box<dyn::std::error::Error>> {
    let client = SyncClientBuilder::new().build()?;

    let index_ops = (0..100)
        .into_iter()
        .map(|i| bulk_raw().index(json!({ "id": i, "likes": 0 })).id(i));

    // Index some documents
    let _response = client
        .bulk()
        .index(NewsArticle::static_index())
        .ty(NewsArticle::static_ty())
        .extend(index_ops)
        .params_fluent(|p| p.url_param("refresh", true))
        .send()?;

    let update_ops = (0..100).into_iter().map(|i| {
        bulk::<NewsArticle>()
            .update_script(i, "ctx._source.likes++")
            // Request that the updated document's source be returned with the response
            .source(true)
    });

    // Update the documents
    let response = client
        .bulk()
        .index(NewsArticle::static_index())
        .ty(NewsArticle::static_ty())
        .extend(update_ops)
        .send()?;

    for op in response {
        if let Ok(op) = op {
            println!("{:?}", op.into_document::<UpdatedNewsArticle>().unwrap());
        }
    }

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
