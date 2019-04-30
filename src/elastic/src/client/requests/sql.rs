/*!
Builders for [sql queries][sql].

[sql]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-rest.html
*/

use futures::{
    Future,
    Poll,
};

use client::requests::endpoints::SqlQueryRequest;
use client::requests::raw::RawRequestInner;
use client::requests::{
    empty_body,
    DefaultBody,
    RequestBuilder,
};
use client::responses::SqlResponse;
use client::sender::{
    AsyncSender,
    Sender,
    SyncSender,
};
use client::Client;

use error::{
    Error,
    Result,
};

use serde_json::json;

/**
A [sql query request][sql] builder that can be configured before sending.

Call [`Client.sql`][Client.sql] to get a `SqlRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[sql]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-rest.html[docs-delete]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.sql]: ../../struct.Client.html#sql-request
*/
pub type SqlRequestBuilder<TSender, TBody> = RequestBuilder<TSender, SqlRequestInner<TBody>>;

#[doc(hidden)]
pub struct SqlRequestInner<TBody> {
    body: TBody,
}

/**
# Sql request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /**
    Creates a [`SqlRequestBuilder`][SqlRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Runs a simple [Query String][docs-querystring] query:

    ```no_run
    # extern crate elastic;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.sql()
                         .body(json!({
                           "query": "SELECT * FROM library GROUP BY author"
                         }))
                         .send()?;

    // Iterate through the hits
    for row in response.rows() {
        for column in row {
            println!("{:?}", column);
        }
    }
    # Ok(())
    # }
    ```

    [SqlRequestBuilder]: requests/sql/type.SqlRequestBuilder.html
    [builder-methods]: requests/sql/type.SqlRequestBuilder.html#builder-methods
    [send-sync]: requests/sql/type.SqlRequestBuilder.html#send-synchronously
    [send-async]: requests/sql/type.SqlRequestBuilder.html#send-asynchronously
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-commands.html
    */
    pub fn sql(&self) -> SqlRequestBuilder<TSender, DefaultBody> {
        RequestBuilder::initial(self.clone(), SqlRequestInner::new(empty_body()))
    }

    /**
    Createss a sql query request.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Runs a simple [Query String][docs-querystring] query:

    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.sql_query("SELECT * FROM library GROUP BY author")
                         .send()?;

    // Iterate through the hits
    for row in response.rows() {
        for column in row {
            println!("{:?}", column);
        }
    }
    # Ok(())
    # }
    ```

    [send-sync]: requests/sql/type.SqlRequestBuilder.html#send-synchronously
    [send-async]: requests/sql/type.SqlRequestBuilder.html#send-asynchronously
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-commands.html
    */
    pub fn sql_query(&self, query: &str) -> SqlRequestBuilder<TSender, serde_json::Value> {
        self.sql().query(query)
    }
}

impl<TBody> SqlRequestInner<TBody> {
    fn new(body: TBody) -> Self {
        SqlRequestInner { body: body }
    }

    fn into_request(self) -> SqlQueryRequest<'static, TBody> {
        SqlQueryRequest::new(self.body)
    }
}

/**
# Builder methods

Configures a `SqlRequestBuilder` before sending it.
*/
impl<TSender, TBody> SqlRequestBuilder<TSender, TBody>
where
    TSender: Sender,
{
    /**
    Sets the body for the sql request.

    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self, body: TNewBody) -> SqlRequestBuilder<TSender, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            SqlRequestInner { body: body },
        )
    }

    /**
    Sets the query for the sql request.
    */
    pub fn query(self, query: &str) -> SqlRequestBuilder<TSender, serde_json::Value> {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            SqlRequestInner {
                body: json!({ "query": query }),
            },
        )
    }
}

/**
# Send synchronously
 */
impl<TBody> SqlRequestBuilder<SyncSender, TBody>
where
    TBody: Into<<SyncSender as Sender>::Body> + 'static,
{
    /**
    Sends a `SqlRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Runs a simple [Query String][docs-querystring] query:

    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.sql_query("SELECT * FROM library GROUP BY author")
                         .send()?;

    // Iterate through the hits
    for row in response.rows() {
        for column in row {
            println!("{:?}", column);
        }
    }
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-commands.html
     */
    pub fn send(self) -> Result<SqlResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
 */
impl<TBody> SqlRequestBuilder<AsyncSender, TBody>
where
    TBody: Into<<AsyncSender as Sender>::Body> + 'static,
{
    /**
    Sends a `SqlRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised search response.

    # Examples

    Runs a simple [Query String][docs-querystring] query:

    ```no_run
    # extern crate tokio;
    # extern crate futures;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.sql_query("SELECT * FROM library GROUP BY author")
                       .send();

    future.and_then(|response| {
        // Iterate through the hits
        for row in response.rows() {
            for column in row {
                println!("{:?}", column);
            }
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-commands.html
    */

    pub fn send(self) -> Pending {
        let req = self.inner.into_request();

        let res_future =
            RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = SqlResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = SqlResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = SqlResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;
    use serde_json::Value;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.sql().inner.into_request();

        assert_eq!("/_xpack/sql", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.sql().body("{}").inner.into_request();

        assert_eq!("{}", req.body);
    }
}
