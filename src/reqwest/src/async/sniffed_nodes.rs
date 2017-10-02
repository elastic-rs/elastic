/*! Connection sniffing and multiple static nodes. */

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use reqwest::unstable::async::Client;
use futures::{Future, IntoFuture};
use ::{Error, RequestParams};
use async::{AsyncElasticClient, AsyncFromResponse};
use async::nodes_info::NodesInfoResponse;
use static_nodes::StaticNodes;
use req::NodesInfoRequest;
use res::parsing::parse;

/** 
Periodically sniff nodes in a cluster.

Requests are load balanced between the sniffed nodes using a round-robin strategy.
The base url for the node is obtained by the `http.publish_address` field on a [node info request].
*/
#[derive(Clone)]
pub struct SniffedNodes {
    client: Client,
    default_params: RequestParams,
    refresh_params: RequestParams,
    inner: Rc<RefCell<SniffedNodesInner>>,
}

struct SniffedNodesInner {
    refresh: bool,
    refreshing: bool,
    nodes: StaticNodes,
}

impl SniffedNodes {
    /** Create a cluster sniffer with the given base parameters. */
    pub fn new(client: Client, params: RequestParams) -> Self {
        let builder = params.inner.clone();
        let nodes = StaticNodes::round_robin(vec![params.base_url.clone()], builder);

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform nodes
        let refresh_params = params
            .clone()
            .url_param("filter_path", "nodes.*.http.publish_address");

        SniffedNodes {
            client: client,
            default_params: params,
            refresh_params: refresh_params,
            inner: Rc::new(RefCell::new(SniffedNodesInner {
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
            let refresh_nodes = self.client
                .elastic_req(&self.refresh_params, NodesInfoRequest::new())
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
