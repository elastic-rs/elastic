use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, SeqVisitor, MapVisitor};
use serde_json::Value;
use parse::MaybeOkResponse;
use common::Shards;
use super::{HttpResponse, FromResponse, ApiResult};

use std::cmp;
use std::fmt;
use std::io::Read;
use std::error::Error;

type BulkError = Value;

/// Response for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).
/// 
/// This type splits successful and failed bulk operations so it's easier
/// to handle errors in bulk requests.
#[derive(Deserialize, Debug, Clone)]
pub struct BulkResponse {
    pub took: u64,
    errors: bool,
    pub items: BulkItems,
}

impl BulkResponse {
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    pub fn is_err(&self) -> bool {
        self.errors
    }
}

/// Response for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).
/// 
/// This type only accumulates bulk operations that failed.
/// It can be more efficient if you only care about errors.
#[derive(Deserialize, Debug, Clone)]
pub struct BulkErrorsResponse {
    pub took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_item_errors")]
    pub items: Vec<BulkItemError>,
}

impl BulkErrorsResponse {
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    pub fn is_err(&self) -> bool {
        self.errors
    }
}

/// A successful bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItem {
    pub action: BulkAction,
    pub index: String,
    pub ty: String,
    pub id: String,
    pub version: Option<u32>,
    pub shards: Option<Shards>,
    pub created: Option<bool>,
    pub found: Option<bool>,
}

/// A failed bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItemError {
    pub action: BulkAction,
    pub index: String,
    pub ty: String,
    pub id: String,
    pub err: BulkError
}

impl fmt::Display for BulkItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BulkItemError {
    fn description(&self) -> &str {
        "Bulk operation failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// Bulk items split by success or failure.
#[derive(Debug, Clone)]
pub struct BulkItems {
    pub ok: Vec<BulkItem>,
    pub err: Vec<BulkItemError>,
}

/// The bulk action being performed.
#[derive(Debug, Clone)]
pub enum BulkAction {
    Index,
    Create,
    Update,
    Delete,
}

impl FromResponse for BulkResponse {
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

impl FromResponse for BulkErrorsResponse {
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

// Deserialisation

impl Deserialize for BulkAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer 
    {
        struct BulkActionVisitor;

        impl Visitor for BulkActionVisitor {
            type Value = BulkAction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<BulkAction, E>
                where E: DeError
            {
                match value {
                    "index" => Ok(BulkAction::Index),
                    "create" => Ok(BulkAction::Create),
                    "update" => Ok(BulkAction::Update),
                    "delete" => Ok(BulkAction::Delete),
                    _ => Err(E::custom("unexpected bulk action")),
                }
            }
        }

        deserializer.deserialize_str(BulkActionVisitor)
    }
}

struct BulkItemDe {
    action: BulkAction,
    inner: BulkItemDeInner,
}

// TODO: Make this BulkItemDeInner<'a> after `serde` 1.0
// That way we can avoid allocating for ignored bulk ops.
#[derive(Deserialize, Debug, Clone)]
struct BulkItemDeInner {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Option<Shards>,
    pub created: Option<bool>,
    pub found: Option<bool>,
    status: u16,
    error: Option<BulkError>,
}

impl BulkItemDe {
    fn into_result(self) -> Result<BulkItem, BulkItemError> {
        match self.inner.error {
            Some(err) => Err(BulkItemError {
                action: self.action,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                err: err
            }),
            None => Ok(BulkItem {
                action: self.action,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                version: self.inner.version,
                shards: self.inner.shards,
                created: self.inner.created,
                found: self.inner.found,
            })
        }
    }
}

impl Deserialize for BulkItemDe {
    fn deserialize<D>(deserializer: D) -> Result<BulkItemDe, D::Error>
        where D: Deserializer 
    {
        struct BulkItemDeVisitor;

        impl Visitor for BulkItemDeVisitor {
            type Value = BulkItemDe;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a bulk item")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: MapVisitor 
            {
                let (action, inner) = visitor.visit()?.ok_or(V::Error::custom("expected at least one field"))?;

                let result = BulkItemDe {
                    action: action,
                    inner: inner
                };

                Ok(result)
            }
        }

        deserializer.deserialize(BulkItemDeVisitor)
    }
}

impl Deserialize for BulkItems {
    fn deserialize<D>(deserializer: D) -> Result<BulkItems, D::Error>
        where D: Deserializer 
    {
        struct BulkItemsVisitor;

        impl Visitor for BulkItemsVisitor {
            type Value = BulkItems;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<BulkItems, E>
                where E: DeError,
            {
                Ok(BulkItems { ok: vec![], err: vec![] })
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<BulkItems, V::Error>
                where V: SeqVisitor,
            {
                let mut values = BulkItems {
                    ok: Vec::with_capacity(cmp::min(visitor.size_hint().0, 4096)),
                    err: Vec::with_capacity(cmp::min(visitor.size_hint().0, 512))
                };

                while let Some(value) = visitor.visit::<BulkItemDe>()? {
                    match value.into_result() {
                        Ok(item) => values.ok.push(item),
                        Err(item) => values.err.push(item)
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize(BulkItemsVisitor)
    }
}

fn deserialize_bulk_item_errors<D>(deserializer: D) -> Result<Vec<BulkItemError>, D::Error>
    where D: Deserializer 
{
    struct BulkErrorItemsVisitor;

        impl Visitor for BulkErrorItemsVisitor {
            type Value = Vec<BulkItemError>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Vec<BulkItemError>, E>
                where E: DeError,
            {
                Ok(vec![])
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<BulkItemError>, V::Error>
                where V: SeqVisitor,
            {
                let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().0, 4096));

                while let Some(value) = visitor.visit::<BulkItemDe>()? {
                    match value.into_result() {
                        Err(item) => values.push(item),
                        _ => ()
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize(BulkErrorItemsVisitor)
}