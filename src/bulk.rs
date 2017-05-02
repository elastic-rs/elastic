use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, SeqAccess, MapAccess};
use serde_json::Value;
use common::Shards;

use parse::{HttpResponseHead, IsOk, ResponseBody, MaybeOkResponse, ApiResult};
use error::*;

use std::cmp;
use std::fmt;
use std::io::Read;
use std::error::Error;
use std::borrow::Cow;

type BulkError = Value;

/// Response for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).
/// 
/// This type splits successful and failed bulk operations so it's easier
/// to handle errors in bulk requests.
/// 
/// # Examples
/// 
/// Send a bulk request and iterate through the results:
/// 
/// ```no_run
/// # extern crate elastic_responses;
/// # use elastic_responses::*;
/// # fn do_request() -> BulkResponse { unimplemented!() }
/// # fn main() {
/// // Send a request (omitted, see `samples/bulk`), and read the response.
/// // Parse body to JSON as an elastic_responses::BulkResponse object
/// let body_as_json: BulkResponse = do_request();
///
/// // Do something with successful operations
/// for op in body_as_json.items.ok {
///     match op.action {
///         BulkAction::Create => {
///             println!("created index: {}, type: {}, id: {}", 
///                 op.index, 
///                 op.ty, 
///                 op.id
///             );
///         },
///         _ => ()
///     }
/// }
///
/// // Do something with failed operations
/// for op in body_as_json.items.err {
///     match op.action {
///         BulkAction::Delete => (), // Ignore failed deletes
///         _ => println!("bulk op failed: {:?}", op) 
///     }
/// }
/// # }
/// ```
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
/// 
/// # Examples
/// 
/// Send a bulk request and iterate through the errors:
/// 
/// ```no_run
/// # extern crate elastic_responses;
/// # use elastic_responses::*;
/// # fn do_request() -> BulkErrorsResponse { unimplemented!() }
/// # fn main() {
/// // Send a request (omitted, see `samples/bulk`), and read the response.
/// // Parse body to JSON as an elastic_responses::BulkErrorsResponse object
/// let body_as_json: BulkErrorsResponse = do_request();
/// 
/// // Do something with failed operations
/// for op in body_as_json.items {
///     match op.action {
///         BulkAction::Delete => (), // Ignore failed deletes
///         _ => println!("bulk op failed: {:?}", op) 
///     }
/// }
/// # }
/// ```
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
#[derive(Deserialize, Debug, Clone)]
pub enum BulkAction {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

impl IsOk for BulkResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: B) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

impl IsOk for BulkErrorsResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: B) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

// Deserialisation

struct BulkItemDe<'de> {
    action: BulkAction,
    inner: BulkItemDeInner<'de>,
}

#[derive(Deserialize, Debug, Clone)]
struct BulkItemDeInner<'a> {
    #[serde(rename = "_index", borrow)]
    pub index: Cow<'a, str>,
    #[serde(rename = "_type", borrow)]
    pub ty: Cow<'a, str>,
    #[serde(rename = "_id", borrow)]
    pub id: Cow<'a, str>,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Option<Shards>,
    pub created: Option<bool>,
    pub found: Option<bool>,
    status: u16,
    error: Option<BulkError>,
}

impl<'de> BulkItemDe<'de> {
    fn into_err(self) -> Option<BulkItemError> {
        match self.inner.error {
            Some(err) => Some(BulkItemError {
                action: self.action,
                index: self.inner.index.into_owned(),
                ty: self.inner.ty.into_owned(),
                id: self.inner.id.into_owned(),
                err: err
            }),
            None => None
        }
    }

    fn into_result(self) -> Result<BulkItem, BulkItemError> {
        match self.inner.error {
            Some(err) => Err(BulkItemError {
                action: self.action,
                index: self.inner.index.into_owned(),
                ty: self.inner.ty.into_owned(),
                id: self.inner.id.into_owned(),
                err: err
            }),
            None => Ok(BulkItem {
                action: self.action,
                index: self.inner.index.into_owned(),
                ty: self.inner.ty.into_owned(),
                id: self.inner.id.into_owned(),
                version: self.inner.version,
                shards: self.inner.shards,
                created: self.inner.created,
                found: self.inner.found,
            })
        }
    }
}

impl<'de> Deserialize<'de> for BulkItemDe<'de> {
    fn deserialize<D>(deserializer: D) -> Result<BulkItemDe<'de>, D::Error>
        where D: Deserializer<'de> 
    {
        struct BulkItemDeVisitor;

        impl<'de> Visitor<'de> for BulkItemDeVisitor {
            type Value = BulkItemDe<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a bulk item")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: MapAccess<'de>
            {
                let (action, inner) = visitor.next_entry()?.ok_or(V::Error::custom("expected at least one field"))?;

                let result = BulkItemDe {
                    action: action,
                    inner: inner
                };

                Ok(result)
            }
        }

        deserializer.deserialize_any(BulkItemDeVisitor)
    }
}

impl<'de> Deserialize<'de> for BulkItems {
    fn deserialize<D>(deserializer: D) -> Result<BulkItems, D::Error>
        where D: Deserializer<'de>
    {
        struct BulkItemsVisitor;

        impl<'de> Visitor<'de> for BulkItemsVisitor {
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
                where V: SeqAccess<'de>,
            {
                let mut values = BulkItems {
                    ok: Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096)),
                    err: Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 512))
                };

                while let Some(value) = visitor.next_element::<BulkItemDe>()? {
                    match value.into_result() {
                        Ok(item) => values.ok.push(item),
                        Err(item) => values.err.push(item)
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkItemsVisitor)
    }
}

fn deserialize_bulk_item_errors<'de, D>(deserializer: D) -> Result<Vec<BulkItemError>, D::Error>
    where D: Deserializer <'de>
{
    struct BulkErrorItemsVisitor;

        impl<'de> Visitor<'de> for BulkErrorItemsVisitor {
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
                where V: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096));

                while let Some(value) = visitor.next_element::<BulkItemDe>()? {
                    if let Some(value) = value.into_err() {
                        values.push(value);
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkErrorItemsVisitor)
}