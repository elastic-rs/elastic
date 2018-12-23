use elastic::prelude::*;
use elastic::Error;

pub fn call(run: &str) -> Result<AsyncClient, Error> {
    match run {
        // Get a default client
        _ => AsyncClientBuilder::new().build(),
    }
}
