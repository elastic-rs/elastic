use fluent_builder::SharedFluentBuilder;
use reqwest::{
    Client as SyncHttpClient,
    ClientBuilder as SyncHttpClientBuilder,
};
use std::{
    error::Error as StdError,
    sync::Arc,
};

use crate::{
    client::Client,
    error::{
        self,
        Error,
    },
    http::{
        sender::{
            sniffed_nodes::SniffedNodesBuilder,
            NodeAddress,
            NodeAddressesBuilder,
            PreRequestParams,
            SyncPreSend,
            SyncSender,
        },
        SyncHttpRequest,
    },
};

/**
A synchronous Elasticsearch client.

Use a [`SyncClientBuilder`][SyncClientBuilder] to configure and build a `SyncClient`.
For more details about the methods available to a `SyncClient`, see the base [`Client`][Client] type.

# Examples

Create a synchronous `Client` and send a ping request:

```no_run
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;

let response = client.request(PingRequest::new())
                     .send()?
                     .into_response::<PingResponse>()?;
# Ok(())
# }
```

[Client]: ../client/struct.Client.html
[SyncClientBuilder]: struct.SyncClientBuilder.html
*/
pub type SyncClient = Client<SyncSender>;

impl SyncClient {
    /**
    Get a builder for a synchronous client.
    */
    pub fn builder() -> SyncClientBuilder {
        SyncClientBuilder::new()
    }
}

/** A builder for a syncronous client. */
pub struct SyncClientBuilder {
    http: Option<SyncHttpClient>,
    nodes: NodeAddressesBuilder,
    params: SharedFluentBuilder<PreRequestParams>,
    pre_send: Option<Arc<SyncPreSend>>,
}

impl Default for SyncClientBuilder {
    fn default() -> Self {
        SyncClientBuilder::new()
    }
}

impl SyncClientBuilder {
    /**
    Create a new client builder.

    By default, a client constructed by this builder will:

    - Send requests to `localhost:9200`
    - Not use any authentication
    - Not use TLS
    */
    pub fn new() -> Self {
        SyncClientBuilder {
            http: None,
            nodes: NodeAddressesBuilder::default(),
            params: SharedFluentBuilder::new(),
            pre_send: None,
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: PreRequestParams) -> Self {
        SyncClientBuilder {
            http: None,
            nodes: NodeAddressesBuilder::default(),
            params: SharedFluentBuilder::new().value(params),
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
    let builder = SyncClientBuilder::new()
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
    let builder = SyncClientBuilder::new()
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
    let builder = SyncClientBuilder::new()
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
    let builder = SyncClientBuilder::new()
        .params(PreRequestParams::default()
            .url_param("pretty", true));
    ```
    */
    pub fn params(mut self, params: impl Into<PreRequestParams>) -> Self {
        self.params = self.params.value(params.into());

        self
    }

    /** Use the given `reqwest::Client` for sending requests. */
    pub fn http_client(mut self, client: SyncHttpClient) -> Self {
        self.http = Some(client);

        self
    }

    /**
    Specify a function to tweak a raw request before sending.

    This function will be applied to all outgoing requests and gives you the chance to perform operations the require the complete raw request,
    such as request signing.
    Prefer the `params` method on the client or individual requests where possible.
    */
    pub fn pre_send_raw(
        mut self,
        pre_send: impl Fn(&mut SyncHttpRequest) -> Result<(), Box<dyn StdError + Send + Sync>>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.pre_send = Some(Arc::new(pre_send));

        self
    }

    /**
    Construct a [`SyncClient`][SyncClient] from this builder.

    [SyncClient]: type.SyncClient.html
    */
    pub fn build(self) -> Result<SyncClient, Error> {
        let http = self
            .http
            .map(Ok)
            .unwrap_or_else(|| SyncHttpClientBuilder::new().build())
            .map_err(error::build)?;

        let params = self.params.into_value(PreRequestParams::default);
        let sender = SyncSender {
            http,
            pre_send: self.pre_send,
        };

        let addresses = self.nodes.build(params, sender.clone());

        Ok(SyncClient { sender, addresses })
    }
}
