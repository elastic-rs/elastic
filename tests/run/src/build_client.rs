use tokio_core::reactor::Handle;
use elastic::prelude::*;
use elastic::error::Error;

pub fn call(handle: &Handle, run: &str) -> Result<AsyncClient, Error> {
    match run {
        // Get a default client
        _ => AsyncClientBuilder::new().build(handle)
    }
}
