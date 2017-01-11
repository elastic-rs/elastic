use serde_json::Error as JsonError;
use reqwest::Error as HttpError;

error_chain! {
    foreign_links {
        Json(JsonError);
        Http(HttpError);
    }
}
