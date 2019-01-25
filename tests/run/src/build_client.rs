use elastic::prelude::*;
use elastic::Error;
use futures_cpupool::CpuPool;
use std::time::Duration;
use tokio_core::reactor::Handle;

pub fn call(handle: &Handle, run: &str) -> Result<AsyncClient, Error> {
    match run {
        // Get a client that sniffs nodes super frequently
        "sniffed_node" => AsyncClientBuilder::new().sniff_nodes_fluent("http://localhost:9200", |n| n.wait(Duration::from_secs(1))).serde_pool(CpuPool::new(4)).build(handle),
        // Get a default client
        _ => AsyncClientBuilder::new().build(handle),
    }
}
