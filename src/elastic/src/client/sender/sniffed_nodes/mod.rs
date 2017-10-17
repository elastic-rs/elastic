/*! Load balanced nodes sniffed regularly from an Elasticsearch cluster. */

mod nodes_info;
use self::nodes_info::*;

use std::sync::{Arc, RwLock};
use futures::{Future, IntoFuture};
use client::sender::static_nodes::StaticNodes;
use client::sender::{Sender, SyncSender, AsyncSender, SendableRequest, RequestParams, NextParams};
use client::requests::NodesInfoRequest;
use error::Error;
use private;

/** 
Periodically sniff nodes in a cluster.

Requests are load balanced between the sniffed nodes using a round-robin strategy.
The base url for the node is obtained by the `http.publish_address` field on a [node info request].
*/
#[derive(Clone)]
pub struct SniffedNodes<TSender> {
    sender: TSender,
    default_params: RequestParams,
    refresh_params_builder: Arc<Fn(RequestParams) -> RequestParams>,
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
        let (base_url, builder) = default_params.clone().split();
        let nodes = StaticNodes::round_robin(vec![base_url], builder);

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform nodes
        let refresh_params_builder = |params: RequestParams|
            params.url_param("filter_path", "nodes.*.http.publish_address");

        SniffedNodes {
            sender: sender,
            default_params: default_params,
            refresh_params_builder: Arc::new(refresh_params_builder),
            inner: Arc::new(RwLock::new(SniffedNodesInner {
                refresh: true,
                refreshing: false,
                nodes: nodes
            })),
        }
    }

    /**
    Check whether the addresses should be refreshed.

    If the addresses should be refreshed, and we're not currently refreshing then we set `refresh` to `false` and `refreshing` to `true`.
    This method should only be called by `next`, which will always set `refreshing` back to `false` when it's finished.
    */
    fn should_refresh(&self) -> bool {
        let mut inner = self.inner.write().expect("lock poisoned");

        if !inner.refreshing && inner.refresh {
            inner.refresh = false;
            inner.refreshing = true;
            true
        } else {
            false
        }
    }
}

impl<TSender> private::Sealed for SniffedNodes<TSender> { }

// TODO: Share most of this logic
impl NextParams for SniffedNodes<AsyncSender> {
    type Params = Box<Future<Item = RequestParams, Error = Error>>;

    fn next(&self) -> Self::Params {
        if !self.should_refresh() {
            let inner = self.inner.read().expect("lock poisoned");
            let address = Ok(inner.nodes.next().unwrap_or_else(|_| self.default_params.clone())).into_future();

            Box::new(address)
        } else {
            let default_params = self.default_params.clone();

            // TODO: Make this more resilient to failure
            // TODO: Ensure we only have 1 refresh happening at a time (extra refreshing property)
            let req = SendableRequest::new(
                NodesInfoRequest::new(),
                self.default_params.clone(),
                Some(self.refresh_params_builder.clone()));

            let send_inner = self.inner.clone();
            let finally_inner = self.inner.clone();

            let refresh_nodes = self.sender
                .send(req)
                .and_then(|res| res.into_response::<NodesInfoResponse>())
                .and_then(move |parsed| {
                    let mut inner = send_inner.write().expect("lock poisoned");

                    inner.nodes.nodes = parsed
                        .into_iter()
                        .filter_map(|node| node.http
                            .and_then(|http| http.publish_address)
                            .map(|publish_address| Arc::<str>::from(publish_address)))
                        .collect();

                    Ok(inner.nodes.next().unwrap_or(default_params))
                })
                .then(move |res| {
                    let mut inner = finally_inner.write().expect("lock poisoned");
                    inner.refreshing = false;

                    res
                });

            Box::new(refresh_nodes)
        }
    }
}

impl NextParams for SniffedNodes<SyncSender> {
    type Params = Result<RequestParams, Error>;

    fn next(&self) -> Self::Params {
        unimplemented!()
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
