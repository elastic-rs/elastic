/*! Load balanced nodes sniffed regularly from an Elasticsearch cluster. */

/*
This implementation currently has a few limitations that aren't great long term:

- Requests are blocked by refreshes when they're deemed necessary
- If refreshing fails for any reason then the entire request fails

We could potentially work around these limitations this way:

Asynchronously:

- Instead of returning a future to refresh, store the future on the sniffer
- When fetching refresh params, poll this future if it's `Some` (or check if it should be if it's `None`)
  - If the future returns `Ready(params)` return those and set the inner future to `None`
  - If the future returns `NotReady` return the default params

Synchronously:

- Offload the request to update parameters to a thread pool

This means our `SniffedNodes` structure looks completely different in synchronous and asynchronous scenarios.
It's effectively a rewrite.
*/

use std::{
    sync::{
        Arc,
        RwLock,
    },
    time::{
        Duration,
        Instant,
    },
};
use url::Url;

use crate::{
    client::responses::NodesInfoResponse,
    endpoints::NodesInfoRequest,
    error::{
        self,
        Error,
    },
    http::{
        sender::{
            static_nodes::StaticNodes,
            NodeAddress,
            PreRequestParams,
            RequestParams,
            SendableRequest,
            SendableRequestParams,
        },
        DefaultBody,
    },
    private,
};

/**
Periodically sniff nodes in a cluster.

Requests are load balanced between the sniffed nodes using a round-robin strategy.
The base url for the node is obtained by the `http.publish_address` field on a [node info request].

Nodes are refreshed on the next request after the specified timeout.
If updating the nodes fails for some reason then the request itself will also fail.

[node info request]: https://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-nodes-info.html
*/
#[derive(Clone)]
pub struct SniffedNodes<TSender> {
    sender: TSender,
    refresh_params: RequestParams,
    inner: Arc<RwLock<SniffedNodesInner>>,
}

/**
A builder for a cluster sniffer.
*/
pub struct SniffedNodesBuilder {
    base_url: NodeAddress,
    wait: Option<Duration>,
}

struct SniffedNodesInner {
    last_update: Option<Instant>,
    wait: Duration,
    refreshing: bool,
    nodes: StaticNodes,
}

impl SniffedNodesBuilder {
    /**
    Create a new `SniffedNodesBuilder` with the given base address.
    */
    pub fn new<I>(address: I) -> Self
    where
        I: Into<NodeAddress>,
    {
        SniffedNodesBuilder {
            base_url: address.into(),
            wait: None,
        }
    }

    /**
    Specify a given base address.
    */
    pub fn base_url<I>(mut self, address: I) -> Self
    where
        I: Into<NodeAddress>,
    {
        self.base_url = address.into();
        self
    }

    /**
    Specify a minimum duration to wait before refreshing the set of node addresses.
    */
    pub fn wait(mut self, wait: Duration) -> Self {
        self.wait = Some(wait);
        self
    }

    /**
    Build a cluster sniffer using the given sender and parameters.

    A `filter_path` url parameter will be added to the `refresh_parameters`.
    */
    pub fn build<TSender>(
        self,
        base_params: PreRequestParams,
        sender: TSender,
    ) -> SniffedNodes<TSender> {
        let nodes = StaticNodes::round_robin(vec![self.base_url.clone()], base_params.clone());
        let wait = self.wait.unwrap_or_else(|| Duration::from_secs(90));

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform nodes
        let refresh_params = RequestParams::from_parts(self.base_url, base_params)
            .url_param("filter_path", "nodes.*.http.publish_address");

        SniffedNodes {
            sender: sender,
            refresh_params: refresh_params,
            inner: Arc::new(RwLock::new(SniffedNodesInner {
                last_update: None,
                wait: wait,
                refreshing: false,
                nodes: nodes,
            })),
        }
    }
}

impl<T> From<T> for SniffedNodesBuilder
where
    T: Into<NodeAddress>,
{
    fn from(address: T) -> Self {
        SniffedNodesBuilder::new(address)
    }
}

/*
Shared logic between sync and async methods.

These methods definitely aren't intended to be made public.
There are invariants that are shared between them that require they're called in specific ways.
*/
impl<TSender> SniffedNodes<TSender> {
    /** Gets the sender */
    pub fn sender(&self) -> &TSender {
        &self.sender
    }
    
