use uuid::Uuid;
use elastic_reqwest::{SyncBody, SyncElasticClient};
use reqwest::Client as SyncHttpClient;

use error::{self, Result};
use client::requests::HttpRequest;
use client::responses::{sync_response, SyncResponseBuilder};
use client::{private, Client, Sender, RequestParams};

/** 
A synchronous Elasticsearch client.
 
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
*/
pub type SyncClient = Client<SyncSender>;

/** A synchronous request sender. */
#[derive(Clone)]
pub struct SyncSender {
    pub (in client) http: SyncHttpClient
}

impl private::Sealed for SyncSender {}

impl Sender for SyncSender {
    type Body = SyncBody;
    type Response = Result<SyncResponseBuilder>;

    fn send<TRequest, TBody>(&self, req: TRequest, params: &RequestParams) -> Self::Response
        where TRequest: Into<HttpRequest<'static, TBody>>,
              TBody: Into<Self::Body>
    {
        let correlation_id = Uuid::new_v4();
        let req = req.into();

        info!("Elasticsearch Request: correlation_id: '{}', path: '{}'", correlation_id, req.url.as_ref());

        let res = match self.http.elastic_req(params, req).map_err(|e| error::request(e)) {
            Ok(res) => {
                info!("Elasticsearch Response: correlation_id: '{}', status: '{}'", correlation_id, res.status());
                res
            },
            Err(e) => {
                error!("Elasticsearch Response: correlation_id: '{}', error: '{}'", correlation_id, e);
                Err(e)?
            }
        };
        
        Ok(sync_response(res))
    }
}

/** A builder for a syncronous client. */
pub struct SyncClientBuilder {
    http: Option<SyncHttpClient>,
    params: RequestParams
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
            params: RequestParams::default()
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: RequestParams) -> Self {
        SyncClientBuilder {
            http: None,
            params: params
        }
    }

    /**
    Set the base url. 

    The url must be fully qualified.
    This method is a convenient alternative to using `params` to specify the `base_url`.

    # Examples

    Specify a base url for the client to send requests to.
    In this case, the base url is HTTPS, and not on the root path:

    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .base_url("https://my_es_cluster/some_path");
    ```
    */
    pub fn base_url<I>(mut self, base_url: I) -> Self 
        where I: Into<String>
    {
        self.params = self.params.base_url(base_url);

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
        where F: Fn(RequestParams) -> RequestParams
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
        let http = self.http.map(|http| Ok(http))
                            .unwrap_or(SyncHttpClient::new())
                            .map_err(|e| error::build(e))?;

        Ok(SyncClient {
            sender: SyncSender {
                http: http
            },
            params: self.params
        })
    }
}
