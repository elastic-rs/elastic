use serde::de::{Deserialize, Deserializer, Visitor, Error as DeError, SeqAccess, MapAccess};
use serde_json::Value;
use common::{DefaultAllocatedField, Shards};

use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

use std::cmp;
use std::fmt;
use std::slice::Iter;
use std::vec::IntoIter;
use std::marker::PhantomData;
use std::error::Error;

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
/// for op in body_as_json.iter() {
///     match op {
///         Ok(ref op) => {
///             // Do something with successful operations
///             println!("ok: {:?}", op)
///         },
///         Err(ref e) => {
///             // Do something with failed operations
///             println!("err: {:?}", op)
///         }
///     }
/// }
/// # }
/// ```
/// 
/// A bulk response can also be connverted into an iterator of `Result`s that combines the successful and failed operations:
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
/// for op in body_as_json {
///     match op {
///         Ok(op) => {
///             // Do something with successful operations
///             println!("ok: {:?}", op)
///         },
///         Err(e) => {
///             // Do something with failed operations
///             println!("err: {:?}", op),
///         }
///     }
/// }
/// # }
/// ```
/// 
/// # Optimising bulk responses
/// 
/// If you're only interested in bulk operations that failed, see [`BulkErrorsResponse`](struct.BulkErrorsResponse.html).
/// It can avoid allocating bulk operation responses that will never be processed.
/// 
/// Both the `BulkResponse` and `BulkErrorsResponse` types have generic parameters for the index, type and id fields.
/// If your bulk operations have a small set of possible values for these fields you can avoid
/// allocating `String`s on the heap by using an alternative type, like an `enum`.
/// 
/// In the example below, we expect all bulk operations to use either a type called `mytypea` or `mytypeb`
/// and an index called `myindex`:
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
/// #[serde(rename_all = "lowercase")]
/// enum Index {
///     MyIndex,
/// }
/// 
/// #[derive(Deserialize)]
/// #[serde(rename_all = "lowercase")]
/// enum Type {
///     MyTypeA,
///     MyTypeB,
/// }
/// 
/// let bulk: BulkResponse<Index, Type> = do_request();
/// # }
/// ```
/// 
/// Also see the [`string-cache`](https://github.com/servo/string-cache) crate as an alternative to using `String`s and `enum`s.
#[derive(Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "TIndex: Deserialize<'de>, TType: Deserialize<'de>, TId: Deserialize<'de>"))]
pub struct BulkResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_items")]
    items: Vec<BulkItemResult<TIndex, TType, TId>>,
}

impl<TIndex, TType, TId> BulkResponse<TIndex, TType, TId> {
    /// Time in milliseconds it took for Elasticsearch to process the request.
    pub fn took(&self) -> u64 {
        self.took
    }

    /// Returns `true` if all bulk operations succeeded.
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    /// Returns `true` if any bulk operations failed.
    pub fn is_err(&self) -> bool {
        self.errors
    }

    /// Iterate through the bulk operations.
    pub fn iter(&self) -> ResultIter<TIndex, TType, TId> {
        ResultIter(self.items.iter())
    }
}

impl<TIndex, TType, TId> IntoIterator for BulkResponse<TIndex, TType, TId> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = ResultIntoIter<TIndex, TType, TId>;

    fn into_iter(self) -> Self::IntoIter {
        ResultIntoIter(self.items.into_iter())
    }
}

/// An iterator for a bulk operation that may have succeeded or failed.
pub struct ResultIntoIter<TIndex, TType, TId>(IntoIter<BulkItemResult<TIndex, TType, TId>>);

impl<TIndex, TType, TId> Iterator for ResultIntoIter<TIndex, TType, TId> {
    type Item = BulkItemResult<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// An iterator for a bulk operation that may have succeeded or failed.
pub struct ResultIter<'a, TIndex: 'a, TType: 'a, TId: 'a>(Iter<'a, BulkItemResult<TIndex, TType, TId>>);

impl<'a, TIndex: 'a, TType: 'a, TId: 'a> Iterator for ResultIter<'a, TIndex, TType, TId> {
    type Item = BulkItemResultBrw<'a, TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.as_ref())
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
#[serde(bound(deserialize = "TIndex: Deserialize<'de>, TType: Deserialize<'de>, TId: Deserialize<'de>"))]
pub struct BulkErrorsResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_item_errors")]
    items: Vec<BulkItemError<TIndex, TType, TId>>,
}

impl<TIndex, TType, TId> IntoIterator for BulkErrorsResponse<TIndex, TType, TId> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = ErrorIntoIter<TIndex, TType, TId>;

    fn into_iter(self) -> Self::IntoIter {
        ErrorIntoIter(self.items.into_iter())
    }
}

pub struct ErrorIntoIter<TIndex, TType, TId>(IntoIter<BulkItemError<TIndex, TType, TId>>);

impl<TIndex, TType, TId> Iterator for ErrorIntoIter<TIndex, TType, TId> {
    type Item = BulkItemError<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct ErrorIter<'a, TIndex: 'a, TType: 'a, TId: 'a>(Iter<'a, BulkItemError<TIndex, TType, TId>>);

impl<'a, TIndex: 'a, TType: 'a, TId: 'a> Iterator for ErrorIter<'a, TIndex, TType, TId> {
    type Item = &'a BulkItemError<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<TIndex, TType, TId> BulkErrorsResponse<TIndex, TType, TId> {
    /// Time in milliseconds it took for Elasticsearch to process the request.
    pub fn took(&self) -> u64 {
        self.took
    }

    /// Returns `true` if all bulk operations succeeded.
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    /// Returns `true` if any bulk operations failed.
    pub fn is_err(&self) -> bool {
        self.errors
    }

    /// Iterate through the bulk operation errors.
    pub fn iter(&self) -> ErrorIter<TIndex, TType, TId> {
        ErrorIter(self.items.iter())
    }
}

type BulkItemResult<TIndex, TType, TId> = Result<BulkItem<TIndex, TType, TId>, BulkItemError<TIndex, TType, TId>>;
type BulkItemResultBrw<'a, TIndex, TType, TId> = Result<&'a BulkItem<TIndex, TType, TId>, &'a BulkItemError<TIndex, TType, TId>>;

/// A successful bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItem<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    action: BulkAction,
    index: TIndex,
    ty: TType,
    id: TId,
    version: Option<u32>,
    shards: Option<Shards>,
    created: Option<bool>,
    found: Option<bool>,
}

