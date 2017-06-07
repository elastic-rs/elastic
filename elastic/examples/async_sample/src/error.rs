use std::io::Error as IoError;
use futures::sync::mpsc::SendError;
use hyper::Error as HyperError;
use elastic_responses::error::ResponseError;

use body;

quick_error!{
    #[derive(Debug)]
    pub enum Error {
        Io(err: IoError) {
            from()
        }
        Hyper(err: HyperError) {
            from()
        }
        Response(err: ResponseError) {
            from()
        }
        FileBody(err: SendError<body::FileChunkResult>) {
            from()
        }
    }
}
