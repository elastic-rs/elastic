//! Elasticsearch API Client
//!
//! This crate is a meta-package that makes it easy to work
//! with the Elasticsearch REST API.

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;
extern crate elastic_responses;

pub mod client {
    //! HTTP client, requests and responses.
    //!
    //! This module contains the core `ElasticClient` trait, as well
    //! as request and response types.

    pub use reqwest::Client;

    /// A client wrapper over [`reqwest`](https://github.com/seanmonstar/reqwest).
    pub use elastic_reqwest::*;

    /// Request types the Elasticsearch REST API.
    pub use elastic_requests::*;

    /// Response types for the Elasticsearch REST API.
    pub use elastic_responses::*;

    use serde_json;
    use serde::Serialize;
    use super::types::prelude::{FieldType, Document, DocumentType, DocumentMapping};

    /// A trait for converting a serialisable type into a request.
    pub trait FromType<T> {
        // TODO: Support an Id
        //      May need to use builders for this
        // TODO: Return Result<Self, Error>
        fn from_ty(ty: T) -> Self;
    }

    /// A trait for converting a document into a request.
    pub trait FromDoc<T, M> {
        // TODO: Return Result<Self, Error>
        fn from_doc(doc: T) -> Self;
    }

    /// A trait for converting a document mapping into a request.
    pub trait FromMapping<M> {
        // TODO: Return Result<Self, Error>
        fn from_mapping(mapping: M) -> Self;
    }

    impl<'a, 'b, IDoc> FromType<(Index<'a>, Type<'a>, &'b IDoc)> for IndexRequest<'a>
        where IDoc: Serialize
    {
        fn from_ty((index, ty, doc): (Index<'a>, Type<'a>, &'b IDoc)) -> Self {
            let doc = serde_json::to_string(&doc).unwrap();

            Self::for_index_ty(index, ty, doc)
        }
    }

    impl<'a, 'b, IDoc, IMapping> FromDoc<(Index<'a>, &'b IDoc), IMapping> for IndexRequest<'a>
        where IDoc: DocumentType<IMapping>,
              IMapping: DocumentMapping
    {
        fn from_doc((index, doc): (Index<'a>, &'b IDoc)) -> Self {
            let ty = IDoc::name();

            Self::from_ty((index, Type::from(ty), doc))
        }
    }

    impl<'a, IMapping> FromMapping<(Index<'a>, IMapping)> for IndicesPutMappingRequest<'a>
        where IMapping: DocumentMapping
    {
        fn from_mapping((index, mapping): (Index<'a>, IMapping)) -> Self {
            let mapping = serde_json::to_string(&Document::from(mapping)).unwrap();

            Self::for_index_ty(index, IMapping::name(), mapping)
        }
    }

    impl <'a, 'b, IDoc, IMapping> FromDoc<(Index<'a>, &'b IDoc), IMapping> for IndicesPutMappingRequest<'a>
        where IDoc: DocumentType<IMapping>,
              IMapping: DocumentMapping
    {
        fn from_doc((index, _): (Index<'a>, &'b IDoc)) -> Self
        {
            Self::from_mapping((index, IDoc::mapping()))
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
