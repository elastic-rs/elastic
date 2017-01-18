pub use serde_json::Error as JsonError;
pub use reqwest::Error as HttpError;
pub use elastic_responses::error::ApiError;

error_chain! {
    foreign_links {
    	Api(ApiError);
        Json(JsonError);
        Http(HttpError);
    }
}
