use uuid::Uuid;
use elastic_reqwest::{SyncBody, SyncElasticClient};
use elastic_reqwest::sniffer::{MultipleAddresses, RoundRobin};
use reqwest::{Client as SyncHttpClient, ClientBuilder as SyncHttpClientBuilder};

use error::{self, Result};
use client::requests::HttpRequest;
use client::responses::{sync_response, SyncResponseBuilder};
use client::{private, Client, RequestParams, RequestParamsBuilder, Sender, SendableRequest, DEFAULT_NODE_ADDRESS};

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

let response = client.request(PingRequest::new())
                     .send()?
                     .into_response::<PingResponse>()?;
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
    addresses: SyncAddresses,
}

#[derive(Clone)]
enum SyncAddresses {
    Static(MultipleAddresses<RoundRobin>),
}

impl SyncAddresses {
    fn next(&self) -> Result<RequestParams> {
        match *self {
            SyncAddresses::Static(ref addresses) => Ok(addresses.next()),
        }
    }
}

impl private::Sealed for SyncSender {}

impl Sender for SyncSender {
    type Body = SyncBody;
    type Response = Result<SyncResponseBuilder>;

    fn send<TRequest, TBody>(&self, request: SendableRequest<TRequest, TBody>) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body> + 'static
    {
        let correlation_id = Uuid::new_v4();
        let params_builder = request.params_builder;
        let req = request.inner.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', path: '{}'",
            correlation_id,
            req.url.as_ref()
        );

        let params = self.addresses.next()
            .map_err(|e| {
                error!(
                    "Elasticsearch Node Selection: correlation_id: '{}', error: '{}'",
                    correlation_id,
                    e
                );
                e
            })
            .map(move |params| {
                if let Some(params_builder) = params_builder {
                    params_builder(params)
                } else {
                    params
                }
            })?;

        let res = match self.http.elastic_req(&params, req).map_err(error::request) {
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

/** A builder for a syncronous client. */
pub struct SyncClientBuilder {
    http: Option<SyncHttpClient>,
    addresses: SyncAddressesBuilder,
    params: RequestParamsBuilder,
}

enum SyncAddressesBuilder {
    Static(Vec<String>),
}

impl Default for SyncAddressesBuilder {
    fn default() -> Self {
        SyncAddressesBuilder::Static(vec![DEFAULT_NODE_ADDRESS.to_owned()])
    }
}

impl SyncAddressesBuilder {
    fn build(self, params: RequestParamsBuilder) -> SyncAddresses {
        match self {
            SyncAddressesBuilder::Static(addresses) => {
                SyncAddresses::Static(MultipleAddresses::round_robin(addresses, params))
            }
        }
    }
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
            addresses: SyncAddressesBuilder::default(),
            params: RequestParamsBuilder::default(),
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: RequestParamsBuilder) -> Self {
        SyncClientBuilder {
            http: None,
            addresses: SyncAddressesBuilder::default(),
            params: params,
        }
    }

    /**
    Specify a set of static node addresses to load balance requests on.
    */
    pub fn static_addresses<I>(mut self, addresses: I) -> Self
        where I: IntoIterator<Item = String>
    {
        self.addresses = SyncAddressesBuilder::Static(addresses.into_iter().collect());

        self
    }

    /**
    Specify default request parameters.
    
    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| p.url_param("pretty", true));
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params(|p| p.header(Authorization("let me in".to_owned())));
    ```

    Specify a base url (prefer the [`base_url`][SyncClientBuilder.base_url] method on `SyncClientBuilder` instead):

    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| p.base_url("https://my_es_cluster/some_path"));
    ```

    [SyncClientBuilder.base_url]: #method.base_url
    */
    pub fn params<F>(mut self, builder: F) -> Self
    where
        F: Fn(RequestParamsBuilder) -> RequestParamsBuilder,
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

        let addresses = self.addresses.build(self.params);

        Ok(SyncClient {
            sender: SyncSender {
                http: http,
                addresses: addresses,
            }
        })
    }
}
