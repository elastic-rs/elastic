use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, SeqAccess, MapAccess};
use serde_json::Value;
use common::Shards;

use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

use std::cmp;
use std::fmt;
use std::marker::PhantomData;
use std::error::Error;

type BulkError = Value;

type DefaultAllocatedField = String;

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
/// 
/// # Optimising bulk responses
/// 
/// The `BulkResponse` type has a few generic parameters for the index, type and id fields.
/// If your bulk operations have a small set of possible values for these fields you can avoid
/// allocating `String`s on the heap by using an alternative type, like an `enum`.
/// 
/// In the example below, we expect all bulk operations to use either a type called `mytypea` or `mytypeb`
/// and an index called `myindex :
/// 
/// ```no_run
/// # extern crate serde;
/// # #[macro_use] extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate elastic_responses;
/// # use elastic_responses::*;
/// # fn main() {
/// # fn do_request() -> BulkResponse<Index, Type> { unimplemented!() }
/// #[derive(Deserialize)]
/// enum Index {
///     #[serde(rename = "myindex")]
///     MyIndex,
/// }
/// 
/// #[derive(Deserialize)]
/// enum Type {
///     #[serde(rename = "mytypea")]
///     MyTypeA,
///     #[serde(rename = "mytypeb")]
///     MyTypeB,
/// }
/// 
/// let bulk: BulkResponse<Index, Type> = do_request();
/// # }
/// ``` 
#[derive(Deserialize, Debug, Clone)]
pub struct BulkResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    pub took: u64,
    errors: bool,
    pub items: BulkItems<TIndex, TType, TId>,
}

impl<TIndex, TType, TId> BulkResponse<TIndex, TType, TId> {
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
///
/// # Optimising bulk responses
/// 
/// The `BulkErrorsResponse` type has a few generic parameters for the index, type and id fields.
/// If your bulk operations have a small set of possible values for these fields you can avoid
/// allocating `String`s on the heap by using an alternative type, like an `enum`.
/// 
/// In the example below, we expect all bulk operations to use either a type called `mytypea` or `mytypeb`
/// and an index called `myindex :
/// 
/// ```no_run
/// # extern crate serde;
/// # #[macro_use] extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate elastic_responses;
/// # use elastic_responses::*;
/// # fn main() {
/// # fn do_request() -> BulkErrorsResponse<Index, Type> { unimplemented!() }
/// #[derive(Deserialize)]
/// enum Index {
///     #[serde(rename = "myindex")]
///     MyIndex,
/// }
/// 
/// #[derive(Deserialize)]
/// enum Type {
///     #[serde(rename = "mytypea")]
///     MyTypeA,
///     #[serde(rename = "mytypeb")]
///     MyTypeB,
/// }
/// 
/// let bulk: BulkErrorsResponse<Index, Type> = do_request();
/// # }
/// ``` 
#[derive(Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "TIndex: Deserialize<'de>, TType: Deserialize<'de>, TId: Deserialize<'de>"))]
pub struct BulkErrorsResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    pub took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_item_errors")]
    pub items: Vec<BulkItemError<TIndex, TType, TId>>,
}

impl<TIndex, TType, TId> BulkErrorsResponse<TIndex, TType, TId> {
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    pub fn is_err(&self) -> bool {
        self.errors
    }
}

/// A successful bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItem<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    pub action: BulkAction,
    pub index: TIndex,
    pub ty: TType,
    pub id: TId,
    pub version: Option<u32>,
    pub shards: Option<Shards>,
    pub created: Option<bool>,
    pub found: Option<bool>,
}

/// A failed bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItemError<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    pub action: BulkAction,
    pub index: TIndex,
    pub ty: TType,
    pub id: TId,
    pub err: BulkError
}

