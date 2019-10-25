/*!
Builders for [bulk requests][docs-bulk].

[docs-bulk]: https://www.elastic.co/guide/en/elasticsearch/reference/current/bulk.html
*/

use std::{
    error::Error as StdError,
    fmt,
    marker::PhantomData,
    time::Duration,
};

use futures::Future;
use serde::{
    de::DeserializeOwned,
    ser::Serialize,
};

use crate::{
    client::{
        requests::{
            raw::RawRequestInner,
            Pending as BasePending,
            RequestBuilder,
        },
        responses::{
            BulkErrorsResponse,
            BulkResponse,
        },
        Client,
        RequestParams,
    },
    endpoints::BulkRequest,
    error::{
        self,
        Error,
    },
    http::{
        receiver::IsOk,
        sender::{
            AsyncSender,
            Sender,
            SyncSender,
        },
        AsyncBody,
        SyncBody,
    },
    params::{
        Index,
        Type,
    },
    types::document::DEFAULT_DOC_TYPE,
};

/**
A [bulk request][docs-bulk] builder that can be configured before sending.

Call [`Client.bulk`][Client.bulk] to get a `BulkRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

Call [`Client.bulk_stream`][Client.bulk_stream] to get a `BulkRequestBuilder` that can be used to stream bulk operations asynchronously.

[docs-bulk]: https://www.elastic.co/guide/en/elasticsearch/reference/current/bulk.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.bulk]: ../../struct.Client.html#bulk-request
[Client.bulk_stream]: ../../struct.Client.html#bulk-stream-request
*/
pub type BulkRequestBuilder<TSender, TBody, TResponse> =
    RequestBuilder<TSender, BulkRequestInner<TBody, TResponse>>;

mod operation;
mod stream;

pub use self::{
    operation::*,
    stream::*,
};

#[doc(hidden)]
pub struct BulkRequestInner<TBody, TResponse> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: WrappedBody<TBody>,
    _marker: PhantomData<TResponse>,
}

/**
# Bulk request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /**
    Create a [`BulkRequestBuilder`][BulkRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Send a bulk request to index some documents:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    # }
    # let client = SyncClientBuilder::new().build()?;
    let ops = (0..1000)
        .into_iter()
        .map(|i| bulk::<MyType>().index(MyType {
                id: i.to_string(),
                title: "some string value".to_owned()
            })
            .id(i));

    let response = client.bulk()
                         .index("myindex")
                         .ty(MyType::static_ty())
                         .extend(ops)
                         .send()?;

    for op in response {
        match op {
            Ok(op) => println!("ok: {:?}", op),
            Err(op) => println!("err: {:?}", op),
        }
    }
    # Ok(())
    # }
    ```

    [BulkRequestBuilder]: requests/bulk/type.BulkRequestBuilder.html
    [builder-methods]: requests/bulk/type.BulkRequestBuilder.html#builder-methods
    [send-sync]: requests/bulk/type.BulkRequestBuilder.html#send-synchronously
    [send-async]: requests/bulk/type.BulkRequestBuilder.html#send-asynchronously
    [types-mod]: ../../types/index.html
    [documents-mod]: ../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn bulk(&self) -> BulkRequestBuilder<TSender, Vec<u8>, BulkResponse> {
        RequestBuilder::initial(
            self.clone(),
            BulkRequestInner {
                index: None,
                ty: None,
                body: WrappedBody::new(Vec::new()),
                _marker: PhantomData,
            },
        )
    }
}

/**
# Bulk stream request
*/
impl Client<AsyncSender> {
    /**
    Create a [`BulkRequestBuilder`][BulkRequestBuilder] with this `Client` that can be configured before sending.

    This method can configure a channel that individual bulk operations can be sent to.
    The operations will be batched and debounced in the backgroun rather than being sent immediately.

    For more details, see:

    - [builder methods][builder-methods]
    - [stream builder methods][stream-builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Stream a bulk request to index some documents:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use std::time::Duration;
    # use futures::{Future, Stream, Sink};
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    # }
    # let client = AsyncClientBuilder::new().build()?;
    let (bulk_stream, bulk_responses) = client.bulk_stream()
        .index("bulk_idx")
        .ty(MyType::static_ty())
        .timeout(Duration::from_secs(5))
        .body_size_bytes(1024)
        .build();

    let ops = (0..1000)
        .into_iter()
        .map(|i| bulk::<MyType>().index(MyType {
                id: i.to_string(),
                title: "some string value".into()
            })
            .id(i));

    let req_future = bulk_stream.send_all(futures::stream::iter_ok(ops));

    let res_future = bulk_responses.for_each(|bulk| {
        println!("response:");
        for op in bulk {
            match op {
                Ok(op) => println!("  ok: {:?}", op),
                Err(op) => println!("  err: {:?}", op),
            }
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [BulkRequestBuilder]: requests/bulk/type.BulkRequestBuilder.html
    [builder-methods]: requests/bulk/type.BulkRequestBuilder.html#stream-builder-methods\
    [send-sync]: requests/bulk/type.BulkRequestBuilder.html#send-synchronously
    [send-async]: requests/bulk/type.BulkRequestBuilder.html#send-asynchronously
    [types-mod]: ../../types/index.html
    [documents-mod]: ../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn bulk_stream<TDocument>(
        &self,
    ) -> BulkRequestBuilder<AsyncSender, Streamed<TDocument>, BulkResponse> {
        RequestBuilder::initial(
            self.clone(),
            BulkRequestInner {
                index: None,
                ty: None,
                body: WrappedBody::new(Streamed::new()),
                _marker: PhantomData,
            },
        )
    }
}

/**
# Builder methods

Configure a `BulkRequestBuilder` before sending it.
*/
impl<TSender, TBody, TResponse> BulkRequestBuilder<TSender, TBody, TResponse>
where
    TSender: Sender,
{
    /**
    Set the default type for the bulk request.

    If an operation doesn't specify a type, then it will default to the supplied value here.

    # Deferred errors

    Calling `ty` without also calling `index` will result in an error when sending the request.
    */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = Some(ty.into());
        self
    }

