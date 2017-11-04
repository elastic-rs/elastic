/*! Load balanced nodes sniffed regularly from an Elasticsearch cluster. */

mod nodes_info;
use self::nodes_info::*;

use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};
use futures::{Future, IntoFuture};
use client::sender::static_nodes::StaticNodes;
use client::sender::{Sender, SyncSender, AsyncSender, SendableRequest, RequestParams, NextParams};
use client::requests::{NodesInfoRequest, DefaultBody};
use error::{self, Error};
use private;

/**
Periodically sniff nodes in a cluster.

Requests are load balanced between the sniffed nodes using a round-robin strategy.
The base url for the node is obtained by the `http.publish_address` field on a [node info request].
*/
#[derive(Clone)]
pub struct SniffedNodes<TSender> {
    sender: TSender,
    refresh_params: RequestParams,
    inner: Arc<RwLock<SniffedNodesInner>>,
}

// Lock<Updating>

// Request should only check 

// LastUpdatedTime
// CurrentTime
// If !Updating && CurrentTime > LastUpdatedTime + Wait
//   Set LastUpdatedTime = CurrentTime
//   Set Updating

// Get CurrentTime
// Get Read Lock
//   Check Refreshing + LastUpdatedTime + Wait
//   No Refresh: return next
//   Refresh: drop read lock
// Get Write Lock
//   Check Refreshing
//   No Refresh: return next
//   Refresh:
//     Set Refreshing = True
//     Execute Refresh
//     Set Refreshing = False
//     return next

struct SniffedNodesInner {
    last_update: Option<Instant>,
    wait: Duration,
    refreshing: bool,
    nodes: StaticNodes,
}

impl SniffedNodesInner {
    fn should_refresh(&self) -> bool {
        // If there isn't a value for the last update then assume we need to refresh.
        let last_update_is_stale = self.last_update.as_ref().map(|last_update| last_update.elapsed() > self.wait);

        !self.refreshing && last_update_is_stale.unwrap_or(true)
    }

    fn update_nodes_and_next(&mut self, parsed: NodesInfoResponse) -> Result<RequestParams, Error> {
        self.nodes.nodes = parsed
            .into_iter()
            .filter_map(|node| node.http
                .and_then(|http| http.publish_address)
                .map(|publish_address| Arc::<str>::from(publish_address)))
            .collect();

        self.nodes.next().map_err(error::request)
    }
}

impl<TSender> SniffedNodes<TSender> {
    /**
    Create a cluster sniffer with the given base parameters.

    The default parameters are returned in cases where there are no addresses available.
    These aren't necessarily the same as the parameters the `Sender` uses internally to sniff the cluster state.
    */
    pub fn new(sender: TSender, refresh_params: RequestParams) -> Self {
        let nodes = {
            let (base_url, builder) = refresh_params.clone().split();
            StaticNodes::round_robin(vec![base_url], builder)
        };

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform nodes
        SniffedNodes {
            sender: sender,
            refresh_params: refresh_params.url_param("filter_path", "nodes.*.http.publish_address"),
            inner: Arc::new(RwLock::new(SniffedNodesInner {
                last_update: None,
                wait: Duration::from_secs(30),
                refreshing: false,
                nodes: nodes
            })),
        }
    }

    /**
    Get the next async address or refresh.

    This method takes a generic function that will resolve to a new set of node addresses.
    */
    fn async_next<TRefresh, TRefreshFuture>(&self, refresh: TRefresh) -> Box<Future<Item = RequestParams, Error = Error>>
    where
        TRefresh: Fn(SendableRequest<NodesInfoRequest<'static>, RequestParams, DefaultBody>) -> TRefreshFuture,
        TRefreshFuture: Future<Item = NodesInfoResponse, Error = Error> + 'static,
    {
        if let Some(address) = self.next_or_start_refresh() {
            return Box::new(address.into_future());
        }

        // Perform the refresh
        let inner = self.inner.clone();
        let req = self.sendable_request();

        let refresh_nodes = refresh(req)
            .then(move |fresh_nodes| Self::finish_refresh(&inner, fresh_nodes));

        Box::new(refresh_nodes)
    }

    /**
    Get the next sync address or refresh.

    This method takes a generic function that will resolve to a new set of node addresses.
    */
    fn sync_next<TRefresh>(&self, refresh: TRefresh) -> Result<RequestParams, Error>
    where
        TRefresh: Fn(SendableRequest<NodesInfoRequest<'static>, RequestParams, DefaultBody>) -> Result<NodesInfoResponse, Error>,
    {
        if let Some(address) = self.next_or_start_refresh() {
            return address;
        }

        // Perform the refresh
        let req = self.sendable_request();

        let fresh_nodes = refresh(req);
        Self::finish_refresh(&self.inner, fresh_nodes)
    }
}

/*
Shared logic between sync and async methods.

These methods definitely aren't intended to be made public.
There are invariants that are shared between them that require they're called in specific ways.
*/
impl<TSender> SniffedNodes<TSender> {
    /**
    Return a node address if the set of nodes is still current.

    If this method returns `Some` then the set of nodes is current and an address is returned.
    If this method returns `None` then eventually call `finish_refresh`.
    */
    fn next_or_start_refresh(&self) -> Option<Result<RequestParams, Error>> {
        // Attempt to get an address using only a read lock first
        let read_fresh = {
            let inner = self.inner.read().expect("lock poisoned");

            if !inner.should_refresh() {
                // Return the next address without refreshing
                let address = inner.nodes.next().map_err(error::request);

                Some(address)
            }
            else {
                None
            }
        };

        // Attempt to refresh using a write lock otherwise
        read_fresh.or_else(|| {
            let mut inner = self.inner.write().expect("lock poisoned");

            if inner.refreshing {
                // Return the next address without refreshing
                // This is unlikely but it's possible that a write lock
                // gets acquired after another thread kicks off a refresh.
                // In that case we don't want to do another one.
                let address = inner.nodes.next().map_err(error::request);

                Some(address)
            }
            else {
                inner.refreshing = true;

                None
            }
        })
    }

    fn sendable_request(&self) -> SendableRequest<NodesInfoRequest<'static>, RequestParams, DefaultBody> {
        SendableRequest::new(
            NodesInfoRequest::new(),
            self.refresh_params.clone(),
            None)
    }

    fn finish_refresh(inner: &RwLock<SniffedNodesInner>, fresh_nodes: Result<NodesInfoResponse, Error>) -> Result<RequestParams, Error> {
        let mut inner = inner.write().expect("lock poisoned");

        inner.refreshing = false;
        inner.last_update = Some(Instant::now());

        match fresh_nodes {
            Ok(parsed) => inner.update_nodes_and_next(parsed),
            Err(e) => Err(e)
        }
    }
}

impl<TSender> private::Sealed for SniffedNodes<TSender> { }

impl NextParams for SniffedNodes<AsyncSender> {
    type Params = Box<Future<Item = RequestParams, Error = Error>>;