impl<TIndex, TType, TId> BulkItem<TIndex, TType, TId> {
    /// The bulk action for this operation.
    pub fn action(&self) -> BulkAction {
        self.action
    }

    /// The document version after this operation.
    pub fn version(&self) -> Option<u32> {
        self.version.clone()
    }

    /// Whether or not this operation created the document.
    /// 
    /// `created` will only be `true` if the action is `Index` and the document didn't already exist.
    pub fn created(&self) -> bool {
        self.created.clone().unwrap_or(false)
    }

    /// Whether or not this operation found the document.
    /// 
    /// `found` will only be `true` if the action is `Delete` and the document did already exist.
    pub fn found(&self) -> bool {
        self.found.clone().unwrap_or(false)
    }
    
    /// The index for this operation.
    pub fn index(&self) -> &TIndex {
        &self.index
    }
    
    /// The document type for this operation.
    pub fn ty(&self) -> &TType {
        &self.ty
    }
    
    /// The document id for this operation.
    pub fn id(&self) -> &TId {
        &self.id
    }
}

/// A failed bulk response item.
#[derive(Debug, Clone)]
pub struct BulkItemError<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    action: BulkAction,
    index: TIndex,
    ty: TType,
    id: TId,
    err: BulkError
}

impl<TIndex, TType, TId> BulkItemError<TIndex, TType, TId> {
    /// The bulk action for this operation.
    pub fn action(&self) -> BulkAction {
        self.action
    }

    /// The index for this operation.
    pub fn index(&self) -> &TIndex {
        &self.index
    }
    
    /// The document type for this operation.
    pub fn ty(&self) -> &TType {
        &self.ty
    }
    
    /// The document id for this operation.
    pub fn id(&self) -> &TId {
        &self.id
    }
}

impl<TIndex, TType, TId> fmt::Display for BulkItemError<TIndex, TType, TId>
    where TIndex: fmt::Display + fmt::Debug,
          TType: fmt::Display + fmt::Debug,
          TId: fmt::Display + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bulk operation failed. Details: index: {}, type: {}, id: {}, inner error: {}", self.index, self.ty, self.id, self.err)
    }
}

impl<TIndex, TType, TId> Error for BulkItemError<TIndex, TType, TId> 
    where TIndex: fmt::Display + fmt::Debug,
          TType: fmt::Display + fmt::Debug,
          TId: fmt::Display + fmt::Debug
{
    fn description(&self) -> &str {
        "bulk operation failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// The bulk action being performed.
#[derive(Deserialize, Debug, Clone, Copy)]
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
    index: TIndex,
    #[serde(rename = "_type")]
    ty: TType,
    #[serde(rename = "_id")]
    id: TId,
    #[serde(rename = "_version")]
    version: Option<u32>,
    #[serde(rename = "_shards")]
    shards: Option<Shards>,
    created: Option<bool>,
    found: Option<bool>,
    status: u16,
    error: Option<BulkError>,
}

impl<'de, TIndex, TType, TId> BulkItemDe<TIndex, TType, TId>
    where TIndex: Deserialize<'de>,
          TType: Deserialize<'de>,
          TId: Deserialize<'de>,
{
    fn into_result(self) -> BulkItemResult<TIndex, TType, TId> {
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
          TId: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<BulkItemDe<TIndex, TType, TId>, D::Error>
        where D: Deserializer<'de> 
    {
        struct BulkItemDeVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for BulkItemDeVisitor<TIndex, TType, TId>
            where TIndex: Deserialize<'de>,
                  TType: Deserialize<'de>,
                  TId: Deserialize<'de>,
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

fn deserialize_bulk_items<'de, D, TIndex, TType, TId>(deserializer: D) -> Result<Vec<BulkItemResult<TIndex, TType, TId>>, D::Error>
    where D: Deserializer <'de>,
          TIndex: Deserialize<'de>,
          TType: Deserialize<'de>,
          TId: Deserialize<'de>
{
    struct BulkItemsVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>,
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for BulkItemsVisitor<TIndex, TType, TId> 
            where TIndex: Deserialize<'de>,
                  TType: Deserialize<'de>,
                  TId: Deserialize<'de>
        {
            type Value = Vec<BulkItemResult<TIndex, TType, TId>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Vec<BulkItemResult<TIndex, TType, TId>>, E>
                where E: DeError,
            {
                Ok(vec![])
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<BulkItemResult<TIndex, TType, TId>>, V::Error>
                where V: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096));

                while let Some(value) = visitor.next_element::<BulkItemDe<_, _, _>>()? {
                    values.push(value.into_result());
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkItemsVisitor { _marker: PhantomData })
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

                while let Some(value) = visitor.next_element::<BulkItemDe<_, _, _>>()? {
                    if let Some(value) = value.into_result().err() {
                        values.push(value);
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_any(BulkErrorItemsVisitor { _marker: PhantomData })
}