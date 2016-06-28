use std::io::stderr;
use std::io::Write;
use std::marker::PhantomData;
use std::time::Duration;
use std::process::exit;

use rotor::Scope;
use rotor_http::client::*;
use url::Url;

use super::{ ElasticContext, State, ElasticRequest };
use super::req::Req;

//State machine for connections
pub struct Cli<C: ElasticContext> {
	state: State,
	_marker: PhantomData<C>
}
impl <C: ElasticContext> Cli<C> {
	pub fn new(state: State) -> Self {
		Cli {
			state: state,
			_marker: PhantomData
		}
	}
}
impl <C: ElasticContext> Client for Cli<C> {
    type Requester = Req<C>;
    type Seed = usize;

    //Create a machine that looks at a queue in the global context
    fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
    	println!("{}: Cli.create", seed);

    	let state = State {
        	queue: seed
        };

    	if !scope.queue_exists(&state.queue) {
    		println!("{}: Cli.create: creating queue", state.queue);

    		scope.add_queue(state.queue);
    	}
    	else {
    		println!("{}: Cli.create: queue exists", state.queue);
    	}

        Cli::new(state)
    }

    //If we're twiddling thumbs, try to pop a request from our queue
    fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
    	println!("{}: Cli.connection_idle", self.state.queue);

    	match scope.try_pop(&self.state.queue) {
			Some(req) => {
				println!("{}: Cli.connection_idle: found message", self.state.queue);

				Task::Request(self, Req::new(req))
			},
			_ => {
				println!("{}: Cli.connection_idle: no message", self.state.queue);

				Task::Sleep(self, scope.now() + Duration::from_millis(1000))
			}
		}
    }

    //Check for a new message
    fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
    	println!("{}: Cli.wakeup", self.state.queue);

        self.connection_idle(conn, scope)
    }

    //Wakeup and check for a new message
    fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
        println!("{}: Cli.timeout", self.state.queue);

        //TODO: Remove this. Just testing stuff
        println!("{}: Cli.timeout: pushing test message", self.state.queue);
        scope.push(
            &self.state.queue, 
            ElasticRequest {
                url: Url::parse("http://localhost:9200").unwrap()
            }
        );

        self.wakeup(conn, scope)
    }

    fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
    	println!("{}: Cli.connection_error", self.state.queue);

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
}