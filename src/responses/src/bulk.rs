/*!
Response types for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).
*/

use common::{DefaultAllocatedField, DocumentResult, Shards};
use serde::de::{Deserialize, Deserializer, Error as DeError, MapAccess, SeqAccess, Visitor};
use serde_json::Value;

use parsing::IsOkOnSuccess;

use std::cmp;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::slice::Iter;
use std::vec::IntoIter;

type BulkError = Value;

/**
Response for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).

Individual bulk items are a `Result` of [`OkItem`](struct.OkItem.html) or [`ErrorItem`](struct.ErrorItem.html) and can be iterated over.
Any individual bulk item may be an `Err(ErrorItem)`, so it's important to check them.
The `is_ok` and `is_err` methods on `BulkResponse` make it easier to assert there are no errors.

# Examples

Send a bulk request and iterate through the results:

```no_run
# extern crate elastic_responses;
# use elastic_responses::*;
# fn do_request() -> BulkResponse { unimplemented!() }
# fn main() {
let response: BulkResponse = do_request();

// Check if the response contains any errors
if response.is_err() {
    println!("some bulk items failed");
}

// Iterate through all items
for item in response {
    match item {
        Ok(item) => {
            // Do something with the `OkItem`s
            println!("ok: {:?}", item)
        },
        Err(item) => {
            // Do something with the `ErrorItem`s
            println!("err: {:?}", item)
        }
    }
}
# }
```

Use `iter` to iterate over individual items without taking ownership of them:

```no_run
# extern crate elastic_responses;
# use elastic_responses::*;
# fn do_request() -> BulkResponse { unimplemented!() }
# fn main() {
let response: BulkResponse = do_request();

// Do something with successful items for index `myindex`
let item_iter = response.iter()
                        .filter_map(Result::ok)
                        .filter(|o| o.index() == "myindex");

for item in item_iter {
    // Do something with the `OkItem`s
    println!("ok: {:?}", item);
}
# }
```

# Optimising bulk responses

If you're only interested in bulk items that failed, see [`BulkErrorsResponse`](struct.BulkErrorsResponse.html).
It can avoid allocating bulk item responses that will never be processed.

Both the `BulkResponse` and `BulkErrorsResponse` types have generic parameters for the index, type and id fields.
If your bulk items have a small set of possible values for these fields you can avoid
allocating `String`s on the heap by using an alternative type, like an `enum`.

In the example below, we expect all bulk items to use either a type called `mytypea` or `mytypeb`
and an index called `myindex`:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# extern crate serde_json;
# extern crate elastic_responses;
# use elastic_responses::*;
# fn main() {
# fn do_request() -> BulkResponse<Index, Type> { unimplemented!() }
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Index {
    MyIndex,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    MyTypeA,
    MyTypeB,
}

let bulk: BulkResponse<Index, Type> = do_request();
# }
```

Other crates that can avoid allocating strings:

- [`string-cache`](https://github.com/servo/string-cache) crate for interning string values
- [`inlinable-string`](https://github.com/fitzgen/inlinable_string) crate for storing small strings on the stack

# Taking `BulkResonse` as an argument

The `BulkResponse` type has three default generic parameters for the index, type and id fields.
If you need to accept a `BulkResponse` as a function argument, you should specify these generics.
Otherwise the function will only accept a default `BulkResponse`:

```
# use elastic_responses::*;
// Do: Supports any BulkResponse
fn takes_any_response<TIndex, TType, TId>(res: BulkResponse<TIndex, TType, TId>) {

}

// Don't: Only supports default BulkResponse
fn takes_default_response(res: BulkResponse) {

}
```
*/
#[derive(Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "TIndex: Deserialize<'de>, TType: Deserialize<'de>, TId: Deserialize<'de>"))]
pub struct BulkResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_items")]
    items: Vec<ItemResult<TIndex, TType, TId>>,
}

impl<TIndex, TType, TId> BulkResponse<TIndex, TType, TId> {
    /** Time in milliseconds it took for Elasticsearch to process the request. */
    pub fn took(&self) -> u64 {
        self.took
    }

    /** Returns `true` if all bulk items succeeded. */
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    /** Returns `true` if any bulk items failed. */
    pub fn is_err(&self) -> bool {
        self.errors
    }

