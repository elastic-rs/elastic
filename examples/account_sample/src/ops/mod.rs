pub mod commands;
pub mod queries;

use elastic::client::{RequestParams, SyncClient, SyncClientBuilder};
use elastic::error::Result;

/// A wrapper over the `elastic::Client` that we can implement commands
/// and queries for.
/// This isn't strictly necessary but is one way to avoid exposing
/// functionality from the underlying client to consumers.
/// In an application where commands and queries aren't just executed in
/// the `main` function, you can use a `T: EnsureBankIndexExists` type bound.
pub struct Client {
    io: SyncClient,
}

impl Client {
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = SyncClientBuilder::from_params(params).build()?;

        Ok(Client { io: client })
    }
}
