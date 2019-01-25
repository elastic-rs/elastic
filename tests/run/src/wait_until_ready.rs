use elastic::prelude::*;
use elastic::Error;
use futures::{stream, Future, Stream};
use std::error::Error as StdError;
use std::fmt;
use std::time::{Duration, Instant};
use tokio_timer::{Deadline, Interval};

#[derive(Clone)]
struct Ping {
    client: AsyncClient,
}

impl Ping {
    fn is_ready(&self) -> Box<Future<Item = bool, Error = Box<StdError>>> {
        let request = self.client.ping().send().map_err(|e| e.into());

        let check = request.then(|res: Result<PingResponse, Error>| match res {
            Ok(_) => Ok(true),
            _ => Ok(false),
        });

        Box::new(check)
    }
}

pub fn call(client: AsyncClient, timeout_secs: u64) -> Box<Future<Item = (), Error = Box<StdError>>> {
    println!("waiting up to {}s until the cluster is ready...", timeout_secs);

    let stream = stream::repeat(Ping { client: client });

    let wait = Interval::new(Instant::now(), Duration::from_secs(10)).from_err();

    let poll = stream.take_while(|ping| ping.is_ready().map(|ready| !ready)).zip(wait).collect();

    let poll_or_timeout = Deadline::new(poll, Instant::now() + Duration::from_secs(timeout_secs))
        .map(|_| ())
        .map_err(|e| if let Some(e) = e.into_inner() { e } else { Box::new(TimeoutError) });

    Box::new(poll_or_timeout)
}

#[derive(Debug)]
struct TimeoutError;

impl StdError for TimeoutError {
    fn description(&self) -> &str {
        "timeout"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timeout")
    }
}