    /**
    Iterate through the bulk items.

    The items in this iterator are a standard `Result` where `Ok` means the item succeeded
    and `Err` means it failed.

    To move out of the items in a `BulkResponse` instead of borrowing them, call `into_iter`.

    # Examples

    Iterate through the individual items in a `BulkResponse`:

    ```no_run
    # extern crate elastic_responses;
    # use elastic_responses::*;
    # fn do_request() -> BulkResponse { unimplemented!() }
    # fn main() {
    let response: BulkResponse = do_request();

    // Iterate through all items
    for item in response.iter() {
        match item {
            Ok(ref item) => {
                // Do something with the `OkItem`s
                println!("ok: {:?}", item)
            },
            Err(ref item) => {
                // Do something with the `ErrorItem`s
                println!("err: {:?}", item)
            }
        }
    }
    # }
    ```
    */
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

/** An owning iterator for a bulk item that may have succeeded or failed. */
pub struct ResultIntoIter<TIndex, TType, TId>(IntoIter<ItemResult<TIndex, TType, TId>>);

impl<TIndex, TType, TId> Iterator for ResultIntoIter<TIndex, TType, TId> {
    type Item = ItemResult<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/** A borrowing iterator for a bulk item that may have succeeded or failed. */
pub struct ResultIter<'a, TIndex: 'a, TType: 'a, TId: 'a>(Iter<'a, ItemResult<TIndex, TType, TId>>);

impl<'a, TIndex: 'a, TType: 'a, TId: 'a> Iterator for ResultIter<'a, TIndex, TType, TId> {
    type Item = ItemResultBrw<'a, TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| item.as_ref())
    }
}

/**
Response for a [bulk request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html).

This type only accumulates bulk items that failed.
It can be more efficient if you only care about errors.
Individual bulk items are [`ErrorItem`](struct.ErrorItem.html) and can be iterated over.

# Examples

Send a bulk request and iterate through the errors:

```no_run
# extern crate elastic_responses;
# use elastic_responses::*;
# use elastic_responses::bulk::Action;
# fn do_request() -> BulkErrorsResponse { unimplemented!() }
# fn main() {
let response: BulkErrorsResponse = do_request();

// Do something with failed items
for item in response {
    match item.action() {
        Action::Delete => (), // Ignore failed deletes
        _ => println!("err: {:?}", item)
    }
}
# }
```

Use `iter` to iterate over individual errors without taking ownership of them:

```no_run
# extern crate elastic_responses;
# use elastic_responses::*;
# fn do_request() -> BulkErrorsResponse { unimplemented!() }
# fn main() {
let response: BulkErrorsResponse = do_request();

// Do something with errors for index `myindex`
let item_iter = response.iter()
                        .filter(|o| o.index() == "myindex");

for item in item_iter {
    println!("err: {:?}", item);
}
# }
```

# Taking `BulkErrorsResponse` as an argument

The `BulkErrorsResponse` type has three default generic parameters for the index, type and id fields.
If you need to accept a `BulkErrorsResponse` as a function argument, you should specify these generics.
Otherwise the function will only accept a default `BulkErrorsResponse`:

```
# use elastic_responses::*;
// Do: Supports any BulkErrorsResponse
fn takes_any_response<TIndex, TType, TId>(res: BulkErrorsResponse<TIndex, TType, TId>) {

}

// Don't: Only supports default BulkErrorsResponse
fn takes_default_response(res: BulkErrorsResponse) {

}
```
*/
#[derive(Deserialize, Debug, Clone)]
#[serde(bound(deserialize = "TIndex: Deserialize<'de>, TType: Deserialize<'de>, TId: Deserialize<'de>"))]
pub struct BulkErrorsResponse<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    took: u64,
    errors: bool,
    #[serde(deserialize_with = "deserialize_bulk_item_errors")]
    items: Vec<ErrorItem<TIndex, TType, TId>>,
}

impl<TIndex, TType, TId> IntoIterator for BulkErrorsResponse<TIndex, TType, TId> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = ErrorIntoIter<TIndex, TType, TId>;

    fn into_iter(self) -> Self::IntoIter {
        ErrorIntoIter(self.items.into_iter())
    }
}

/** An owning iterator for a bulk item that failed. */
pub struct ErrorIntoIter<TIndex, TType, TId>(IntoIter<ErrorItem<TIndex, TType, TId>>);

