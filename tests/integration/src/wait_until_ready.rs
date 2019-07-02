use elastic::prelude::*;
use futures::{
    stream,
    Future,
    Stream,
};
use std::{
    error::Error as StdError,
    fmt,
    time::Duration,
};

type Error = Box<dyn StdError>;

pub fn call(client: AsyncClient, timeout_secs: u64) -> Result<(), Error> {
    let wait = tokio::runtime::current_thread::block_on_all(call_future(client, timeout_secs));

    match wait {
        Ok(()) | Err(Done::Ready) => Ok(()),
        Err(Done::Error(e)) => Err(e),
    }
}

fn call_future(client: AsyncClient, timeout_secs: u64) -> Box<dyn Future<Item = (), Error = Done>> {
    println!("waiting ~{}s until the cluster is ready...", timeout_secs);

    let start = std::time::Instant::now();

    let stream = stream::unfold(false, move |is_done| {
        let now = std::time::Instant::now();
        if is_done || now - Duration::from_secs(timeout_secs) > start {
            None
        } else {
            let client = client.clone();

            let poll = tokio_timer::sleep(Duration::from_secs(3))
                .map_err(Error::from)
                .and_then(move |_| {
                    client
                        .ping()
                        .send()
                        .then(|r| {
                            let r: Result<_, Error> = match r {
                                Ok(_) => Ok(((), true)),
                                Err(_) => Ok(((), false)),
                            };

                            r
                        })
                        .map_err(Error::from)
                });

            Some(poll)
        }
    });

    let poll = stream
        .fold((), |_, _| {
            let r: Result<_, Error> = Ok(());

            r
        })
        // FIXME: This is a super weird hack
        // The stream seems to never terminate
        // unless we return an error. My guess
        // is the runtime attempting to wait on
        // any remaining futures is never returning.
        // I'm not too sure why...
        .then(|r| match r {
            Ok(_) => Err(Done::Ready),
            Err(e) => Err(Done::Error(e)),
        });

    Box::new(poll)
}

#[derive(Debug)]
enum Done {
    Ready,
    Error(Error),
}

impl StdError for Done {
    fn description(&self) -> &str {
        "timeout"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timeout")
    }
}
