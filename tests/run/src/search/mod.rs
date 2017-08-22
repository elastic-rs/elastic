use ::{Test, IntegrationTest};
use futures::Future;
use elastic::prelude::*;
use elastic::error::Error;

mod no_index;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| ::run_test(client, no_index::NoIndex))
    ]
}