impl<TIndex, TType, TId> Iterator for ErrorIntoIter<TIndex, TType, TId> {
    type Item = ErrorItem<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/** A borrowing iterator for a bulk item that failed. */
pub struct ErrorIter<'a, TIndex: 'a, TType: 'a, TId: 'a>(Iter<'a, ErrorItem<TIndex, TType, TId>>);

impl<'a, TIndex: 'a, TType: 'a, TId: 'a> Iterator for ErrorIter<'a, TIndex, TType, TId> {
    type Item = &'a ErrorItem<TIndex, TType, TId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<TIndex, TType, TId> BulkErrorsResponse<TIndex, TType, TId> {
    /** Time in milliseconds it took for Elasticsearch to process the request. */
    pub fn took(&self) -> u64 {
        self.took
    }

    /** Returns `true` if all bulk items succeeded. */
    pub fn is_ok(&self) -> bool {
        !self.errors
    }

    /** Returns `true` if any bulk itemss failed. */
    pub fn is_err(&self) -> bool {
        self.errors
    }

    /**
    Iterate through the bulk item errors.

    Items in this iterator all all errors that occurred while handling the bulk request.

    # Examples

    Iterate through the individual items in a `BulkErrorsResponse`:

    ```no_run
    # extern crate elastic_responses;
    # use elastic_responses::*;
    # fn do_request() -> BulkErrorsResponse { unimplemented!() }
    # fn main() {
    let response: BulkErrorsResponse = do_request();

    // Iterate through all items
    for item in response.iter() {
        // Do something with failed items
        println!("err: {:?}", item)
    }
    # }
    ```
    */
    pub fn iter(&self) -> ErrorIter<TIndex, TType, TId> {
        ErrorIter(self.items.iter())
    }
}

type ItemResult<TIndex, TType, TId> = Result<OkItem<TIndex, TType, TId>, ErrorItem<TIndex, TType, TId>>;
type ItemResultBrw<'a, TIndex, TType, TId> = Result<&'a OkItem<TIndex, TType, TId>, &'a ErrorItem<TIndex, TType, TId>>;

/** A successful bulk response item. */
#[derive(Debug, Clone)]
pub struct OkItem<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    action: Action,
    index: TIndex,
    ty: TType,
    id: TId,
    version: Option<u32>,
    shards: Option<Shards>,
    result: Option<DocumentResult>,
}

impl<TIndex, TType, TId> OkItem<TIndex, TType, TId> {
    /** The bulk action for this item. */
    pub fn action(&self) -> Action {
        self.action
    }

    /** The document version after this item. */
    pub fn version(&self) -> Option<u32> {
        self.version.clone()
    }

    /**
    Whether or not this item created the document.

    `created` will only be `true` if the action is `Index` and the document didn't already exist.
    */
    pub fn created(&self) -> bool {
        match self.result {
            Some(DocumentResult::Created) => true,
            _ => false,
        }
    }

    /**
    Whether or not this item deleted the document.

    `deleted` will only be `true` if the action is `Delete` and the document existed
    */
    pub fn deleted(&self) -> bool {
        match self.result {
            Some(DocumentResult::Deleted) => true,
            _ => false,
        }
    }

    /** The index for this item. */
    pub fn index(&self) -> &TIndex {
        &self.index
    }

    /** The document type for this item. */
    pub fn ty(&self) -> &TType {
        &self.ty
    }

    /** The document id for this item. */
    pub fn id(&self) -> &TId {
        &self.id
    }
}

/** A failed bulk response item. */
#[derive(Debug, Clone)]
pub struct ErrorItem<TIndex = DefaultAllocatedField, TType = DefaultAllocatedField, TId = DefaultAllocatedField> {
    action: Action,
    index: TIndex,
    ty: TType,
    id: TId,
    err: BulkError,
}

impl<TIndex, TType, TId> ErrorItem<TIndex, TType, TId> {
    /** The bulk action for this item. */
    pub fn action(&self) -> Action {
        self.action
    }

    /** The index for this item. */
    pub fn index(&self) -> &TIndex {
        &self.index
    }

    /** The document type for this item. */
    pub fn ty(&self) -> &TType {
        &self.ty
    }

    /** The document id for this item. */
    pub fn id(&self) -> &TId {
        &self.id
    }
}

impl<TIndex, TType, TId> fmt::Display for ErrorItem<TIndex, TType, TId>
where
    TIndex: fmt::Display + fmt::Debug,
    TType: fmt::Display + fmt::Debug,
    TId: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bulk item failed. Details: index: {}, type: {}, id: {}, inner error: {}", self.index, self.ty, self.id, self.err)
    }
}

