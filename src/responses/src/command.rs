/*!
Response types for a standard command.
*/

use parsing::IsOkOnSuccess;

/** A standard command acknowledgement response. */
#[derive(Deserialize, Debug, Clone)]
pub struct CommandResponse {
    acknowledged: bool,
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

impl IsOkOnSuccess for CommandResponse {}
