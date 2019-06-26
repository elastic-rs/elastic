use elastic::{
    prelude::*,
    Error,
};
use std::time::Duration;

pub fn call(run: &str) -> Result<AsyncClient, Error> {
    match run {
        // Get a client that sniffs nodes super frequently
        "sniffed_node" => AsyncClientBuilder::new()
            .sniff_nodes_fluent("http://localhost:9200", |n| n.wait(Duration::from_secs(1)))
            //.serde_pool(Arc::new(ThreadPool::new()))
            .build(),
        // Get a default client
        _ => AsyncClientBuilder::new().build(),
    }
}
