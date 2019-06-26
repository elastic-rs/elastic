use fluent_builder::SharedFluentBuilder;
use futures::Future;
use reqwest::r#async::Client as AsyncHttpClient;
use std::{
    error::Error as StdError,
    sync::Arc,
};
use tokio_threadpool::ThreadPool;

use crate::{
    client::Client,
    error::Error,
    http::{
        sender::{
            sniffed_nodes::SniffedNodesBuilder,
            AsyncPreSend,
            AsyncSender,
            NodeAddress,
            NodeAddressesBuilder,
            PreRequestParams,
        },
        AsyncHttpRequest,
    },
};

/**
An asynchronous Elasticsearch client.

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure and build an `AsyncClient`.
For more details about the methods available to an `AsyncClient`, see the base [`Client`][Client] type.

# Examples

Create an asynchronous `Client` and send a ping request:

```no_run
# use futures::Future;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
let client = AsyncClientBuilder::new().build()?;

let response_future = client.request(PingRequest::new())
                            .send()
                            .and_then(|res| res.into_response::<PingResponse>());

tokio::runtime::current_thread::block_on_all(response_future)?;
# Ok(())
# }
```

[Client]: ../struct.Client.html
[AsyncClientBuilder]: struct.AsyncClientBuilder.html
*/
pub type AsyncClient = Client<AsyncSender>;

impl AsyncClient {
    /**
    Get a builder for an asynchronous client.
    */
    pub fn builder() -> AsyncClientBuilder {
        AsyncClientBuilder::new()
    }
}

/** A builder for an asynchronous client. */
pub struct AsyncClientBuilder {
    http: Option<AsyncHttpClient>,
    serde_pool: Option<Arc<ThreadPool>>,
    nodes: NodeAddressesBuilder,
    params: SharedFluentBuilder<PreRequestParams>,
    pre_send: Option<Arc<AsyncPreSend>>,
}

impl Default for AsyncClientBuilder {
    fn default() -> Self {
        AsyncClientBuilder::new()
    }
}

impl AsyncClientBuilder {
    /**
    Create a new client builder.

    By default, a client constructed by this builder will:

    - Send requests to `localhost:9200`
    - Not deserialise repsonses on a cpu pool
    - Not use any authentication
    - Not use TLS
    */
    pub fn new() -> Self {
        AsyncClientBuilder {
            http: None,
            serde_pool: None,
            params: SharedFluentBuilder::new(),
            nodes: NodeAddressesBuilder::default(),
            pre_send: None,
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: PreRequestParams) -> Self {
        AsyncClientBuilder {
            http: None,
            serde_pool: None,
            params: SharedFluentBuilder::new().value(params),
            nodes: NodeAddressesBuilder::default(),
            pre_send: None,
        }
    }

    /**
    Specify a static node nodes to send requests to.
    */
    pub fn static_node(self, node: impl Into<NodeAddress>) -> Self {
        self.static_nodes(vec![node])
    }

    /**
    Specify a set of static node nodes to load balance requests on.
    */
    pub fn static_nodes<I, S>(mut self, nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NodeAddress>,
    {
        let nodes = nodes.into_iter().map(|address| address.into()).collect();
        self.nodes = NodeAddressesBuilder::Static(nodes);

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.

    # Examples

    Use a given base url for sniffing the cluster's node addresses from:

    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .sniff_nodes("http://localhost:9200");
    ```
    */
    pub fn sniff_nodes(mut self, builder: impl Into<SniffedNodesBuilder>) -> Self {
        self.nodes = self.nodes.sniff_nodes(builder.into());

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.

    # Examples

    Use a given base url for sniffing the cluster's node addresses from and specify a minimum duration to wait before refreshing:

    ```
    # use std::time::Duration;
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .sniff_nodes_fluent("http://localhost:9200", |n| n
            .wait(Duration::from_secs(90)));
    ```
    */
    pub fn sniff_nodes_fluent(
        mut self,
        address: impl Into<NodeAddress>,
        builder: impl Fn(SniffedNodesBuilder) -> SniffedNodesBuilder + Send + 'static,
    ) -> Self {
        self.nodes = self.nodes.sniff_nodes_fluent(address.into(), builder);

        self
    }

    /**
    Specify default request parameters.

    # Examples

    Require all responses use pretty-printing:

    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .params_fluent(|p| p
            .url_param("pretty", true));
    ```
    */
    pub fn params_fluent(
        mut self,
        builder: impl Fn(PreRequestParams) -> PreRequestParams + Send + 'static,
    ) -> Self {
        self.params = self.params.fluent(builder).shared();

        self
    }

    /**
    Specify default request parameters.

    # Examples

    Require all responses use pretty-printing:

    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .params(PreRequestParams::new()
            .url_param("pretty", true));
    ```
    */
    pub fn params(mut self, params: impl Into<PreRequestParams>) -> Self {
        self.params = self.params.value(params.into());

        self
    }

    /**
    Use the given `ThreadPool` for serialising and deserialising responses.

    If the pool is `None` then responses will be serialised and deserialised on the same thread as the io `Core`.

    # Examples

    Use a cpu pool to serialise and deserialise responses:

    ```
    # extern crate tokio_threadpool;
    # use std::sync::Arc;
    # use elastic::prelude::*;
    # use tokio_threadpool::ThreadPool;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    let pool = ThreadPool::new();

    let builder = AsyncClientBuilder::new().serde_pool(Arc::new(pool));
    # Ok(())
    # }
    ```
    */
    pub fn serde_pool(mut self, serde_pool: impl Into<Option<Arc<ThreadPool>>>) -> Self {
        self.serde_pool = serde_pool.into();

        self
    }

    /**
    Specify a function to tweak a raw request before sending.

    This function will be applied to all outgoing requests and gives you the chance to perform operations the require the complete raw request,
    such as request singing.
    Prefer the `params` method on the client or individual requests where possible.
    */
    pub fn pre_send_raw(
        mut self,
        pre_send: impl Fn(
                &mut AsyncHttpRequest,
            )
                -> Box<dyn Future<Item = (), Error = Box<dyn StdError + Send + Sync>> + Send>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.pre_send = Some(Arc::new(pre_send));

        self
    }

    /** Use the given `reqwest::Client` for sending requests. */
    pub fn http_client(mut self, client: AsyncHttpClient) -> Self {
        self.http = Some(client);

        self
    }

    /**
    Construct an [`AsyncClient`][AsyncClient] from this builder.

    [AsyncClient]: type.AsyncClient.html
    */
    pub fn build(self) -> Result<AsyncClient, Error> {
        let http = self.http.unwrap_or_else(|| AsyncHttpClient::new());
        let params = self.params.into_value(|| PreRequestParams::default());

        let sender = AsyncSender {
            http,
            serde_pool: self.serde_pool,
            pre_send: self.pre_send,
        };

        let addresses = self.nodes.build(params, sender.clone());

        Ok(AsyncClient {
            sender: sender,
            addresses: addresses,
        })
    }
}
