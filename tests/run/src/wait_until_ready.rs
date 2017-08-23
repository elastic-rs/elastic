use std::error::Error as StdError;
use std::time::Duration;
use tokio_timer::Timer;
use futures::{stream, Future, Stream};
use elastic::prelude::*;
use elastic::error::Error;

pub fn call(client: AsyncClient) -> Box<Future<Item = (), Error = Box<StdError>>> {
    println!("waiting until the cluster is ready...");

    let timer = Timer::default();
    let stream = stream::unfold((), move |_| {
        let request = client.request(PingRequest::new()).send().map_err(|e| e.into());

        let map_result = request.then(|res: Result<AsyncResponseBuilder, Error>| match res {
            Ok(_) => Ok((true, ())),
            _ => Ok((false, ()))
        });

        Some(map_result)
    });

    let wait = timer.interval(Duration::from_secs(10));

    let poll = stream
        .zip(wait)
        .take_while(|&(r, ())| Ok(!r))
        .into_future()
        .map(|_| ())
        .map_err(|(e, _)| e.into());

    Box::new(timer.timeout(poll, Duration::from_secs(60)))
}