impl<TIndex, TType, TId> fmt::Display for BulkItemError<TIndex, TType, TId>
    where TIndex: fmt::Display + fmt::Debug,
          TType: fmt::Display + fmt::Debug,
          TId: fmt::Display + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<TIndex, TType, TId> Error for BulkItemError<TIndex, TType, TId> 
    where TIndex: fmt::Display + fmt::Debug,
          TType: fmt::Display + fmt::Debug,
          TId: fmt::Display + fmt::Debug
{
    fn description(&self) -> &str {
        "Bulk operation failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// Bulk items split by success or failure.
#[derive(Debug, Clone)]
pub struct BulkItems<TIndex, TType, TId> {
    pub ok: Vec<BulkItem<TIndex, TType, TId>>,
    pub err: Vec<BulkItemError<TIndex, TType, TId>>,
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

impl<TIndex, TType, TId> IsOk for BulkResponse<TIndex, TType, TId> {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

impl<TIndex, TType, TId> IsOk for BulkErrorsResponse<TIndex, TType, TId> {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

// Deserialisation

struct BulkItemDe<TIndex, TType, TId> {
    action: BulkAction,
    inner: BulkItemDeInner<TIndex, TType, TId>,
}

#[derive(Deserialize, Debug, Clone)]
struct BulkItemDeInner<TIndex, TType, TId> {
    #[serde(rename = "_index")]
    pub index: TIndex,
    #[serde(rename = "_type")]
    pub ty: TType,
    #[serde(rename = "_id")]
    pub id: TId,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_shards")]
    pub shards: Option<Shards>,
    pub created: Option<bool>,
    pub found: Option<bool>,
    status: u16,
    error: Option<BulkError>,
}

impl<TIndex, TType, TId> BulkItemDe<TIndex, TType, TId> {
    fn into_err(self) -> Option<BulkItemError<TIndex, TType, TId>> {
        match self.inner.error {
            Some(err) => Some(BulkItemError {
                action: self.action,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                err: err,
            }),
            None => None
        }
    }

    fn into_result(self) -> Result<BulkItem<TIndex, TType, TId>, BulkItemError<TIndex, TType, TId>> {
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

impl<'de, TIndex, TType, TId> Deserialize<'de> for BulkItemDe<TIndex, TType, TId> 
    where TIndex: Deserialize<'de>,
          TType: Deserialize<'de>,
          TId: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<BulkItemDe<TIndex, TType, TId>, D::Error>
        where D: Deserializer<'de> 
    {
        struct BulkItemDeVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>,
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for BulkItemDeVisitor<TIndex, TType, TId> 
            where TIndex: Deserialize<'de>,
                  TType: Deserialize<'de>,
                  TId: Deserialize<'de>
        {
            type Value = BulkItemDe<TIndex, TType, TId>;

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

        deserializer.deserialize_any(BulkItemDeVisitor { _marker: PhantomData })
    }
}

impl<'de, TIndex, TType, TId> Deserialize<'de> for BulkItems<TIndex, TType, TId> 
    where TIndex: Deserialize<'de>,
          TType: Deserialize<'de>,
          TId: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<BulkItems<TIndex, TType, TId>, D::Error>
        where D: Deserializer<'de>
    {
        struct BulkItemsVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>,
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for BulkItemsVisitor<TIndex, TType, TId> 
            where TIndex: Deserialize<'de>,
                  TType: Deserialize<'de>,
                  TId: Deserialize<'de>
        {
            type Value = BulkItems<TIndex, TType, TId>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<BulkItems<TIndex, TType, TId>, E>
                where E: DeError,
            {
                Ok(BulkItems { ok: vec![], err: vec![] })
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<BulkItems<TIndex, TType, TId>, V::Error>
                where V: SeqAccess<'de>,
            {
                let mut values = BulkItems {
                    ok: Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096)),
                    err: Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 512))
                };

                while let Some(value) = visitor.next_element::<BulkItemDe<TIndex, TType, TId>>()? {
                    match value.into_result() {
                        Ok(item) => values.ok.push(item),
                        Err(item) => values.err.push(item)
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkItemsVisitor { _marker: PhantomData })
    }
}

fn deserialize_bulk_item_errors<'de, D, TIndex, TType, TId>(deserializer: D) -> Result<Vec<BulkItemError<TIndex, TType, TId>>, D::Error>
    where D: Deserializer <'de>,
          TIndex: Deserialize<'de>,
          TType: Deserialize<'de>,
          TId: Deserialize<'de>
{
    struct BulkErrorItemsVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>,
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for BulkErrorItemsVisitor<TIndex, TType, TId> 
            where TIndex: Deserialize<'de>,
                  TType: Deserialize<'de>,
                  TId: Deserialize<'de>
        {
            type Value = Vec<BulkItemError<TIndex, TType, TId>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Vec<BulkItemError<TIndex, TType, TId>>, E>
                where E: DeError,
            {
                Ok(vec![])
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<BulkItemError<TIndex, TType, TId>>, V::Error>
                where V: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096));

                while let Some(value) = visitor.next_element::<BulkItemDe<TIndex, TType, TId>>()? {
                    if let Some(value) = value.into_err() {
                        values.push(value);
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkErrorItemsVisitor { _marker: PhantomData })
}