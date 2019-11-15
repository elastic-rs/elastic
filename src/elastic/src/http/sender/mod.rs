/*!
Low-level http client details for request sending.

This module contains low-level implementation details for the high-level [`Client`][Client].
Most of the types here aren't currently usable directly, but once the low-level API matures they should be extensible.
Some notable types include:

- `Sender`: a generic trait that can send a http request and return a response
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

mod params;
pub use self::params::*;

#[cfg(feature="async_sender")]
mod asynchronous;
#[cfg(feature="sync_sender")]
mod synchronous;
#[cfg(feature="async_sender")]
pub use self::asynchronous::*;
#[cfg(feature="sync_sender")]
pub use self::synchronous::*;

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
    client::requests::RequestInner,
    endpoints::Endpoint,
    error::Error,
};

/**
A sendable request.

This type encapsulates the state needed between a [`Client`][Client] and a [`Sender`][Sender] in order to send a request.

[Client]: ../struct.Client.html
[Sender]: trait.Sender.html
*/
pub struct SendableRequest<TEndpoint, TParams, TBody> {
    /** Unique ID for the request. */
    pub correlation_id: Uuid,
    /** Endpoint for the request */
    pub inner: TEndpoint,
    /** Parameters for the request */
    pub params: SendableRequestParams<TParams>,
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

/** Parameters for a SendableRequest */
pub enum SendableRequestParams<TParams> {
    /** Parameters were explicitly defined for this specific request */
    Value(RequestParams),
    /** Paremeters weren't explicitly defined, so they must be built */
    Builder {
        /** Base parameters */
        params: TParams,
        /** Builder for the parameters */
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
pub trait Sender: Clone {
    /** The kind of request body this sender accepts. */
    type Body;
    /** The kind of response this sender produces. */
    type Response;
    /** The kind of request parameters this sender accepts. */
    type Params: Send+'static;

    /** Send a raw request. */
    fn send<TEndpoint, TParams, TBody>(
        &self,
        request: SendableRequest<TEndpoint, TParams, TBody>,
    ) -> Self::Response
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<Self::Body> + Send + 'static,
        TParams: Into<Self::Params> + Send + 'static;

    /**
    Gets the parameters for the next query.

    A set of request parameters are fetched before each HTTP request. This method
    makes it possible to load balance requests between multiple nodes in an Elasticsearch
    cluster. Out of the box elastic provides implementations for a static set of nodes or
    nodes sniffed from the Nodes Stats API.
    */
    fn next_params(
        &self,
        addresses: &NodeAddresses,
    ) -> Self::Params;
}

/**
Represents a type that can send a typed request and deserialize the result.

You probably don't need to touch this trait directly.
See the [`Client`][Client] type for making requests.

Senders should implement this for every type implementing `RequestInner`, for example
via `impl<T: RequestInner> TypedSender<T> for MySender`.

In the future, when [generic associated types][gna] are stabilized, this trait should be
folded into the `Sender` trait, using a generic associated type for the response object
(ex. `type TypedResponse<TReqInner>`).

[gna]: https://rust-lang.github.io/rfcs/1598-generic_associated_types.html
*/
pub trait TypedSender<TReqInner>: Sender
where
    TReqInner: RequestInner,
{
    
    /**
    Response object containing the deserialized result or an error.

    This is very generic to allow flexibility between sync and async implementations,
    but should ideally be some type using the `TReqInner::Response` type.

    For `SyncSender`, this is `Result<TReqInner::Response, elastic::error::Error>`.
    For `AsyncSender`, this is a type that implements `Future<Item=TReqInner::Response, elastic::error::Error>`.
    */
    type TypedResponse;

    
    /**
    Sends a request and deserializes the result.

    The caller is responsible for converting the request object (`TEndpoint::Request`)
    to a `SendableRequest`. The caller passes either the sendable request or an error if
    the conversion failed. If it's an error, the implementation should return an appropriate
    error type (Ex. `Err` for `SyncSender` or a pre-failed future for `AsyncSender`).
    */
    fn typed_send<TParams, TEndpoint, TBody>(
        &self,
        request: Result<SendableRequest<TEndpoint, TParams, TBody>, Error>,
    ) -> Self::TypedResponse
    where
        TEndpoint: Into<Endpoint<'static, TBody>> + Send + 'static,
        TBody: Into<Self::Body> + Send + 'static,
        TParams: Into<Self::Params> + Send + 'static;
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
pub enum NodeAddresses {
    /** Static list of nodes */
    Static(StaticNodes),
    /** Fetch set of nodes from a single node of the cluster */
    Sniffed(SniffedNodes),
}

impl NodeAddresses {
    /** Static set of nodes to connect to */
    pub fn static_nodes(nodes: StaticNodes) -> Self {
        NodeAddresses::Static(nodes)
    }

    /** Fetch set of nodes from a single node of the cluster */
    pub fn sniffed_nodes(nodes: SniffedNodes) -> Self {
        NodeAddresses::Sniffed(nodes)
    }
}

/** Builder for `NodeAddresses` */
pub enum NodeAddressesBuilder {
    /** Static list of nodes */
    Static(Vec<NodeAddress>),
    /** Fetch set of nodes from a single node of the cluster */
    Sniffed(SharedStatefulFluentBuilder<NodeAddress, SniffedNodesBuilder>),
}

impl NodeAddressesBuilder {
    /** Sniff nodes */
    pub fn sniff_nodes(self, builder: SniffedNodesBuilder) -> Self {
        match self {
            NodeAddressesBuilder::Sniffed(fluent_builder) => {
                NodeAddressesBuilder::Sniffed(fluent_builder.value(builder))
            }
            _ => NodeAddressesBuilder::Sniffed(SharedStatefulFluentBuilder::from_value(builder)),
        }
    }

    /** Sniff nodes */
    pub fn sniff_nodes_fluent<F>(self, address: NodeAddress, fleunt_method: F) -> Self
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
    /** Builds the node addresses */
    pub fn build(
        self,
        params: PreRequestParams,
    ) -> NodeAddresses {
        match self {
            NodeAddressesBuilder::Static(nodes) => {
                let nodes = StaticNodes::round_robin(nodes, params);

                NodeAddresses::static_nodes(nodes)
            }
            NodeAddressesBuilder::Sniffed(builder) => {
                let nodes = builder
                    .into_value(SniffedNodesBuilder::new)
                    .build(params);

                NodeAddresses::sniffed_nodes(nodes)
            }
        }
    }
}
