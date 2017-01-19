pub use serde_json::Error as JsonError;
pub use reqwest::Error as HttpError;
pub use elastic_responses::error::ApiError;

use elastic_responses::error::ResponseError;

error_chain! {
    foreign_links {
    	Api(ApiError);
        Json(JsonError);
        Http(HttpError);
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Self {
        match err {
            ResponseError::Api(err) => ErrorKind::Api(err).into(),
            ResponseError::Json(err) => ErrorKind::Json(err).into(),
        }
    }
}
