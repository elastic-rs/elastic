use std::sync::Arc;
use reqwest::{Client as SyncHttpClient, ClientBuilder as SyncHttpClientBuilder, RequestBuilder as SyncHttpRequestBuilder};

use error::{self, Result};
use private;
use client::sender::{build_method, build_url, NextParams, NodeAddress, NodeAddresses, NodeAddressesBuilder, NodeAddressesInner, PreRequestParams, RequestParams, SendableRequest, Sender};
use client::sender::sniffed_nodes::SniffedNodesBuilder;
use client::requests::{HttpRequest, SyncBody};
use client::responses::{sync_response, SyncResponseBuilder};
use client::Client;

/** 
A synchronous Elasticsearch client.

Use a [`SyncClientBuilder`][SyncClientBuilder] to configure and build a `SyncClient`.
 
# Examples

Create a synchronous `Client` and send a ping request:

```no_run
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;

let response = client.ping().send()?;
# Ok(())
# }
```

[SyncClientBuilder]: struct.SyncClientBuilder.html
*/
pub type SyncClient = Client<SyncSender>;

/** A synchronous request sender. */
#[derive(Clone)]
pub struct SyncSender {
    pub(in client) http: SyncHttpClient,
}

impl private::Sealed for SyncSender {}

impl Sender for SyncSender {
    type Body = SyncBody;
    type Response = Result<SyncResponseBuilder>;
    type Params = Params;

    fn send<TRequest, TParams, TBody>(&self, request: SendableRequest<TRequest, TParams, TBody>) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body> + 'static,
        TParams: Into<Self::Params> + 'static,
    {
        let correlation_id = request.correlation_id;
        let params_builder = request.params_builder;
        let req = request.inner.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', path: '{}'",
            correlation_id,
            req.url.as_ref()
        );

        let params = request.params.into().inner.map_err(|e| {
            error!(
                "Elasticsearch Node Selection: correlation_id: '{}', error: '{}'",
                correlation_id,
                e
            );
            e
        })?;

        let params = params_builder.into_value(move || params);

        let mut req = build_req(&self.http, params, req);

        let res = match req.send().map_err(error::request) {
            Ok(res) => {
                info!(
                    "Elasticsearch Response: correlation_id: '{}', status: '{}'",
                    correlation_id,
                    res.status()
                );
                res
            }
            Err(e) => {
                error!(
                    "Elasticsearch Response: correlation_id: '{}', error: '{}'",
                    correlation_id,
                    e
                );
                Err(e)?
            }
        };

        Ok(sync_response(res))
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
    inner: Result<RequestParams>,
}

impl Params {
    fn new(res: Result<RequestParams>) -> Self {
        Params { inner: res }
    }
}

impl From<RequestParams> for Params {
    fn from(params: RequestParams) -> Self {
        Params::new(Ok(params))
    }
}