    /**
    Set the default index for the bulk request.

    If an operation doesn't specify an index, then it will default to the supplied value here.
    */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = Some(index.into());
        self
    }

    /**
    Set the type used to deserialize the index field on the response.

    Sometimes a bulk response will use the same index value many times.
    To avoid allocating a lot of individual strings, the type used to deserialize the field can be changed.
    `string_cache::DefaultAtom` or a custom `enum` can be effective ways to reduce allocations in large bulk responses.
    */
    pub fn response_index<I>(self) -> BulkRequestBuilder<TSender, TBody, TResponse::WithNewIndex>
    where
        TResponse: ChangeIndex<I>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            BulkRequestInner {
                index: self.inner.index,
                ty: self.inner.ty,
                body: self.inner.body,
                _marker: PhantomData,
            },
        )
    }

    /**
    Set the type used to deserialize the type field on the response.

    Sometimes a bulk response will use the same type value many times.
    To avoid allocating a lot of individual strings, the type used to deserialize the field can be changed.
    `string_cache::DefaultAtom` or a custom `enum` can be effective ways to reduce allocations in large bulk responses.
    */
    pub fn response_ty<I>(self) -> BulkRequestBuilder<TSender, TBody, TResponse::WithNewType>
    where
        TResponse: ChangeType<I>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            BulkRequestInner {
                index: self.inner.index,
                ty: self.inner.ty,
                body: self.inner.body,
                _marker: PhantomData,
            },
        )
    }

    /**
    Set the type used to deserialize the id field on the response.

    It's less likely that id fields in bulk responses will be repeated, but they're probably short.
    To avoid allocating a lot of individual strings, the type used to deserialize the field can be changed.
    `inlinable_string::InlinableString` can be an effective way to recude allocation in large bulk responses.
    */
    pub fn response_id<I>(self) -> BulkRequestBuilder<TSender, TBody, TResponse::WithNewId>
    where
        TResponse: ChangeId<I>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            BulkRequestInner {
                index: self.inner.index,
                ty: self.inner.ty,
                body: self.inner.body,
                _marker: PhantomData,
            },
        )
    }
}

