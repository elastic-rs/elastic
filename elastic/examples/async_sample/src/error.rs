use std::io::Error as IoError;
use futures::sync::mpsc::SendError;
use hyper::Error as HyperError;

use body;

quick_error!{
    #[derive(Debug)]
    pub enum RequestError {
        Io(err: IoError) {
            from()
        }
        Body(err: SendError<body::ChunkResult>) {
            from()
        }
        Request(err: HyperError) {
            from()
        }
    }
}
