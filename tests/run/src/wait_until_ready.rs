use std::error::Error as StdError;
use std::time::Duration;
use tokio_timer::Timer;
use futures::{stream, Future, Stream};
use elastic::prelude::*;
use elastic::Error;

#[derive(Clone)]
struct Ping {
    client: AsyncClient,
}

impl Ping {
    fn is_not_ready(&self) -> Box<Future<Item = bool, Error = Box<StdError>>> {
        let request = self.client
            .ping()
            .send()
            .map_err(|e| e.into());

        let check = request.then(|res: Result<PingResponse, Error>| match res {
            Ok(_) => Ok(false),
            _ => Ok(true),
        });

        Box::new(check)
    }
}

pub fn call(client: AsyncClient, timeout_secs: u64) -> Box<Future<Item = (), Error = Box<StdError>>> {
    println!(
        "waiting up to {}s until the cluster is ready...",
        timeout_secs
    );

    let timer = Timer::default();
    let stream = stream::repeat(Ping { client: client });

    let wait = timer.interval(Duration::from_secs(10)).from_err();

    let poll = stream
        .take_while(|ping| ping.is_not_ready())
        .zip(wait)
        .collect();

    let poll_or_timeout = timer
        .timeout(poll, Duration::from_secs(timeout_secs))
        .map(|_| ());

    Box::new(poll_or_timeout)
}