impl<TSender, TBody, TIndex, TType, TId>
    BulkRequestBuilder<TSender, TBody, BulkResponse<TIndex, TType, TId>>
where
    TSender: Sender,
{
    /**
    Only deserialize failed bulk operations in the response.

    Elasticsearch returns a response that's proportional in size to the number of operations in the request.
    If you only care about failures then it can be more efficient to ignore the common case where operations succeed.
    */
    pub fn errors_only(
        self,
    ) -> BulkRequestBuilder<TSender, TBody, BulkErrorsResponse<TIndex, TType, TId>> {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            BulkRequestInner {
                index: self.inner.index,
                ty: self.inner.ty,
                body: self.inner.body,
                _marker: PhantomData,
            },
        )
    }
}

impl<TSender, TBody, TResponse> BulkRequestBuilder<TSender, TBody, TResponse>
where
    TSender: Sender,
    TBody: BulkBody,
{
    fn push_internal<TDocument, TOperation>(&mut self, op: TOperation)
    where
        TOperation: Into<BulkOperation<TDocument>>,
        TDocument: Serialize,
    {
        self.inner.body.with_inner_mut(|b| b.push(op.into()));
    }

    /**
    Push an operation onto the bulk request.

    # Deferred errors

    If the document can't be serialized then sending the request will return an error.
    */
    pub fn push<TDocument, TOperation>(mut self, op: TOperation) -> Self
    where
        TOperation: Into<BulkOperation<TDocument>>,
        TDocument: Serialize,
    {
        self.push_internal(op);
        self
    }

    /**
    Push a collection of operations onto the bulk request.

    # Deferred errors

    If any documents can't be serialized then sending the request will return an error.
    */
    pub fn extend<TIter, TDocument>(mut self, iter: TIter) -> Self
    where
        TIter: IntoIterator<Item = BulkOperation<TDocument>>,
        TDocument: Serialize,
    {
        for op in iter.into_iter() {
            self.push_internal(op);
        }
        self
    }
}

impl<TSender, TBody, TDocument, TResponse> Extend<BulkOperation<TDocument>>
    for BulkRequestBuilder<TSender, TBody, TResponse>
where
    TSender: Sender,
    TBody: BulkBody,
    TDocument: Serialize,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = BulkOperation<TDocument>>,
    {
        for op in iter.into_iter() {
            self.push_internal(op);
        }
    }
}

/**
# Stream builder methods

Configure a `SearchRequestBuilder` before sending it.
*/
impl<TDocument, TResponse> BulkRequestBuilder<AsyncSender, Streamed<TDocument>, TResponse> {
    /**
    Specify a timeout for filling up the request buffer.

    This parameter can be used to control the miminum frequency of bulk requests.
    If the timeout expires before the buffer is full then a bulk request will be sent with whatever data was written.
    The timeout isn't restarted when operations are pushed.
    */
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner.body.with_inner_mut(|s| {
            s.timeout = timeout;
            Ok(())
        });

        self
    }

    /**
    Specify a maximum request size in bytes.

    This parameter can be used to control the maximum size of a single bulk request emitted.
    Operations will be appended to the request until the `body_size` is reached.
    */
    pub fn body_size_bytes(mut self, body_size: usize) -> Self {
        self.inner.body.with_inner_mut(|s| {
            s.body_size = body_size;
            Ok(())
        });

        self
    }

    /**
    Create a channel for streaming bulk operations.

    This will return a channel with a [`BulkSender`] and [`BulkReceiver`] pair.
    Push operations into the sender.
    Once an internal buffer is full, or a timeout expires then the bulk request will be sent.
    Responses can be pulled by the receiver.

    # Examples

    > TODO
    */
    pub fn build(self) -> (BulkSender<TDocument, TResponse>, BulkReceiver<TResponse>) {
        let body = self
            .inner
            .body
            .try_into_inner()
            .expect("building a stream should be infallible");

        let body_size = body.body_size;
        let duration = body.timeout;

        let params = self.params_builder.into_value(RequestParams::default);
        let body = SenderBody::new(body_size);
        let timeout = Timeout::new(duration);
        let req_template =
            SenderRequestTemplate::new(self.client, params, self.inner.index, self.inner.ty);

        BulkSender::new(req_template, timeout, body)
    }
}

