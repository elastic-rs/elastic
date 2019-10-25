use reqwest::{
    Client as SyncHttpClient,
    RequestBuilder as SyncHttpRequestBuilder,
};
use std::{
    error::Error as StdError,
    sync::Arc,
};

use crate::{
    endpoints::Endpoint,
    error::{
        self,
        Error,
    },
    http::{
        receiver::{
            sync_response,
            SyncResponseBuilder,
        },
        sender::{
            build_reqwest_method,
            build_url,
            NextParams,
            NodeAddresses,
            NodeAddressesInner,
            RequestParams,
            SendableRequest,
            SendableRequestParams,
            Sender,
        },
        SyncBody,
        SyncHttpRequest,
        Url,
    },
    private,
};

pub(crate) type SyncPreSend =
    dyn Fn(&mut SyncHttpRequest) -> Result<(), Box<dyn StdError + Send + Sync>> + Send + Sync;

/** A synchronous request sender. */
#[derive(Clone)]
pub struct SyncSender {
    pub(crate) http: SyncHttpClient,
    pub(crate) pre_send: Option<Arc<SyncPreSend>>,
}

impl private::Sealed for SyncSender {}

impl Sender for SyncSender {
    type Body = SyncBody;
    type Response = Result<SyncResponseBuilder, Error>;
    type Params = Params;

    fn send<TEndpoint, TParams, TBody>(
        &self,
        request: SendableRequest<TEndpoint, TParams, TBody>,
    ) -> Self::Response
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<Self::Body> + Send + 'static,
        TParams: Into<Self::Params> + Send + 'static,
    {
        let correlation_id = request.correlation_id;
        let params = request.params;
        let endpoint = request.inner.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', path: '{}'",
            correlation_id,
            endpoint.url.as_ref()
        );

        let params = match params {
            SendableRequestParams::Value(params) => params,
            SendableRequestParams::Builder { params, builder } => {
                let params = params.into().inner.log_err(|e| {
                    error!(
                        "Elasticsearch Node Selection: correlation_id: '{}', error: '{:?}'",
                        correlation_id, e
                    )
                })?;

                builder.into_value(move || params)
            }
        };

        let mut req = build_req(endpoint, params).log_err(|e| {
            error!(
                "Elasticsearch Request: correlation_id: '{}', error: '{:?}'",
                correlation_id, e
            )
        })?;

        if let Some(ref pre_send) = self.pre_send {
            pre_send(&mut req)
                .map_err(error::wrapped)
                .map_err(error::request)
                .log_err(|e| {
                    error!(
                        "Elasticsearch Request Pre-send: correlation_id: '{}', error: '{:?}'",
                        correlation_id, e
                    )
                })?;
        }

        let req = build_reqwest(&self.http, req)
            .build()
            .map_err(error::request)?;

        let res = match self.http.execute(req).map_err(error::request) {
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
                    "Elasticsearch Response: correlation_id: '{}', error: '{:?}'",
                    correlation_id, e
                );
                return Err(e);
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
fn build_req(
    endpoint: Endpoint<impl Into<SyncBody>>,
    params: RequestParams,
) -> Result<SyncHttpRequest, Error> {
    let endpoint = SyncHttpRequest {
        url: Url::parse(&build_url(&endpoint.url, &params)).map_err(error::request)?,
        method: endpoint.method,
        headers: params.get_headers(),
        body: endpoint.body.map(|body| body.into()),
    };

    Ok(endpoint)
}

/** Build a synchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
fn build_reqwest(client: &SyncHttpClient, req: SyncHttpRequest) -> SyncHttpRequestBuilder {
    let SyncHttpRequest {
        url,
        method,
        headers,
        body,
        ..
    } = req;

    let method = build_reqwest_method(method);

    let mut req = client.request(method, url);
    {
        req = req.headers((&*headers).clone());

        if let Some(body) = body {
            req = req.body(body.into_inner());
        }
    }

    req
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