    fn next(&self) -> Self::Params {
        self.async_next(|req| self.sender.send(req).and_then(|res| res.into_response::<NodesInfoResponse>()))
    }
}

impl NextParams for SniffedNodes<SyncSender> {
    type Params = Result<RequestParams, Error>;

    fn next(&self) -> Self::Params {
        self.sync_next(|req| self.sender.send(req).and_then(|res| res.into_response::<NodesInfoResponse>()))
    }
}

#[cfg(test)]
mod tests {
    use futures::Future;
    use super::*;

    fn sender() -> SniffedNodes<()> {
        SniffedNodes::new((), RequestParams::default().base_url(initial_address()))
    }

    fn expected_nodes() -> NodesInfoResponse {
        NodesInfoResponse {
            nodes: expected_addresses().into_iter()
                .map(|address| {
                    SniffedNode {
                        http: Some(SniffedNodeHttp {
                            publish_address: Some(address.to_owned())
                        })
                    }
                })
                .collect()
        }
    }

    fn initial_address() -> &'static str {
        "http://initial:9200"
    }

    fn expected_addresses() -> Vec<&'static str> {
        vec![
            "http://a:9200",
            "http://b:9200",
        ]
    }

    fn assert_node_addresses_equal(nodes: &SniffedNodes<()>, expected_addresses: Vec<&'static str>) {
        let inner = nodes.inner.read().expect("lock poisoned");
        let actual: Vec<&str> = inner.nodes.nodes.iter().map(|node| node.as_ref()).collect();

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
    fn async_refresh_success() {
        let nodes = sender();
        let nodes_while_refreshing = nodes.clone();

        let res = nodes.async_next(move |_| {
            assert_refreshing_equal(&nodes_while_refreshing, true);

            Ok(expected_nodes()).into_future()
        })
        .wait();

        assert!(res.is_ok());

        assert_node_addresses_equal(&nodes, expected_addresses());
        assert_refreshing_equal(&nodes, false);
    }

    #[test]
    fn async_refresh_fail() {
        let nodes = sender();
        let nodes_while_refreshing = nodes.clone();

        let res = nodes.async_next(move |_| {
            assert_refreshing_equal(&nodes_while_refreshing, true);

            Err(error::test()).into_future()
        })
        .wait();

        assert!(res.is_err());

        assert_node_addresses_equal(&nodes, vec![initial_address()]);
        assert_refreshing_equal(&nodes, false);
    }

    #[test]
    fn sync_refresh_success() {
        let nodes = sender();
        let nodes_while_refreshing = nodes.clone();

        let res = nodes.sync_next(move |_| {
            assert_refreshing_equal(&nodes_while_refreshing, true);

            Ok(expected_nodes())
        });

        assert!(res.is_ok());

        assert_node_addresses_equal(&nodes, expected_addresses());
        assert_refreshing_equal(&nodes, false);
    }

    #[test]
    fn sync_refresh_fail() {
        let nodes = sender();
        let nodes_while_refreshing = nodes.clone();

        let res = nodes.sync_next(move |_| {
            assert_refreshing_equal(&nodes_while_refreshing, true);

            Err(error::test())
        });

        assert!(res.is_err());

        assert_node_addresses_equal(&nodes, vec![initial_address()]);
        assert_refreshing_equal(&nodes, false);
    }
}