impl<TBody, TResponse> BulkRequestInner<TBody, TResponse>
where
    TBody: BulkBody,
{
    fn into_request(self) -> Result<BulkRequest<'static, TBody>, Error> {
        let body = self.body.try_into_inner()?;

        match (self.index, self.ty) {
            (Some(index), ty) => match ty {
                None => Ok(BulkRequest::for_index(index, body)),
                Some(ref ty) if &ty[..] == DEFAULT_DOC_TYPE => {
                    Ok(BulkRequest::for_index(index, body))
                }
                Some(ty) => Ok(BulkRequest::for_index_ty(index, ty, body)),
            },
            (None, None) => Ok(BulkRequest::new(body)),
            (None, Some(_)) => Err(error::request(BulkRequestError(
                "missing `index` parameter".to_owned(),
            ))),
        }
    }
}

/**
# Send synchronously
*/
impl<TBody, TResponse> BulkRequestBuilder<SyncSender, TBody, TResponse>
where
    TBody: Into<SyncBody> + BulkBody + Send + 'static,
    TResponse: DeserializeOwned + IsOk + Send + 'static,
{
    /**
    Send a `BulkRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Send a bulk request to index some documents:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    # }
    # let client = SyncClientBuilder::new().build()?;
    let ops = (0..1000)
        .into_iter()
        .map(|i| bulk::<MyType>().index(MyType {
                id: i.to_string(),
                title: "some string value".into()
            })
            .id(i));

    let response = client.bulk()
                         .index("myindex")
                         .ty(MyType::static_ty())
                         .extend(ops)
                         .send()?;

    for op in response {
        match op {
            Ok(op) => println!("ok: {:?}", op),
            Err(op) => println!("err: {:?}", op),
        }
    }
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<TResponse, Error> {
        let req = self.inner.into_request()?;

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TBody, TResponse> BulkRequestBuilder<AsyncSender, TBody, TResponse>
where
    TBody: Into<AsyncBody> + BulkBody + Send + 'static,
    TResponse: DeserializeOwned + IsOk + Send + 'static,
{
    /**
    Send a `BulkRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised search response.

    # Examples

    Send a bulk request to index some documents:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    # }
    # let client = AsyncClientBuilder::new().build()?;
    let ops = (0..1000)
        .into_iter()
        .map(|i| bulk::<MyType>().index(MyType {
                id: i.to_string(),
                title: "some string value".into()
            })
            .id(i));

    let future = client.bulk()
                       .index("myindex")
                       .ty(MyType::static_ty())
                       .extend(ops)
                       .send();

    future.and_then(|response| {
        for op in response {
            match op {
                Ok(op) => println!("ok: {:?}", op),
                Err(op) => println!("err: {:?}", op),
            }
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending<TResponse> {
        let (client, params_builder, inner) = (self.client, self.params_builder, self.inner);

        let req_future = client.sender.maybe_async(move || inner.into_request());

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response())
        });

        Pending::new(res_future)
    }
}

const DEFAULT_BODY_SIZE: usize = 1024 * 1024 * 5;
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/**
A streaming bulk request body.
*/
pub struct Streamed<TDocument> {
    body_size: usize,
    timeout: Duration,
    _marker: PhantomData<TDocument>,
}

impl<TDocument> Streamed<TDocument> {
    fn new() -> Self {
        Streamed {
            body_size: DEFAULT_BODY_SIZE,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            _marker: PhantomData,
        }
    }
}

struct WrappedBody<T> {
    inner: T,
    errs: Vec<Error>,
}

impl<T> WrappedBody<T> {
    fn new(inner: T) -> Self {
        WrappedBody {
            inner,
            errs: Vec::new(),
        }
    }

    fn with_inner_mut(&mut self, f: impl FnOnce(&mut T) -> Result<(), Error>) {
        if let Err(e) = f(&mut self.inner) {
            self.errs.push(e);
        }
    }

    fn try_into_inner(self) -> Result<T, Error> {
        if !self.errs.is_empty() {
            Err(error::request(BulkBodyError(self.errs)))
        } else {
            Ok(self.inner)
        }
    }
}

#[derive(Debug)]
struct BulkBodyError(Vec<Error>);

impl fmt::Display for BulkBodyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "errors ({}) writing bulk request body:", self.0.len())?;

        for err in &self.0 {
            writeln!(f, "{}", err)?;
        }

        Ok(())
    }
}

