/*! Multiple static nodes that can be load balanced by some strategy. */

#![allow(missing_docs)]

use crate::{
    error::{
        self,
        Error,
    },
    http::sender::{
        NextParams,
        NodeAddress,
        PreRequestParams,
        RequestParams,
    },
    private,
};
use std::sync::{
    atomic::{
        AtomicUsize,
        Ordering,
    },
    Arc,
};

/** Select a base address for a given request using some strategy. */
#[derive(Clone)]
pub struct StaticNodes<TStrategy = RoundRobin> {
    nodes: Vec<NodeAddress>,
    strategy: TStrategy,
    params: PreRequestParams,
}

impl<TStrategy> NextParams for StaticNodes<TStrategy>
where
    TStrategy: Strategy + Clone,
{
    type Params = Result<RequestParams, Error>;

    fn next(&self) -> Self::Params {
        self.strategy
            .try_next(&self.nodes)
            .map(|address| RequestParams::from_parts(address, self.params.clone()))
            .map_err(error::request)
    }
}

impl<TStrategy> private::Sealed for StaticNodes<TStrategy> {}

impl<TStrategy> StaticNodes<TStrategy> {
    pub(crate) fn set(&mut self, nodes: Vec<NodeAddress>) -> Result<(), Error> {
        if nodes.is_empty() {
            return Err(error::request(error::message(
                "the number of node addresses must be greater than 0",
            )));
        }

        self.nodes = nodes;

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn get(&self) -> &[NodeAddress] {
        &self.nodes
    }
}

impl StaticNodes<RoundRobin> {
    /** Use a round-robin strategy for balancing traffic over the given set of nodes. */
    pub fn round_robin<I, S>(nodes: I, params: PreRequestParams) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NodeAddress>,
    {
        let nodes: Vec<_> = nodes.into_iter().map(Into::into).collect();

        let strategy = RoundRobin::default();

        StaticNodes {
            nodes,
            strategy,
            params,
        }
    }
}

/** The strategy selects an address from a given collection. */
pub trait Strategy: Send + Sync {
    /** Try get the next address. */
    fn try_next(&self, nodes: &[NodeAddress]) -> Result<NodeAddress, StrategyError>;
}

quick_error! {
    /**
    An error attempting to get an address using a strategy.
    */
    #[derive(Debug)]
    pub enum StrategyError {
        Empty {
            description("the list of addresses was empty")
        }
        /** A different kind of error */
        Other(err: String) {
            description("an error occurred while getting an address")
            display("an error occurred while getting an address. Caused by: {}", err)
        }
        #[doc(hidden)]
        __NonExhaustive
    }
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
    fn try_next(&self, nodes: &[NodeAddress]) -> Result<NodeAddress, StrategyError> {
        if nodes.is_empty() {
            Err(StrategyError::Empty)
        } else {
            let i = self.index.fetch_add(1, Ordering::Relaxed) % nodes.len();
            Ok(nodes[i].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::sender::NextParams;

    fn round_robin(addresses: Vec<&'static str>) -> StaticNodes<RoundRobin> {
        StaticNodes::round_robin(addresses, PreRequestParams::default())
    }

    fn expected_addresses() -> Vec<&'static str> {
        vec!["http://a:9200", "http://b:9200", "http://c:9200"]
    }

    #[test]
    fn round_robin_next_multi() {
        let nodes = round_robin(expected_addresses());

        for _ in 0..10 {
            for expected in expected_addresses() {
                let actual = nodes.next().unwrap();

                assert_eq!(expected, actual.get_base_url());
            }
        }
    }

    #[test]
    fn round_robin_next_single() {
        let expected = "http://a:9200";
        let nodes = round_robin(vec![expected]);

        for _ in 0..10 {
            let actual = nodes.next().unwrap();

            assert_eq!(expected, actual.get_base_url());
        }
    }

    #[test]
    fn round_robin_next_empty_fails() {
        let nodes = round_robin(vec![]);

        assert!(nodes.next().is_err());
    }
}