    /**
    Return a node address if the set of nodes is still current.

    If this method returns `Some` then the set of nodes is current and an address is returned.
    If this method returns `None` then eventually call `finish_refresh`.
    */
    pub fn next_or_start_refresh(&self) -> Result<NextOrRefresh, Error> {
        // Attempt to get an address using only a read lock first
        {
            let inner = self.inner.read().expect("lock poisoned");

            if !inner.should_refresh() {
                // Return the next address without refreshing
                let address = inner.nodes.next().map_err(error::request)?;

                return Ok(NextOrRefresh::Next(address));
            }
        }

        // Attempt to refresh using a write lock otherwise
        {
            let mut inner = self.inner.write().expect("lock poisoned");

            if inner.refreshing {
                // Return the next address without refreshing
                // This is unlikely but it's possible that a write lock
                // gets acquired after another thread kicks off a refresh.
                // In that case we don't want to do another one.
                let address = inner.nodes.next().map_err(error::request)?;

                return Ok(NextOrRefresh::Next(address));
            } else {
                inner.refreshing = true;

                return Ok(NextOrRefresh::NeedsRefresh(Refresher {
                    inner: Arc::clone(&self.inner),
                    refresh_params: self.refresh_params.clone(),
                }));
            }
        }
    }

    /**
    Creates a `SendableRequest` for fetching node information.
    */
    pub fn sendable_request(
        &self,
    ) -> SendableRequest<NodesInfoRequest<'static>, RequestParams, DefaultBody> {
        SendableRequest::new(
            NodesInfoRequest::new(),
            SendableRequestParams::Value(self.refresh_params.clone()),
        )
    }
}

impl SniffedNodesInner {
    fn should_refresh(&self) -> bool {
        // If there isn't a value for the last update then assume we need to refresh.
        let last_update_is_stale = self
            .last_update
            .as_ref()
            .map(|last_update| last_update.elapsed() > self.wait);

        !self.refreshing && last_update_is_stale.unwrap_or(true)
    }

    fn update_nodes_and_next(
        &mut self,
        parsed: NodesInfoResponse,
        scheme: &str,
    ) -> Result<RequestParams, Error> {
        let nodes: Vec<_> = parsed
            .into_iter_addrs()
            .map(|publish_address| format!("{}://{}", scheme, publish_address).into())
            .collect();

        self.nodes.set(nodes)?;
        self.nodes.next().map_err(error::request)
    }
}

impl<TSender> private::Sealed for SniffedNodes<TSender> {}

/** Result of `SniffedNodes::next_or_start_refresh`. */
pub enum NextOrRefresh {
    /** Don't need to refresh, an address is available. */
    Next(RequestParams),
    /**
    Need to refresh. Sender should send a request and submit the
    results to the `Refresher` in this object.
    */
    NeedsRefresh(Refresher),
}

/**
Returned by `SniffedNodes::next_or_start_refresh` when a refresh is needed.

Senders should send a request to an existing node to get the new node list, then
pass the results to `update_nodes_and_next`, which consumes the object and updates
the node list.
*/
pub struct Refresher {
    inner: Arc<RwLock<SniffedNodesInner>>,
    refresh_params: RequestParams,
}
impl Refresher {
    /**
    Updates the node list of the `SniffedNodes` object.

    Senders should call this when the request for nodes comes in.
    */
    pub fn update_nodes_and_next(
        self,
        fresh_nodes: Result<NodesInfoResponse, Error>,
    ) -> Result<RequestParams, Error> {
        let mut inner = self.inner.write().expect("lock poisoned");

        inner.refreshing = false;

        // TODO: We need to deal with the scheme better here
        // The `NodeAddress` should one day be a properly typed url we can interrogate
        let parsed_url =
            Url::parse(self.refresh_params.get_base_url().as_ref()).map_err(error::request)?;
        let scheme = parsed_url.scheme();

        let fresh_nodes = fresh_nodes?;
        let next = inner.update_nodes_and_next(fresh_nodes, scheme)?;

        inner.last_update = Some(Instant::now());

        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn sender() -> SniffedNodes<()> {
        SniffedNodesBuilder::new(initial_address()).build(PreRequestParams::default(), ())
    }

    fn expected_nodes() -> NodesInfoResponse {
        serde_json::from_value(json!({
            "nodes": {
                "node1": {
                    "http": {
                        "publish_address": "a:9200"
                    }
                },
                "node2": {
                    "http": {
                        "publish_address": "127.0.0.1:9200"
                    }
                }
            }
        }))
        .unwrap()
    }

    fn empty_nodes() -> NodesInfoResponse {
        serde_json::from_value(json!({ "nodes": { } })).unwrap()
    }

    fn initial_address() -> &'static str {
        "http://initial:9200"
    }

