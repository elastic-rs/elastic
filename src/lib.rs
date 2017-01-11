//! Elasticsearch API Client
//!
//! This crate is a meta-package that makes it easy to work
//! with the Elasticsearch REST API.

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;
extern crate elastic_responses;

pub mod errors;

pub mod client {
    //! HTTP client, requests and responses.
    //!
    //! This module contains the core `ElasticClient` trait, as well
    //! as request and response types.

    pub use reqwest::{StatusCode};

    /// A client wrapper over [`reqwest`](https://github.com/seanmonstar/reqwest).
    pub use elastic_reqwest::{RequestParams};

    /// Request types the Elasticsearch REST API.
    pub use elastic_requests::*;

    /// Response types for the Elasticsearch REST API.
    pub use elastic_responses::*;

    use serde::{Serialize, Deserialize};
    use serde_json;

    use super::errors::*;
    use reqwest::{self, Client as HttpClient};
    use elastic_reqwest::ElasticClient;
    use super::types::prelude::{FieldType, Document, DocumentType, DocumentMapping};

    pub struct Client(HttpClient);
    pub struct HttpResponse(reqwest::Response);

    impl Client {
        pub fn new() -> Result<Self> {
            let client = HttpClient::new()?;
            Ok(Client(client))
        }

        pub fn request<I>(&self, params: &RequestParams, req: I) -> Result<HttpResponse>
            where I: Into<HttpRequest<'static>>
        {
            let res = self.0.elastic_req(params, req)?;

            Ok(HttpResponse(res))
        }
    }

    impl HttpResponse {
        pub fn raw(self) -> reqwest::Response {
            self.0
        }

        pub fn json<T>(mut self) -> Result<T>
            where T: Deserialize
        {
            self.0.json().map_err(|e| e.into())
        }
    }

    /// A trait for converting a document into a request.
    pub trait TryForDoc<T, M>: Sized {
        fn try_for_doc(doc: T) -> Result<Self>;
    }

    /// A trait for converting a document mapping into a request.
    pub trait TryForMapping<M> 
        where Self: Sized
    {
        fn try_for_mapping(mapping: M) -> Result<Self>;
    }

    impl<'a, 'b, T, M> TryForDoc<(Index<'a>, &'b T), M> for IndexRequest<'a>
        where T: DocumentType<M>,
              M: DocumentMapping
    {
        fn try_for_doc((index, doc): (Index<'a>, &'b T)) -> Result<Self> {
            let ty = T::name();

            let doc = serde_json::to_string(&doc)?;

            Ok(Self::for_index_ty(index, ty, doc))
        }
    }

    impl<'a, 'b, T, M> TryForDoc<(Index<'a>, Id<'a>, &'b T), M> for IndexRequest<'a>
        where T: DocumentType<M>,
              M: DocumentMapping
    {
        fn try_for_doc((index, id, doc): (Index<'a>, Id<'a>, &'b T)) -> Result<Self> {
            let ty = T::name();

            let doc = serde_json::to_string(&doc)?;

            Ok(Self::for_index_ty_id(index, ty, id, doc))
        }
    }

    impl<'a, 'b, T> TryForDoc<&'b T, ()> for Body<'a>
        where T: Serialize
    {
        fn try_for_doc(doc: &T) -> Result<Self> {
            let doc = serde_json::to_string(&doc)?;

            Ok(Self::from(doc))
        }
    }

    impl<'a, M> TryForMapping<(Index<'a>, M)> for IndicesPutMappingRequest<'a>
        where M: DocumentMapping
    {
        fn try_for_mapping((index, mapping): (Index<'a>, M)) -> Result<Self> {
            let mapping = serde_json::to_string(&Document::from(mapping))?;

            Ok(Self::for_index_ty(index, M::name(), mapping))
        }
    }

    impl <'a, 'b, T, M> TryForDoc<(Index<'a>, &'b T), M> for IndicesPutMappingRequest<'a>
        where T: DocumentType<M>,
              M: DocumentMapping
    {
        fn try_for_doc((index, _): (Index<'a>, &'b T)) -> Result<Self> {
            Self::try_for_mapping((index, T::mapping()))
        }
    }
}

pub mod types {
    //! Indexable documents and type mapping.
    //!
    //! This module contains tools for defining Elasticsearch-compatible
    //! document types.

    pub use elastic_types::*;
}

pub mod prelude {
    pub use super::client::*;
    pub use super::types::prelude::*;
}