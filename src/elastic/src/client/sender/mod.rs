/*!
Low-level http client details.

This module contains low-level implementation details for the high-level [`Client`][Client].
Most of the types here aren't currently usable directly, but once the low-level API matures they should be extensible.
Some notable types include:

- `Sender`: a generic trait that can send a http request and return a response
- `NextParams`: a generic trait that can fetch a set of parameters to associate with a request
- `SyncSender`: a synchronous http client
- `AsyncSender`: an asynchronous http client.

[Client]: ../struct.Client.html
*/

pub mod static_nodes;
pub mod sniffed_nodes;

mod sync;
mod async;
mod params;
pub use self::sync::*;
pub use self::async::*;
pub use self::params::*;

use std::sync::Arc;
use std::marker::PhantomData;
use uuid::Uuid;

use client::requests::HttpRequest;
use self::static_nodes::StaticNodes;
use self::sniffed_nodes::SniffedNodes;
use private;

/**
A sendable request.

This type encapsulates the state needed between a [`Client`][Client] and a [`Sender`][Sender] in order to send a request.

[Client]: ../struct.Client.html
[Sender]: trait.Sender.html
*/
pub struct SendableRequest<TRequest, TParams, TBody> {
    correlation_id: Uuid,
    inner: TRequest,
    params: TParams,
    params_builder: Option<Arc<Fn(RequestParams) -> RequestParams>>,
    _marker: PhantomData<TBody>,
}

impl<TRequest, TParams, TBody> SendableRequest<TRequest, TParams, TBody> {
    pub(crate) fn new(inner: TRequest, params: TParams, params_builder: Option<Arc<Fn(RequestParams) -> RequestParams>>) -> Self {
        SendableRequest {
            correlation_id: Uuid::new_v4(),
            inner: inner,
            params: params,
            params_builder: params_builder,
            _marker: PhantomData,
        }
    }
}

/**
Represents a type that can send a request.

You probably don't need to touch this trait directly.
See the [`Client`][Client] type for making requests.

[Client]: struct.Client.html
*/
pub trait Sender: private::Sealed + Clone {
    /* The kind of request body this sender accepts. */
    type Body;
    /* The kind of response this sender produces. */
    type Response;
    /* The kind of request parameters this sender accepts. */
    type Params;

    /* Send a request. */
    fn send<TRequest, TParams, TBody>(&self, request: SendableRequest<TRequest, TParams, TBody>) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body> + 'static,
        TParams: Into<Self::Params> + 'static;
}

/**
Represents a type that can fetch request parameters.
*/
pub trait NextParams: private::Sealed + Clone {
    /* The kind of parameters produces. */
    type Params;

    /* Get a set of request parameters. */
    fn next(&self) -> Self::Params;
}

/**
A common container for a source of node addresses.
*/
#[derive(Clone)]
pub struct NodeAddresses<TSender> {
    inner: NodeAddressesInner<TSender>,
}

impl<TSender> NodeAddresses<TSender> {
    fn static_nodes(nodes: Vec<Arc<str>>, params: PreRequestParams) -> Self {
        NodeAddresses {
            inner: NodeAddressesInner::Static(StaticNodes::round_robin(nodes, params))
        }
    }

    fn sniffed_nodes(sender: TSender, params: RequestParams) -> Self {
        NodeAddresses {
            inner: NodeAddressesInner::Sniffed(SniffedNodes::new(sender, params))
        }
    }
}

impl<TSender> private::Sealed for NodeAddresses<TSender> {}

#[derive(Clone)]
enum NodeAddressesInner<TSender> {
    Static(StaticNodes),
    Sniffed(SniffedNodes<TSender>),
}

enum NodeAddressesBuilder {
    Static(Vec<Arc<str>>),
    Sniffed(Arc<str>),
}

impl Default for NodeAddressesBuilder {
    fn default() -> Self {
        NodeAddressesBuilder::Static(vec![DEFAULT_NODE_ADDRESS.into()])
    }
}

impl NodeAddressesBuilder {
    fn build<TSender>(self, params: PreRequestParams, sender: TSender) -> NodeAddresses<TSender> {
        match self {
            NodeAddressesBuilder::Static(nodes) => {
                NodeAddresses::static_nodes(nodes, params)
            },
            NodeAddressesBuilder::Sniffed(default_node) => {
                let params = RequestParams::from_parts(default_node, params);

                NodeAddresses::sniffed_nodes(sender, params)
            }
        }
    }
}
