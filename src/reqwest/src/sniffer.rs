/*! Connection sniffing and multiple static addresses. */

#![allow(warnings)]

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use reqwest::unstable::async::Client;
use futures::{Future, IntoFuture};
use {Error, RequestParams, RequestParamsBuilder};
use async::AsyncElasticClient;
use req::NodesInfoRequest;

/** Select a base address for a given request using some strategy. */
pub struct MultipleAddresses<TStrategy> {
    addresses: Vec<Arc<str>>,
    strategy: TStrategy,
    params_builder: RequestParamsBuilder,
}

impl<TStrategy> MultipleAddresses<TStrategy>
where
    TStrategy: Strategy,
{
    /** Get the next address for a request. */
    pub fn next(&self) -> RequestParams {
        let address = self.strategy.next(&self.addresses);
        RequestParams::from_parts(address, self.params_builder.clone())
    }
}

impl MultipleAddresses<RoundRobin> {
    /** Use a round-robin strategy for balancing traffic over the given set of addresses. */
    pub fn round_robin<I, S>(addresses: I, params_builder: RequestParamsBuilder) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let addresses = addresses.into_iter().map(|a| a.as_ref().into()).collect();
        let strategy = RoundRobin::default();

        MultipleAddresses {
            addresses: addresses,
            strategy: strategy,
            params_builder: params_builder,
        }
    }
}

/** The strategy selects an address from a given collection. */
pub trait Strategy: Send + Sync {
    /** Get the next address. */
    fn next(&self, addresses: &[Arc<str>]) -> Arc<str>;
}

/** A round-robin strategy cycles through addresses sequentially. */
pub struct RoundRobin {
    index: Arc<AtomicUsize>,
}

impl Default for RoundRobin {
    fn default() -> Self {
        RoundRobin {
            index: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl Strategy for RoundRobin {
    fn next(&self, addresses: &[Arc<str>]) -> Arc<str> {
        let i = self.index.fetch_add(1, Ordering::Relaxed) % addresses.len();
        addresses[i].clone()
    }
}

/** Periodically sniff node addresses in a cluster. */
pub struct AsyncClusterSniffer {
    client: Client,
    base_params: RequestParams,
    refresh_params: RequestParams,
    refresh: bool,
    addresses: Rc<RefCell<MultipleAddresses<RoundRobin>>>,
}

struct SniffedNodes {
    nodes: Vec<Arc<str>>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNode<'a> {
    #[serde(borrow)] http: Option<SniffedNodeHttp<'a>>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNodeHttp<'a> {
    #[serde(borrow)] publish_address: Option<&'a str>,
}

impl AsyncClusterSniffer {
    /** Create a cluster sniffer with the given base parameters. */
    pub fn new(client: Client, base_params: RequestParams) -> Self {
        let builder = base_params.inner.clone();
        let addresses = {
            MultipleAddresses {
                addresses: vec![base_params.base_url.clone()],
                strategy: RoundRobin::default(),
                params_builder: builder,
            }
        };

        // Specify a `filter_path` when updating node stats because deserialisation occurs on tokio thread
        // This should change in the future if:
        // - we can provide a cpu pool to deserialise on
        // - we want more metadata about the nodes
        // The publish_address may not correspond to the address the node is actually available on
        // In this case, we might want to offer some kind of filter function that consumers can use to transform addresses
        let refresh_params = base_params
            .clone()
            .url_param("filter_path", "nodes.*.http.publish_address");

        AsyncClusterSniffer {
            client: client,
            base_params: base_params,
            refresh_params: refresh_params,
            refresh: true,
            addresses: Rc::new(RefCell::new(addresses)),
        }
    }

    fn should_refresh(&mut self) -> bool {
        if self.refresh {
            self.refresh = false;
            true
        } else {
            false
        }
    }

    /** Get the next address for a request. */
    pub fn next(&mut self) -> Box<Future<Item = RequestParams, Error = Error>> {
        if self.should_refresh() {
            // Need to refresh the addresses
            let req_future = self.client
                .elastic_req(&self.refresh_params, NodesInfoRequest::new());

            unimplemented!();
        } else {
            let address = Ok(self.addresses.borrow().next()).into_future();
            Box::new(address)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn deserialise_publish_address() {
        let nodes = json!({
            "http" : {
                "publish_address" : "127.0.0.1:9200"
            }
        }).to_string();

        let expected = SniffedNode {
            http: Some(SniffedNodeHttp {
                publish_address: Some("127.0.0.1:9200"),
            }),
        };

        let actual: SniffedNode = serde_json::from_str(&nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
