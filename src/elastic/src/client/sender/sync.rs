use fluent_builder::FluentBuilder;
use reqwest::{Client as SyncHttpClient, ClientBuilder as SyncHttpClientBuilder, RequestBuilder as SyncHttpRequestBuilder};
use std::error::Error as StdError;
use std::sync::Arc;

use client::requests::Endpoint;
use client::responses::{sync_response, SyncResponseBuilder};
use client::sender::sniffed_nodes::SniffedNodesBuilder;
use client::sender::{build_reqwest_method, build_url, NextParams, NodeAddress, NodeAddresses, NodeAddressesBuilder, NodeAddressesInner, PreRequestParams, RequestParams, SendableRequest, SendableRequestParams, Sender};
use client::Client;
use error::{self, Error};
use http::{SyncBody, SyncHttpRequest, Url};
use private;

/**
A synchronous Elasticsearch client.

Use a [`SyncClientBuilder`][SyncClientBuilder] to configure and build a `SyncClient`.
For more details about the methods available to a `SyncClient`, see the base [`Client`][Client] type.

# Examples

Create a synchronous `Client` and send a ping request:

```no_run
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;

let response = client.request(PingRequest::new())
                     .send()?
                     .into_response::<PingResponse>()?;
# Ok(())
# }
```

[Client]: ../struct.Client.html
[SyncClientBuilder]: struct.SyncClientBuilder.html
*/
pub type SyncClient = Client<SyncSender>;

/** A synchronous request sender. */
#[derive(Clone)]
pub struct SyncSender {
    pub(in client) http: SyncHttpClient,
    pre_send: Option<Arc<Fn(&mut SyncHttpRequest) -> Result<(), Box<StdError + Send + Sync>> + Send + Sync>>,
}

impl private::Sealed for SyncSender {}

impl Sender for SyncSender {
    type Body = SyncBody;
    type Response = Result<SyncResponseBuilder, Error>;
    type Params = Params;

    fn send<TEndpoint, TParams, TBody>(&self, request: SendableRequest<TEndpoint, TParams, TBody>) -> Self::Response
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<Self::Body> + 'static,
        TParams: Into<Self::Params> + 'static,
    {
        let correlation_id = request.correlation_id;
        let params = request.params;
        let endpoint = request.inner.into();

        info!("Elasticsearch Request: correlation_id: '{}', path: '{}'", correlation_id, endpoint.url.as_ref());

        let params = match params {
            SendableRequestParams::Value(params) => params,
            SendableRequestParams::Builder { params, builder } => {
                let params = params.into().inner.log_err(|e| error!("Elasticsearch Node Selection: correlation_id: '{}', error: '{:?}'", correlation_id, e))?;

                builder.into_value(move || params)
            }
        };

        let mut req = build_req(endpoint, params).log_err(|e| error!("Elasticsearch Request: correlation_id: '{}', error: '{:?}'", correlation_id, e))?;

        if let Some(ref pre_send) = self.pre_send {
            pre_send(&mut req)
                .map_err(error::wrapped)
                .map_err(error::request)
                .log_err(|e| error!("Elasticsearch Request Pre-send: correlation_id: '{}', error: '{:?}'", correlation_id, e))?;
        }

        let req = build_reqwest(&self.http, req).build().map_err(error::request)?;

        let res = match self.http.execute(req).map_err(error::request) {
            Ok(res) => {
                info!("Elasticsearch Response: correlation_id: '{}', status: '{}'", correlation_id, res.status());
                res
            }
            Err(e) => {
                error!("Elasticsearch Response: correlation_id: '{}', error: '{:?}'", correlation_id, e);
                Err(e)?
            }
        };

        sync_response(res)
    }
}

impl NextParams for NodeAddresses<SyncSender> {
    type Params = Params;

    fn next(&self) -> Self::Params {
        match self.inner {
            NodeAddressesInner::Static(ref nodes) => Params::new(nodes.next()),
            NodeAddressesInner::Sniffed(ref sniffer) => Params::new(sniffer.next()),
        }
    }
}

/** A set of parameters returned by calling `next` on a sync set of `NodeAddresses`. */
pub struct Params {
    inner: Result<RequestParams, Error>,
}

impl Params {
    fn new(res: Result<RequestParams, Error>) -> Self {
        Params { inner: res }
    }
}

impl From<RequestParams> for Params {
    fn from(params: RequestParams) -> Self {
        Params::new(Ok(params))
    }
}

/** Build an Elasticsearch request from an endpoint. */
fn build_req(endpoint: Endpoint<impl Into<SyncBody>>, params: RequestParams) -> Result<SyncHttpRequest, Error> {
    let endpoint = SyncHttpRequest {
        url: Url::parse(&build_url(&endpoint.url, &params)).map_err(error::request)?,
        method: endpoint.method,
        headers: params.get_headers(),
        body: endpoint.body.map(|body| body.into()),
        _private: (),
    };

    Ok(endpoint)
}

/** Build a synchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
fn build_reqwest(client: &SyncHttpClient, req: SyncHttpRequest) -> SyncHttpRequestBuilder {
    let SyncHttpRequest { url, method, headers, body, .. } = req;

    let method = build_reqwest_method(method);

    let mut req = client.request(method, url);
    {
        req.headers(headers);

        if let Some(body) = body {
            req.body(body.into_inner());
        }
    }

    req
}

/** A builder for a syncronous client. */
pub struct SyncClientBuilder {
    http: Option<SyncHttpClient>,
    nodes: NodeAddressesBuilder,
    params: FluentBuilder<PreRequestParams>,
    pre_send: Option<Arc<Fn(&mut SyncHttpRequest) -> Result<(), Box<StdError + Send + Sync>> + Send + Sync + 'static>>,
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
            params: FluentBuilder::new(),
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
            params: FluentBuilder::new().value(params),
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
    pub fn sniff_nodes_fluent(mut self, address: impl Into<NodeAddress>, builder: impl Fn(SniffedNodesBuilder) -> SniffedNodesBuilder + 'static) -> Self {
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

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params_fluent(|p| p
            .header(Authorization("let me in".to_owned())));
    ```
    */
    pub fn params_fluent(mut self, builder: impl Fn(PreRequestParams) -> PreRequestParams + 'static) -> Self {
        self.params = self.params.fluent(builder).boxed();

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

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params(PreRequestParams::default()
            .header(Authorization("let me in".to_owned())));
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
    such as request singing.
    Prefer the `params` method on the client or individual requests where possible.
    */
    pub fn pre_send_raw(mut self, pre_send: impl Fn(&mut SyncHttpRequest) -> Result<(), Box<StdError + Send + Sync>> + Send + Sync + 'static) -> Self {
        self.pre_send = Some(Arc::new(pre_send));

        self
    }

    /**
    Construct a [`SyncClient`][SyncClient] from this builder.

    [SyncClient]: type.SyncClient.html
    */
    pub fn build(self) -> Result<SyncClient, Error> {
        let http = self.http.map(Ok).unwrap_or_else(|| SyncHttpClientBuilder::new().build()).map_err(error::build)?;

        let params = self.params.into_value(|| PreRequestParams::default());
        let sender = SyncSender { http, pre_send: self.pre_send };

        let addresses = self.nodes.build(params, sender.clone());

        Ok(SyncClient { sender: sender, addresses: addresses })
    }
}

trait LogErr<E> {
    fn log_err<F>(self, log: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E> LogErr<E> for Result<T, E> {
    fn log_err<F>(self, log: F) -> Self
    where
        F: FnOnce(&E),
    {
        if let Err(ref e) = self {
            log(e);
        }

        self
    }
}
