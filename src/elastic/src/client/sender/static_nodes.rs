/*! Multiple static nodes that can be load balanced by some strategy. */

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use client::sender::params::{RequestParams, PreRequestParams};

/** Select a base address for a given request using some strategy. */
#[derive(Clone)]
pub struct StaticNodes<TStrategy = RoundRobin> {
    pub(crate) nodes: Vec<Arc<str>>,
    strategy: TStrategy,
    params: PreRequestParams,
}

impl<TStrategy> StaticNodes<TStrategy>
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
            .try_next(&self.nodes)
            .map(|address| RequestParams::from_parts(address, self.params.clone()))
    }
}

impl StaticNodes<RoundRobin> {
    /** Use a round-robin strategy for balancing traffic over the given set of nodes. */
    pub fn round_robin<I, S>(nodes: I, params: PreRequestParams) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Arc<str>>,
    {
        let nodes: Vec<_> = nodes.into_iter().map(Into::into).collect();

        assert!(nodes.len() > 0, "must supply more than 0 nodes");

        let strategy = RoundRobin::default();

        StaticNodes {
            nodes: nodes,
            strategy: strategy,
            params: params,
        }
    }
}

/** The strategy selects an address from a given collection. */
pub trait Strategy: Send + Sync {
    /** Try get the next address. */
    fn try_next(&self, nodes: &[Arc<str>]) -> Option<Arc<str>>;
}

/** A round-robin strategy cycles through nodes sequentially. */
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
    fn try_next(&self, nodes: &[Arc<str>]) -> Option<Arc<str>> {
        if nodes.len() == 0 {
            None
        } else {
            let i = self.index.fetch_add(1, Ordering::Relaxed) % nodes.len();
            Some(nodes[i].clone())
        }
    }
}