/** Build a synchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
fn build_req<I, B>(client: &SyncHttpClient, params: RequestParams, req: I) -> SyncHttpRequestBuilder
where
    I: Into<HttpRequest<'static, B>>,
    B: Into<SyncBody>,
{
    let req = req.into();

    let url = build_url(&req.url, &params);
    let method = build_method(req.method);
    let body = req.body;

    let mut req = client.request(method, &url);
    {
        req.headers(params.get_headers());

        if let Some(body) = body {
            req.body(body.into().into_inner());
        }
    }

    req
}

/** A builder for a syncronous client. */
pub struct SyncClientBuilder {
    http: Option<SyncHttpClient>,
    nodes: NodeAddressesBuilder,
    params: PreRequestParams,
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
            params: PreRequestParams::default(),
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: PreRequestParams) -> Self {
        SyncClientBuilder {
            http: None,
            nodes: NodeAddressesBuilder::default(),
            params: params,
        }
    }

    /**
    Specify a static node nodes to send requests to.
    */
    pub fn static_node<S>(self, node: S) -> Self
    where
        S: Into<NodeAddress>,
    {
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
    pub fn sniff_nodes<I>(mut self, builder: I) -> Self
    where
        I: Into<SniffedNodesBuilder>,
    {
        self.nodes = NodeAddressesBuilder::Sniffed(builder.into());

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
    pub fn sniff_nodes_fluent<I, F>(mut self, address: I, builder: F) -> Self
    where
        I: Into<NodeAddress>,
        F: Fn(SniffedNodesBuilder) -> SniffedNodesBuilder,
    {
        let address = address.into();
        self.nodes = NodeAddressesBuilder::Sniffed(builder(address.into()));

        self
    }

    /**
    Specify default request parameters.
    
    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| {
            p.url_param("pretty", true)
        });
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params(|p| {
            p.header(Authorization("let me in".to_owned()))
        });
    ```
    [SyncClientBuilder.base_url]: #method.base_url
    */
    pub fn params<F>(mut self, builder: F) -> Self
    where
        F: Fn(PreRequestParams) -> PreRequestParams,
    {
        self.params = builder(self.params);

        self
    }

    /** Use the given `reqwest::Client` for sending requests. */
    pub fn http_client(mut self, client: SyncHttpClient) -> Self {
        self.http = Some(client);

        self
    }

    /** 
    Construct a [`SyncClient`][SyncClient] from this builder. 

    [Client]: struct.Client.html
    */
    pub fn build(self) -> Result<SyncClient> {
        let http = self.http
            .map(Ok)
            .unwrap_or_else(|| SyncHttpClientBuilder::new().build())
            .map_err(error::build)?;

        let sender = SyncSender { http: http };

        let addresses = self.nodes.build(self.params, sender.clone());

        Ok(SyncClient {
            sender: sender,
            addresses: addresses,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use reqwest::{Client, Method, RequestBuilder};
    use reqwest::header::ContentType;

    use super::*;
    use client::requests::*;

    fn params() -> RequestParams {
        RequestParams::new("eshost:9200/path").url_param("pretty", false)
    }

    fn builder() -> Option<Arc<Fn(RequestParams) -> RequestParams>> {
        Some(Arc::new(|params| params.url_param("pretty", true)))
    }

    fn expected_req(cli: &Client, method: Method, url: &str, body: Option<Vec<u8>>) -> RequestBuilder {
        let mut req = cli.request(method, url);
        {
            req.header(ContentType::json());

            if let Some(body) = body {
                req.body(body);
            }
        }

        req
    }

    fn assert_req(expected: RequestBuilder, actual: RequestBuilder) {
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[test]
    fn head_req() {
        let cli = Client::new();
        let req = build_req(&cli, params(), builder(), PingHeadRequest::new());

        let url = "eshost:9200/path/?pretty=true";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn get_req() {
        let cli = Client::new();
        let req = build_req(&cli, params(), builder(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn post_req() {
        let cli = Client::new();
        let req = build_req(
            &cli,
            params(),
            builder(),
            PercolateRequest::for_index_ty("idx", "ty", vec![]),
        );

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn put_req() {
        let cli = Client::new();
        let req = build_req(
            &cli,
            params(),
            builder(),
            IndicesCreateRequest::for_index("idx", vec![]),
        );

        let url = "eshost:9200/path/idx?pretty=true";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn delete_req() {
        let cli = Client::new();
        let req = build_req(
            &cli,
            params(),
            builder(),
            IndicesDeleteRequest::for_index("idx"),
        );

        let url = "eshost:9200/path/idx?pretty=true";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn file_into_body() {
        SyncBody::from(File::open("Cargo.toml").unwrap());
    }

    #[test]
    fn owned_string_into_body() {
        SyncBody::from(String::new());
    }

    #[test]
    fn borrowed_string_into_body() {
        SyncBody::from("abc");
    }

    #[test]
    fn owned_vec_into_body() {
        SyncBody::from(Vec::new());
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &'static [u8] = &[0, 1, 2];

        SyncBody::from(BODY);
    }

    #[test]
    fn empty_body_into_body() {
        SyncBody::from(empty_body());
    }

    #[test]
    fn json_value_into_body() {
        SyncBody::from(json!({}));
    }
}