    fn expected_addresses() -> Vec<&'static str> {
        vec!["http://a:9200", "http://127.0.0.1:9200"]
    }

    fn assert_node_addresses_equal(
        nodes: &SniffedNodes<()>,
        expected_addresses: Vec<&'static str>,
    ) {
        let inner = nodes.inner.read().expect("lock poisoned");
        let actual: Vec<&str> = inner.nodes.get().iter().map(|node| node.as_ref()).collect();

        assert_eq!(expected_addresses, actual);
    }

    fn assert_refreshing_equal(nodes: &SniffedNodes<()>, refreshing: bool) {
        let inner = nodes.inner.read().expect("lock poisoned");
        assert_eq!(refreshing, inner.refreshing);
    }

    fn assert_should_refresh_equal(nodes: &SniffedNodes<()>, should_refresh: bool) {
        let inner = nodes.inner.read().expect("lock poisoned");
        assert_eq!(should_refresh, inner.should_refresh());
    }

    #[test]
    fn should_refresh_is_true_initially() {
        let nodes = sender();

        assert_should_refresh_equal(&nodes, true);
    }

    #[test]
    fn should_refresh_is_false_while_refreshing() {
        let nodes = sender();
        {
            let mut inner = nodes.inner.write().expect("lock poisoned");
            inner.refreshing = true;
        }

        assert_should_refresh_equal(&nodes, false);
    }

    #[test]
    fn refresh_success() {
        let nodes = sender();
        let nodes_while_refreshing = nodes.clone();

        let refresher = match nodes.next_or_start_refresh() {
            Ok(NextOrRefresh::NeedsRefresh(r)) => r,
            Ok(NextOrRefresh::Next(_)) => {
                panic!("Expected to refresh here, got an address instead");
            },
            Err(err) => {
                panic!("Expected to refresh here, got an error instead: {:?}", err);
            },
        };

        assert_refreshing_equal(&nodes_while_refreshing, true);
        let res = refresher.update_nodes_and_next(Ok(expected_nodes()));
        assert!(res.is_ok());

        assert_node_addresses_equal(&nodes, expected_addresses());
        assert_refreshing_equal(&nodes, false);
        assert_should_refresh_equal(&nodes, false);
    }

    #[test]
    fn refresh_fail_on_empty() {
        let nodes = sender();

        let refresher = match nodes.next_or_start_refresh() {
            Ok(NextOrRefresh::NeedsRefresh(r)) => r,
            Ok(NextOrRefresh::Next(_)) => {
                panic!("Expected to refresh here, got an address instead");
            },
            Err(err) => {
                panic!("Expected to refresh here, got an error instead: {:?}", err);
            },
        };

        let res = refresher.update_nodes_and_next(Ok(empty_nodes()));
        assert!(res.is_err());

        assert_node_addresses_equal(&nodes, vec![initial_address()]);
        assert_refreshing_equal(&nodes, false);
        assert_should_refresh_equal(&nodes, true);
    }

    #[test]
    fn refresh_fail_on_request() {
        let nodes = sender();

        let refresher = match nodes.next_or_start_refresh() {
            Ok(NextOrRefresh::NeedsRefresh(r)) => r,
            Ok(NextOrRefresh::Next(_)) => {
                panic!("Expected to refresh here, got an address instead");
            },
            Err(err) => {
                panic!("Expected to refresh here, got an error instead: {:?}", err);
            },
        };

        let res = refresher.update_nodes_and_next(Err(error::test()));
        assert!(res.is_err());

        assert_node_addresses_equal(&nodes, vec![initial_address()]);
        assert_refreshing_equal(&nodes, false);
        assert_should_refresh_equal(&nodes, true);
    }

}
