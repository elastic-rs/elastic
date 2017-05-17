use serde::Serialize;
use serde_json::{self, Error as JsonError};

use client::requests::IntoBody;
use http::Body;

/// A wrapper around a document type.
#[derive(Debug, Clone, PartialEq)]
pub struct Document<TDocument>(TDocument);

impl<TDocument> From<TDocument> for Document<TDocument> {
    fn from(doc: TDocument) -> Self {
        Document(doc)
    }
}

impl<TDocument> IntoBody for Document<TDocument> where TDocument: Serialize {
    type Error = JsonError;

    fn into_body(self) -> Result<Body, Self::Error> {
        serde_json::to_vec(&self.0).map(|buf| buf.into())
    }
}