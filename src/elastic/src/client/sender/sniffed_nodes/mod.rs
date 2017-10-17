/*! Load balanced nodes sniffed regularly from an Elasticsearch cluster. */

mod nodes_info;
use self::nodes_info::*;

use std::sync::{Arc, RwLock};
use futures::{Future, IntoFuture};
use client::sender::static_nodes::StaticNodes;
use client::sender::params::RequestParams;
use client::sender::{SyncSender, AsyncSender, SendableRequest};
use client::requests::NodesInfoRequest;
use client::responses::parse::parse;
use error::Error;

/** 
Periodically sniff nodes in a cluster.

Requests are load balanced between the sniffed nodes using a round-robin strategy.
The base url for the node is obtained by the `http.publish_address` field on a [node info request].
*/
#[derive(Clone)]
pub struct SniffedNodes<TSender> {
    sender: TSender,
    default_params: RequestParams,
    refresh_params: Box<Fn(RequestParams) -> RequestParams>,
    inner: Arc<RwLock<SniffedNodesInner>>,
}

struct SniffedNodesInner {
    refresh: bool,
    refreshing: bool,
    nodes: StaticNodes,
}

impl<TSender> SniffedNodes<TSender> {
    /**
    Create a cluster sniffer with the given base parameters.
    
    The default parameters are returned in cases where there are no addresses available.
    These aren't necessarily the same as the parameters the `Sender` uses internally to sniff the cluster state.
    */
    pub fn new(sender: TSender, default_params: RequestParams) -> Self {
        let builder = default_params.inner.clone();
        let nodes = StaticNodes::round_robin(vec![default_params.base_url.clone()], builder);

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform nodes
        let refresh_params = |params|
            params.url_param("filter_path", "nodes.*.http.publish_address");

        SniffedNodes {
            sender: sender,
            default_params: default_params,
            refresh_params: Box::new(refresh_params),
            inner: Arc::new(RwLock::new(SniffedNodesInner {
                refresh: true,
                refreshing: false,
                nodes: nodes
            })),
        }
    }

    fn should_refresh(&self) -> bool {
        let mut inner = self.inner.borrow_mut();

        if !inner.refreshing && inner.refresh {
            inner.refresh = false;
            true
        } else {
            false
        }
    }
}

// TODO: Share most of this logic

impl SniffedNodes<AsyncSender> {
    /** Get the next address for a request. */
    pub fn next(&self) -> Box<Future<Item = RequestParams, Error = Error>> {
        if !self.should_refresh() {
            let address = Ok(self.inner.borrow().nodes.try_next().unwrap_or_else(|| self.default_params.clone())).into_future();
            Box::new(address)
        } else {
            let inner = self.inner.clone();
            {
                inner.borrow_mut().refreshing = true;
            }

            let default_params = self.default_params.clone();

            // TODO: Make this more resilient to failure
            // TODO: Ensure we only have 1 refresh happening at a time (extra refreshing property)
            let req = SendableRequest::new(NodesInfoRequest::new(), self.params_builder.clone());

            let refresh_nodes = self.sender
                .send(req)
                .and_then(|res| parse::<NodesInfoResponse>().from_response(res))
                .and_then(move |parsed| {
                    let mut inner = inner.borrow_mut();

                    inner.nodes.nodes = parsed
                        .into_iter()
                        .filter_map(|node| node.http
                            .and_then(|http| http.publish_address)
                            .map(|publish_address| Arc::<str>::from(publish_address)))
                        .collect();

                    inner.refreshing = false;

                    Ok(inner.nodes.try_next().unwrap_or(default_params))
                });

            Box::new(refresh_nodes)
        }
    }
}

impl SniffedNodes<SyncSender> {
    /** Get the next address for a request. */
    pub fn next(&self) -> Result<RequestParams, Error> {
        if !self.should_refresh() {
            Ok(self.inner.borrow().nodes.try_next().unwrap_or_else(|| self.default_params.clone()))
        } else {
            let inner = self.inner.clone();
            {
                inner.borrow_mut().refreshing = true;
            }

            let default_params = self.default_params.clone();

            // TODO: Make this more resilient to failure
            // TODO: Ensure we only have 1 refresh happening at a time (extra refreshing property)
            let req = SendableRequest::new(NodesInfoRequest::new(), self.params_builder.clone());

            let refresh_nodes = self.sender
                .send(req)
                .and_then(|res| parse::<NodesInfoResponse>().from_response(res))
                .and_then(move |parsed| {
                    let mut inner = inner.borrow_mut();

                    inner.nodes.nodes = parsed
                        .into_iter()
                        .filter_map(|node| node.http
                            .and_then(|http| http.publish_address)
                            .map(|publish_address| Arc::<str>::from(publish_address)))
                        .collect();

                    inner.refreshing = false;

                    Ok(inner.nodes.try_next().unwrap_or(default_params))
                });

            Box::new(refresh_nodes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests::*;

    #[test]
    fn sync_sniffed_nodes_is_send_sync() {
        assert_send::<SniffedNodes<SyncSender>>();
        assert_sync::<SniffedNodes<AsyncSender>>();
    }
}
