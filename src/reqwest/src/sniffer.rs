/*! Connection sniffing and multiple static addresses. */

#![allow(warnings)]

use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use reqwest::unstable::async::Client;
use futures::{Future, IntoFuture};
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, SeqAccess, Error as DeError};
use {Error, RequestParams, RequestParamsBuilder};
use async::{AsyncElasticClient, AsyncFromResponse};
use req::NodesInfoRequest;
use res::parsing::{parse, IsOk, HttpResponseHead, Unbuffered, MaybeOkResponse, ResponseBody};
use res::error::ParseResponseError;

/** Select a base address for a given request using some strategy. */
#[derive(Clone)]
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
        self.try_next().expect("unable to optain a node address")
    }

    /** Try get the next address for a request. */
    pub fn try_next(&self) -> Option<RequestParams> {
        self.strategy
            .try_next(&self.addresses)
            .map(|address| RequestParams::from_parts(address, self.params_builder.clone()))
    }
}

impl MultipleAddresses<RoundRobin> {
    /** Use a round-robin strategy for balancing traffic over the given set of addresses. */
    pub fn round_robin<I, S>(addresses: I, params_builder: RequestParamsBuilder) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let addresses: Vec<_> = addresses.into_iter().map(|a| a.as_ref().into()).collect();

        assert!(addresses.len() > 0, "must supply more than 0 node addresses");

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
    /** Try get the next address. */
    fn try_next(&self, addresses: &[Arc<str>]) -> Option<Arc<str>>;
}

/** A round-robin strategy cycles through addresses sequentially. */
#[derive(Clone)]
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
    fn try_next(&self, addresses: &[Arc<str>]) -> Option<Arc<str>> {
        if addresses.len() == 0 {
            None
        } else {
            let i = self.index.fetch_add(1, Ordering::Relaxed) % addresses.len();
            Some(addresses[i].clone())
        }
    }
}

/** Periodically sniff node addresses in a cluster. */
#[derive(Clone)]
pub struct AsyncClusterSniffer {
    client: Client,
    base_params: RequestParams,
    refresh_params: RequestParams,
    inner: Rc<RefCell<AsyncClusterSnifferInner>>,
}

struct AsyncClusterSnifferInner {
    refresh: bool,
    refreshing: bool,
    nodes: MultipleAddresses<RoundRobin>,
}

impl AsyncClusterSniffer {
    /** Create a cluster sniffer with the given base parameters. */
    pub fn new(client: Client, base_params: RequestParams) -> Self {
        let builder = base_params.inner.clone();
        let nodes = MultipleAddresses::round_robin(vec![base_params.base_url.clone()], builder);

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
            inner: Rc::new(RefCell::new(AsyncClusterSnifferInner {
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
        if self.should_refresh() {
            let mut inner = self.inner.clone();
            {
                inner.borrow_mut().refreshing = true;
            }

            let base_params = self.base_params.clone();

            // TODO: Make this more resilient to failure
            // TODO: Ensure we only have 1 refresh happening at a time (extra refreshing property)
            let refresh_addresses = self.client
                .elastic_req(&self.refresh_params, NodesInfoRequest::new())
                .and_then(|res| parse::<SniffedNodes>().from_response(res))
                .and_then(move |parsed| {
                    let mut inner = inner.borrow_mut();

                    inner.nodes.addresses = parsed.nodes
                        .into_iter()
                        .filter_map(|node| node.http
                            .and_then(|http| http.publish_address)
                            .map(|publish_address| Arc::<str>::from(publish_address)))
                        .collect();

                    inner.refreshing = false;

                    Ok(inner.nodes.try_next().unwrap_or(base_params))
                });

            Box::new(refresh_addresses)
        } else {
            let address = Ok(self.inner.borrow().nodes.try_next().unwrap_or_else(|| self.base_params.clone())).into_future();
            Box::new(address)
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNodes {
    #[serde(deserialize_with = "deserialize_nodes")]
    nodes: Vec<SniffedNode>,
}

impl IsOk for SniffedNodes {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, unbuffered: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(unbuffered)),
            _ => Ok(MaybeOkResponse::err(unbuffered)),
        }
    }
}

fn deserialize_nodes<'de, D>(deserializer: D) -> Result<Vec<SniffedNode>, D::Error>
    where
        D: Deserializer<'de>
{
    #[derive(Debug, PartialEq)]
    struct SniffedNodeSet(Vec<SniffedNode>);

    impl<'de> Deserialize<'de> for SniffedNodeSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
        {
            #[derive(Default)]
            struct SniffedNodeSetVisitor;

            impl<'de> Visitor<'de> for SniffedNodeSetVisitor {
                type Value = SniffedNodeSet;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map of node values")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                    where M: MapAccess<'de>
                {
                    let mut nodes = Vec::with_capacity(access.size_hint().unwrap_or(0));

                    while let Some((_, node)) = access.next_entry::<&str, _>()? {
                        nodes.push(node);
                    }

                    Ok(SniffedNodeSet(nodes))
                }
            }
            
            deserializer.deserialize_map(SniffedNodeSetVisitor::default())
        }
    }

    let nodes = SniffedNodeSet::deserialize(deserializer)?;

    Ok(nodes.0)
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNode {
    http: Option<SniffedNodeHttp>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNodeHttp {
    publish_address: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn deserialise_nodes() {
        let nodes = json!({
            "nodes": {
                "node1": {
                    "http": {
                        "publish_address": "1.1.1.1:9200"
                    }
                },
                "node2": {
                    "http": {
                        "publish_address": "1.1.1.2:9200"
                    }
                }
            }
        }).to_string();

        let expected = SniffedNodes {
            nodes: vec![
                SniffedNode {
                    http: Some(SniffedNodeHttp {
                        publish_address: Some("1.1.1.1:9200".to_owned()),
                    }),
                },
                 SniffedNode {
                    http: Some(SniffedNodeHttp {
                        publish_address: Some("1.1.1.2:9200".to_owned()),
                    }),
                }
            ]
        };

        let actual: SniffedNodes = serde_json::from_str(&nodes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialise_nodes_empty() {
        let nodes = json!({
            "nodes": { }
        }).to_string();

        let expected = SniffedNodes {
            nodes: vec![]
        };

        let actual: SniffedNodes = serde_json::from_str(&nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
