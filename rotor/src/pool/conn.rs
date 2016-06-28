use std::io::stderr;
use std::io::Write;
use std::marker::PhantomData;
use std::time::Duration;
use std::process::exit;

use rotor::Scope;
use rotor_http::client::*;
use url::Url;

use super::{ ElasticContext, State, ElasticRequest };
use super::req::ElasticHttp;

/// A state machine for connections to Elasticsearch.
/// 
/// Given a work queue, this machine will attempt to pull requests off the queue 
/// and process them.
pub struct ElasticConnection<C: ElasticContext> {
	state: State,
	_marker: PhantomData<C>
}
impl <C: ElasticContext> ElasticConnection<C> {
	pub fn new(state: State) -> Self {
		ElasticConnection {
			state: state,
			_marker: PhantomData
		}
	}
}
impl <C: ElasticContext> Client for ElasticConnection<C> {
    type Requester = ElasticHttp<C>;
    type Seed = usize;

    //Create a machine that looks at a queue in the global context
    fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
    	println!("{}: ElasticConnection.create", seed);

    	let state = State {
        	queue: seed
        };

    	if !scope.queue_exists(&state.queue) {
    		println!("{}: ElasticConnection.create: creating queue", state.queue);

    		scope.add_queue(state.queue);
    	}
    	else {
    		println!("{}: ElasticConnection.create: queue exists", state.queue);
    	}

        ElasticConnection::new(state)
    }

    //If we're twiddling thumbs, try to pop a request from our queue
    fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
    	println!("{}: ElasticConnection.connection_idle", self.state.queue);

    	match scope.try_pop(&self.state.queue) {
			Some(req) => {
				println!("{}: ElasticConnection.connection_idle: found message", self.state.queue);

				Task::Request(self, ElasticHttp::new(req))
			},
			_ => {
				println!("{}: ElasticConnection.connection_idle: no message", self.state.queue);

				Task::Sleep(self, scope.now() + Duration::from_millis(1000))
			}
		}
    }

    //Check for a new message
    fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
    	println!("{}: ElasticConnection.wakeup", self.state.queue);

        self.connection_idle(conn, scope)
    }

    //Wakeup and check for a new message
    fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
        println!("{}: ElasticConnection.timeout", self.state.queue);

        //TODO: Remove this. Just testing stuff
        println!("{}: ElasticConnection.timeout: pushing test message", self.state.queue);
        scope.push(
            &self.state.queue, 
            ElasticRequest {
                url: Url::parse("http://localhost:9200").unwrap()
            }
        );
        scope.push(
            &self.state.queue, 
            ElasticRequest {
                url: Url::parse("http://localhost:9200").unwrap()
            }
        );

        self.wakeup(conn, scope)
    }

    fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
    	println!("{}: ElasticConnection.connection_error", self.state.queue);

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
}