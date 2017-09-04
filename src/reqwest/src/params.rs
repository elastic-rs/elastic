pub mod multiple_addresses {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use ::{RequestParams, RequestParamsBuilder};

    pub struct MultipleAddresses<TStrategy> {
        addresses: Vec<Arc<str>>,
        strategy: TStrategy,
        base: RequestParamsBuilder,
    }

    impl<TStrategy> MultipleAddresses<TStrategy> where TStrategy: Strategy {
        pub fn next(&self) -> RequestParams {
            let address = self.strategy.next(&self.addresses);
            RequestParams::from_parts(address, self.base.clone())
        }
    }

    impl MultipleAddresses<RoundRobin> {
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

    pub trait Strategy: Send + Sync {
        fn next(&self, addresses: &[Arc<str>]) -> Arc<str>;
    }

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
}

pub mod sniffer {
    use std::sync::{Arc, RwLock};
    use std::sync::atomic::{AtomicBool, Ordering};
    use reqwest::unstable::async::Client;
    use futures::{Future, IntoFuture};
    use super::multiple_addresses::{MultipleAddresses, RoundRobin};
    use ::{RequestParams, Error};

    pub struct AsyncClusterSniffer {
        client: Client,
        base: RequestParams,
        addresses: Arc<(AtomicBool, RwLock<MultipleAddresses<RoundRobin>>)>,
    }

    impl AsyncClusterSniffer {
        fn new(client: Client, base: RequestParams) -> Self {
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

        fn next(&self) -> Box<Future<Item = RequestParams, Error = Error>> {
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
}
