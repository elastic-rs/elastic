use parse::MaybeOkResponse;
use common::Shards;
use super::{HttpResponse, FromResponse, ApiResult};

use std::io::Read;

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
    pub created: bool,
}

impl BulkIndexItem {
    fn success(&self) -> bool {
        match self.status {
            200...299 => true,
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
    pub created: bool,
}

impl BulkCreateItem {
    pub fn success(&self) -> bool {
        match self.status {
            200...299 => true,
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
}

impl BulkUpdateItem {
    pub fn success(&self) -> bool {
        match self.status {
            200...299 => true,
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
    pub found: bool,
}

impl BulkDeleteItem {
    pub fn success(&self) -> bool {
        match self.status {
            200...299 | 404 => true,
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
