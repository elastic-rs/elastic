use serde_json::{Map, Value};
use parse::MaybeOkResponse;
use common::Shards;
use super::{HttpResponse, FromResponse, ApiResult};

use std::io::Read;

type BulkError = Map<String, Value>;

// TODO: The `success` functions might be a bit pessimistic

#[derive(Deserialize, Debug)]
pub struct BulkResponse {
    pub took: u64,
    pub errors: bool,
    pub items: Vec<BulkItem>,
}

impl BulkResponse {
    pub fn success(&self) -> bool {
        let any_item_errors = self.items.iter().any(|item| !item.success());

        !self.errors && !any_item_errors
    }
}

#[derive(Deserialize, Debug)]
pub enum BulkItem {
    Index(BulkIndexItem),
    Create(BulkCreateItem),
    Update(BulkUpdateItem),
    Delete(BulkDeleteItem),
}

impl BulkItem {
    pub fn success(&self) -> bool {
        match *self {
            BulkItem::Index(ref item) => item.success(),
            BulkItem::Create(ref item) => item.success(),
            BulkItem::Update(ref item) => item.success(),
            BulkItem::Delete(ref item) => item.success(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BulkIndexItem {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub status: u16,
    pub error: Option<BulkError>,
    pub created: bool,
}

impl BulkIndexItem {
    fn success(&self) -> bool {
        match (&self.error, self.status) {
            (&None, 200...299) => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BulkCreateItem {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub status: u16,
    pub error: Option<BulkError>,
    pub created: bool,
}

impl BulkCreateItem {
    pub fn success(&self) -> bool {
        match (&self.error, self.status) {
            (&None, 200...299) => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BulkUpdateItem {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub status: u16,
    pub error: Option<BulkError>,
}

impl BulkUpdateItem {
    pub fn success(&self) -> bool {
        match (&self.error, self.status) {
            (&None, 200...299) => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BulkDeleteItem {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub status: u16,
    pub error: Option<BulkError>,
    pub found: bool,
}

impl BulkDeleteItem {
    pub fn success(&self) -> bool {
        match (&self.error, self.status) {
            (&None, 200...299) | (&None, 404) => true,
            _ => false,
        }
    }
}

impl FromResponse for BulkItem {
    fn from_response<I: Into<HttpResponse<R>>, R: Read>(res: I) -> ApiResult<Self> {
        let res = res.into();

        res.response(|res| {
            match res.status() {
                200...299 => Ok(MaybeOkResponse::ok(res)),
                _ => Ok(MaybeOkResponse::err(res)),
            }
        })
    }
}