impl StdError for BulkBodyError {
    fn description(&self) -> &str {
        "errors writing bulk request body"
    }
}

#[derive(Debug)]
struct BulkRequestError(String);

impl fmt::Display for BulkRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl StdError for BulkRequestError {
    fn description(&self) -> &str {
        "error building bulk request body"
    }
}

/**
A bulk request body.

The body can receive a bulk operation for any type of document.
*/
pub trait BulkBody {
    /**
    Push a new document onto the end of the body.

    # Errors

    If the document can't be serialized then this method will return an error.
    There's no guarantee that other operations can be pushed onto the body after an error has occurred.
    */
    fn push<TDocument>(&mut self, op: BulkOperation<TDocument>) -> Result<(), Error>
    where
        TDocument: Serialize;
}

impl BulkBody for Vec<u8> {
    fn push<TDocument>(&mut self, op: BulkOperation<TDocument>) -> Result<(), Error>
    where
        TDocument: Serialize,
    {
        op.write(self).map_err(error::request)?;

        Ok(())
    }
}

/** A future returned by calling `send`. */
pub type Pending<TResponse> = BasePending<TResponse>;

#[doc(hidden)]
pub trait ChangeIndex<TIndex> {
    type WithNewIndex;
}

impl<TIndex, TType, TId, TNewIndex> ChangeIndex<TNewIndex> for BulkResponse<TIndex, TType, TId> {
    type WithNewIndex = BulkResponse<TNewIndex, TType, TId>;
}

impl<TIndex, TType, TId, TNewIndex> ChangeIndex<TNewIndex>
    for BulkErrorsResponse<TIndex, TType, TId>
{
    type WithNewIndex = BulkErrorsResponse<TNewIndex, TType, TId>;
}

#[doc(hidden)]
pub trait ChangeType<TType> {
    type WithNewType;
}

impl<TIndex, TType, TId, TNewType> ChangeType<TNewType> for BulkResponse<TIndex, TType, TId> {
    type WithNewType = BulkResponse<TIndex, TNewType, TId>;
}

impl<TIndex, TType, TId, TNewType> ChangeType<TNewType> for BulkErrorsResponse<TIndex, TType, TId> {
    type WithNewType = BulkErrorsResponse<TIndex, TNewType, TId>;
}

#[doc(hidden)]
pub trait ChangeId<TId> {
    type WithNewId;
}

impl<TIndex, TType, TId, TNewId> ChangeId<TNewId> for BulkResponse<TIndex, TType, TId> {
    type WithNewId = BulkResponse<TIndex, TType, TNewId>;
}

impl<TIndex, TType, TId, TNewId> ChangeId<TNewId> for BulkErrorsResponse<TIndex, TType, TId> {
    type WithNewId = BulkErrorsResponse<TIndex, TType, TNewId>;
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        tests::*,
    };

    #[test]
    fn is_send() {
        assert_send::<super::Pending<BulkResponse>>();
    }

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.bulk().inner.into_request().unwrap();

        assert_eq!("/_bulk", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .bulk()
            .index("test-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/_bulk", req.url.as_ref());
    }

    #[test]
    fn specify_index_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .bulk()
            .index("test-idx")
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/new-ty/_bulk", req.url.as_ref());
    }

    #[test]
    fn specify_ty_without_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.bulk().ty("new-ty").inner.into_request();

        assert!(req.is_err());
    }
}