impl<TIndex, TType, TId> Error for ErrorItem<TIndex, TType, TId>
where
    TIndex: fmt::Display + fmt::Debug,
    TType: fmt::Display + fmt::Debug,
    TId: fmt::Display + fmt::Debug,
{
    fn description(&self) -> &str {
        "bulk item failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/** The bulk action being performed. */
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

impl<TIndex, TType, TId> IsOkOnSuccess for BulkResponse<TIndex, TType, TId> {}

impl<TIndex, TType, TId> IsOkOnSuccess for BulkErrorsResponse<TIndex, TType, TId> {}

// Deserialisation

struct ItemDe<TIndex, TType, TId> {
    action: Action,
    inner: ItemDeInner<TIndex, TType, TId>,
}

#[derive(Deserialize, Debug, Clone)]
struct ItemDeInner<TIndex, TType, TId> {
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
    result: Option<DocumentResult>,
    status: u16,
    error: Option<BulkError>,
}

impl<'de, TIndex, TType, TId> ItemDe<TIndex, TType, TId>
where
    TIndex: Deserialize<'de>,
    TType: Deserialize<'de>,
    TId: Deserialize<'de>,
{
    fn into_err(self) -> Option<ErrorItem<TIndex, TType, TId>> {
        match self.inner.error {
            Some(err) => Some(ErrorItem {
                action: self.action,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                err: err,
            }),
            None => None,
        }
    }

    fn into_result(self) -> ItemResult<TIndex, TType, TId> {
        if self.inner.error.is_some() {
            Err(self.into_err().expect("expected an error"))
        } else {
            Ok(OkItem {
                action: self.action,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                version: self.inner.version,
                shards: self.inner.shards,
                result: self.inner.result,
            })
        }
    }
}

impl<'de, TIndex, TType, TId> Deserialize<'de> for ItemDe<TIndex, TType, TId>
where
    TIndex: Deserialize<'de>,
    TType: Deserialize<'de>,
    TId: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<ItemDe<TIndex, TType, TId>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemDeVisitor<TIndex, TType, TId> {
            _marker: PhantomData<(TIndex, TType, TId)>,
        }

        impl<'de, TIndex, TType, TId> Visitor<'de> for ItemDeVisitor<TIndex, TType, TId>
        where
            TIndex: Deserialize<'de>,
            TType: Deserialize<'de>,
            TId: Deserialize<'de>,
        {
            type Value = ItemDe<TIndex, TType, TId>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a bulk item")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let (action, inner) = visitor.next_entry()?.ok_or(V::Error::custom("expected at least one field"))?;

                let result = ItemDe { action: action, inner: inner };

                Ok(result)
            }
        }

        deserializer.deserialize_any(ItemDeVisitor { _marker: PhantomData })
    }
}

fn deserialize_bulk_items<'de, D, TIndex, TType, TId>(deserializer: D) -> Result<Vec<ItemResult<TIndex, TType, TId>>, D::Error>
where
    D: Deserializer<'de>,
    TIndex: Deserialize<'de>,
    TType: Deserialize<'de>,
    TId: Deserialize<'de>,
{
    struct OkItemsVisitor<TIndex, TType, TId> {
        _marker: PhantomData<(TIndex, TType, TId)>,
    }

    impl<'de, TIndex, TType, TId> Visitor<'de> for OkItemsVisitor<TIndex, TType, TId>
    where
        TIndex: Deserialize<'de>,
        TType: Deserialize<'de>,
        TId: Deserialize<'de>,
    {
        type Value = Vec<ItemResult<TIndex, TType, TId>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Vec<ItemResult<TIndex, TType, TId>>, E>
        where
            E: DeError,
        {
            Ok(vec![])
        }

        #[inline]
        fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<ItemResult<TIndex, TType, TId>>, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096));

            while let Some(value) = visitor.next_element::<ItemDe<_, _, _>>()? {
                values.push(value.into_result());
            }

            Ok(values)
        }
    }

    deserializer.deserialize_any(OkItemsVisitor { _marker: PhantomData })
}

fn deserialize_bulk_item_errors<'de, D, TIndex, TType, TId>(deserializer: D) -> Result<Vec<ErrorItem<TIndex, TType, TId>>, D::Error>
where
    D: Deserializer<'de>,
    TIndex: Deserialize<'de>,
    TType: Deserialize<'de>,
    TId: Deserialize<'de>,
{
    struct BulkErrorItemsVisitor<TIndex, TType, TId> {
        _marker: PhantomData<(TIndex, TType, TId)>,
    }

    impl<'de, TIndex, TType, TId> Visitor<'de> for BulkErrorItemsVisitor<TIndex, TType, TId>
    where
        TIndex: Deserialize<'de>,
        TType: Deserialize<'de>,
        TId: Deserialize<'de>,
    {
        type Value = Vec<ErrorItem<TIndex, TType, TId>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Vec<ErrorItem<TIndex, TType, TId>>, E>
        where
            E: DeError,
        {
            Ok(vec![])
        }

        #[inline]
        fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<ErrorItem<TIndex, TType, TId>>, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::with_capacity(cmp::min(visitor.size_hint().unwrap_or(0), 4096));

            while let Some(value) = visitor.next_element::<ItemDe<_, _, _>>()? {
                if let Some(value) = value.into_err() {
                    values.push(value);
                }
            }

            Ok(values)
        }
    }

    deserializer.deserialize_any(BulkErrorItemsVisitor { _marker: PhantomData })
}
