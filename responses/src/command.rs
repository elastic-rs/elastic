/*!
Response types for a standard command.
*/

use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

/** A standard command acknowledgement response. */
#[derive(Deserialize, Debug, Clone)]
pub struct CommandResponse {
    acknowledged: bool
}

impl CommandResponse {
    /** 
    Whether or not the request was acknowledged.
    
    This doesn't necessarily mean the request has been fully processed.
    */
    pub fn acknowledged(&self) -> bool {
        self.acknowledged
    }
}

impl IsOk for CommandResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
