/*!
Low-level http client details for request sending.

This module contains low-level implementation details for the high-level [`Client`][Client].
Most of the types here aren't currently usable directly, but once the low-level API matures they should be extensible.
Some notable types include:

- `Sender`: a generic trait that can send a http request and return a response
- `NextParams`: a generic trait that can fetch a set of parameters to associate with a request
- `SyncSender`: a synchronous http client
- `AsyncSender`: an asynchronous http client.

[Client]: ../struct.Client.html
*/

use fluent_builder::{
    SharedFluentBuilder,
    SharedStatefulFluentBuilder,
};

pub mod sniffed_nodes;
pub mod static_nodes;

mod asynchronous;
mod params;
mod synchronous;
pub use self::{
    asynchronous::*,
    params::*,
    synchronous::*,
};

use std::{
    marker::PhantomData,
    sync::Arc,
};
use uuid::Uuid;

use self::{
    sniffed_nodes::{
        SniffedNodes,
        SniffedNodesBuilder,
    },
    static_nodes::StaticNodes,
};
use crate::{
    endpoints::Endpoint,
    private,
};

/**
A sendable request.

This type encapsulates the state needed between a [`Client`][Client] and a [`Sender`][Sender] in order to send a request.

[Client]: ../struct.Client.html
[Sender]: trait.Sender.html
*/
pub struct SendableRequest<TEndpoint, TParams, TBody> {
    correlation_id: Uuid,
    inner: TEndpoint,
    params: SendableRequestParams<TParams>,
    _marker: PhantomData<TBody>,
}

impl<TEndpoint, TParams, TBody> SendableRequest<TEndpoint, TParams, TBody> {
    pub(crate) fn new(inner: TEndpoint, params: SendableRequestParams<TParams>) -> Self {
        SendableRequest {
            correlation_id: Uuid::new_v4(),
            inner,
            params,
            _marker: PhantomData,
        }
    }
}

pub(crate) enum SendableRequestParams<TParams> {
    Value(RequestParams),
    Builder {
        params: TParams,
        builder: SharedFluentBuilder<RequestParams>,
    },
}

/**
Represents a type that can send a request.

You probably don't need to touch this trait directly.
See the [`Client`][Client] type for making requests.

Even though the `Sender` trait is quite generic, it's not really designed to be implemented externally.
The request builders expect there to be 2 concrete implementations of `Sender`, namely `SyncSender` and `AsyncSender`.
The real purpose is to make it possible to share builders so the sync and async APIs don't diverge.

At some point in the future though this may be made more generic so you could reasonably plug your own `Sender`s in to `elastic`.

[Client]: struct.Client.html
*/
pub trait Sender: private::Sealed + Clone {
    /** The kind of request body this sender accepts. */
    type Body;
    /** The kind of response this sender produces. */
    type Response;
    /** The kind of request parameters this sender accepts. */
    type Params;

    /** Send a request. */
    fn send<TEndpoint, TParams, TBody>(
        &self,
        request: SendableRequest<TEndpoint, TParams, TBody>,
    ) -> Self::Response
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<Self::Body> + Send + 'static,
        TParams: Into<Self::Params> + Send + 'static;
}

/**
Represents a type that can fetch request parameters.

A set of request parameters are fetched before each HTTP request.
The `NextParams` trait makes it possible to load balance requests between multiple nodes in an Elasticsearch cluster.
Out of the box `elastic` provides implementations for a static set of nodes or nodes sniffed from the [Nodes Stats API]().
*/
pub trait NextParams: private::Sealed + Clone {
    /**
    The kind of parameters produces.

    This type is designed to link a `NextParams` implementation with a particular `Sender`.
    */
    type Params;

    /** Get a set of request parameters. */
    fn next(&self) -> Self::Params;
}

/**
A single node address.
*/
#[derive(Clone)]
pub struct NodeAddress(Arc<str>);

impl AsRef<str> for NodeAddress {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> From<T> for NodeAddress
where
    T: Into<Arc<str>>,
{
    fn from(address: T) -> Self {
        NodeAddress(address.into().trim_end_matches('/').into())
    }
}

/**
A common container for a source of node addresses.
*/
#[derive(Clone)]
pub struct NodeAddresses<TSender> {
    inner: NodeAddressesInner<TSender>,
}

impl<TSender> NodeAddresses<TSender> {
    pub(crate) fn static_nodes(nodes: StaticNodes) -> Self {
        NodeAddresses {
            inner: NodeAddressesInner::Static(nodes),
        }
    }

    pub(crate) fn sniffed_nodes(nodes: SniffedNodes<TSender>) -> Self {
        NodeAddresses {
            inner: NodeAddressesInner::Sniffed(nodes),
        }
    }
}

impl<TSender> private::Sealed for NodeAddresses<TSender> {}

#[derive(Clone)]
enum NodeAddressesInner<TSender> {
    Static(StaticNodes),
    Sniffed(SniffedNodes<TSender>),
}

pub(crate) enum NodeAddressesBuilder {
    Static(Vec<NodeAddress>),
    Sniffed(SharedStatefulFluentBuilder<NodeAddress, SniffedNodesBuilder>),
}

impl NodeAddressesBuilder {
    pub(crate) fn sniff_nodes(self, builder: SniffedNodesBuilder) -> Self {
        match self {
            NodeAddressesBuilder::Sniffed(fluent_builder) => {
                NodeAddressesBuilder::Sniffed(fluent_builder.value(builder))
            }
            _ => NodeAddressesBuilder::Sniffed(SharedStatefulFluentBuilder::from_value(builder)),
        }
    }

    pub(crate) fn sniff_nodes_fluent<F>(self, address: NodeAddress, fleunt_method: F) -> Self
    where
        F: FnOnce(SniffedNodesBuilder) -> SniffedNodesBuilder + Send + 'static,
    {
        match self {
            NodeAddressesBuilder::Sniffed(fluent_builder) => NodeAddressesBuilder::Sniffed(
                fluent_builder.fluent(address, fleunt_method).shared(),
            ),
            _ => NodeAddressesBuilder::Sniffed(SharedStatefulFluentBuilder::from_fluent(
                address,
                fleunt_method,
            )),
        }
    }
}

impl Default for NodeAddressesBuilder {
    fn default() -> Self {
        NodeAddressesBuilder::Static(vec![DEFAULT_NODE_ADDRESS.into()])
    }
}

impl NodeAddressesBuilder {
    pub(crate) fn build<TSender>(
        self,
        params: PreRequestParams,
        sender: TSender,
    ) -> NodeAddresses<TSender> {
        match self {
            NodeAddressesBuilder::Static(nodes) => {
                let nodes = StaticNodes::round_robin(nodes, params);

                NodeAddresses::static_nodes(nodes)
            }
            NodeAddressesBuilder::Sniffed(builder) => {
                let nodes = builder
                    .into_value(SniffedNodesBuilder::new)
                    .build(params, sender);

                NodeAddresses::sniffed_nodes(nodes)
            }
        }
    }
}
