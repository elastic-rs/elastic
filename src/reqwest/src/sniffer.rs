/*! Connection sniffing and multiple static addresses. */

#![allow(warnings)]

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use reqwest::unstable::async::Client;
use futures::{Future, IntoFuture};
use ::{RequestParams, RequestParamsBuilder, Error};

/** Select a base address for a given request using some strategy. */
pub struct MultipleAddresses<TStrategy> {
    addresses: Vec<Arc<str>>,
    strategy: TStrategy,
    base: RequestParamsBuilder,
}

impl<TStrategy> MultipleAddresses<TStrategy> where TStrategy: Strategy {
    /** Get the next address for a request. */
    pub fn next(&self) -> RequestParams {
        let address = self.strategy.next(&self.addresses);
        RequestParams::from_parts(address, self.base.clone())
    }
}

impl MultipleAddresses<RoundRobin> {
    /** Use a round-robin strategy for balancing traffic over the given set of addresses. */
    pub fn round_robin<I, S>(addresses: I, base: RequestParamsBuilder) -> Self
        where I: IntoIterator<Item = S>,
                S: AsRef<str>
    {
        let addresses = addresses.into_iter().map(|a| a.as_ref().into()).collect();
        let strategy = RoundRobin::default();

        MultipleAddresses {
            addresses: addresses,
            strategy: strategy,
            base: base,
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
            index: Arc::new(AtomicUsize::new(0))
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
    base: RequestParams,
    addresses: Arc<(AtomicBool, RwLock<MultipleAddresses<RoundRobin>>)>,
}

impl AsyncClusterSniffer {
    /** Create a cluster sniffer with the given base parameters. */
    pub fn new(client: Client, base: RequestParams) -> Self {
        let builder = base.inner.clone();
        let addresses = {
            let address = &base.base_url;
            MultipleAddresses::round_robin(&[address], builder)
        };

        AsyncClusterSniffer {
            client: client,
            base: base,
            addresses: Arc::new((AtomicBool::new(true), RwLock::new(addresses)))
        }
    }

    /** Get the next address for a request. */
    pub fn next(&self) -> Box<Future<Item = RequestParams, Error = Error>> {
        if self.addresses.0.swap(false, Ordering::SeqCst) {
            // Need to refresh the addresses
            unimplemented!();
        } else {
            let address = match self.addresses.1.read() {
                Ok(addresses) => addresses.next(),
                Err(_) => self.base.clone()
            };

            let address = Ok(address).into_future();

            Box::new(address)
        }
    }
}
