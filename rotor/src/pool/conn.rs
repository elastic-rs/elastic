use std::io::stderr;
use std::io::Write;
use std::marker::PhantomData;
use std::time::Duration;
use std::process::exit;

use rotor::Scope;
use rotor_http::client::*;

use super::{ ElasticContext, MachineId };
use super::req::Req;

//State machine for connections
pub struct Cli<C: ElasticContext> {
	id: MachineId,
	_marker: PhantomData<C>
}
impl <C: ElasticContext> Cli<C> {
	pub fn new(id: MachineId) -> Self {
		Cli {
			id: id,
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

    	let id = MachineId {
        	token: seed
        };

    	if !scope.queue_exists(&id) {
    		println!("{}: Cli.create: creating queue", seed);

    		scope.add_queue(&id);
    	}
    	else {
    		println!("{}: Cli.create: queue exists", seed);
    	}

        Cli::new(id)
    }

    //If we're twiddling thumbs, try to pop a request from our queue
    fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
    	println!("{}: Cli.connection_idle", self.id.token);

    	match scope.try_pop(&self.id) {
			Some(req) => {
				println!("{}: Cli.connection_idle: found message", self.id.token);

				Task::Request(self, Req::new(req))
			},
			_ => {
				println!("{}: Cli.connection_idle: no message", self.id.token);

				Task::Sleep(self, scope.now() + Duration::from_millis(1000))
			}
		}
    }

    //Check for a new message
    fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
    	println!("{}: Cli.wakeup", self.id.token);

        self.connection_idle(conn, scope)
    }

    //Wakeup and check for a new message
    fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
        println!("{}: Cli.timeout", self.id.token);

        self.wakeup(conn, scope)
    }

    fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
    	println!("{}: Cli.connection_error", self.id.token);

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
}