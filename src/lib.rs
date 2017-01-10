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

    pub use reqwest::Client;

    /// A client wrapper over [`reqwest`](https://github.com/seanmonstar/reqwest).
    pub use elastic_reqwest::*;

    /// Request types the Elasticsearch REST API.
    pub use elastic_requests::*;

    /// Response types for the Elasticsearch REST API.
    pub use elastic_responses::*;

    use serde_json;
    use serde::Serialize;

    use super::errors::*;
    use super::types::prelude::{FieldType, Document, DocumentType, DocumentMapping};

    /// A trait for converting a serialisable type into a request.
    pub trait FromType<T>: Sized {
        fn try_for_ty(ty: T) -> Result<Self>;
    }

    /// A trait for converting a document into a request.
    pub trait FromDoc<T, M>: Sized {
        fn try_for_doc(doc: T) -> Result<Self>;
    }

    /// A trait for converting a document mapping into a request.
    pub trait FromMapping<M>: Sized {
        fn try_for_mapping(mapping: M) -> Result<Self>;
    }

    impl<'a, 'b, IDoc> FromType<(Index<'a>, Type<'a>, &'b IDoc)> for IndexRequest<'a>
        where IDoc: Serialize
    {
        fn try_for_ty((index, ty, doc): (Index<'a>, Type<'a>, &'b IDoc)) -> Result<Self> {
            let doc = serde_json::to_string(&doc)?;

            Ok(Self::for_index_ty(index, ty, doc))
        }
    }

    impl<'a, 'b, IDoc> FromType<(Index<'a>, Type<'a>, Id<'a>, &'b IDoc)> for IndexRequest<'a>
        where IDoc: Serialize
    {
        fn try_for_ty((index, ty, id, doc): (Index<'a>, Type<'a>, Id<'a>, &'b IDoc)) -> Result<Self> {
            let doc = serde_json::to_string(&doc)?;

            Ok(Self::for_index_ty_id(index, ty, id, doc))
        }
    }

    impl<'a, 'b, IDoc, IMapping> FromDoc<(Index<'a>, &'b IDoc), IMapping> for IndexRequest<'a>
        where IDoc: DocumentType<IMapping>,
              IMapping: DocumentMapping
    {
        fn try_for_doc((index, doc): (Index<'a>, &'b IDoc)) -> Result<Self> {
            let ty = IDoc::name();

            Self::try_for_ty((index, Type::from(ty), doc))
        }
    }

    impl<'a, 'b, IDoc, IMapping> FromDoc<(Index<'a>, Id<'a>, &'b IDoc), IMapping> for IndexRequest<'a>
        where IDoc: DocumentType<IMapping>,
              IMapping: DocumentMapping
    {
        fn try_for_doc((index, id, doc): (Index<'a>, Id<'a>, &'b IDoc)) -> Result<Self> {
            let ty = IDoc::name();

            Self::try_for_ty((index, Type::from(ty), id, doc))
        }
    }

    impl<'a, IMapping> FromMapping<(Index<'a>, IMapping)> for IndicesPutMappingRequest<'a>
        where IMapping: DocumentMapping
    {
        fn try_for_mapping((index, mapping): (Index<'a>, IMapping)) -> Result<Self> {
            let mapping = serde_json::to_string(&Document::from(mapping))?;

            Ok(Self::for_index_ty(index, IMapping::name(), mapping))
        }
    }

    impl <'a, 'b, IDoc, IMapping> FromDoc<(Index<'a>, &'b IDoc), IMapping> for IndicesPutMappingRequest<'a>
        where IDoc: DocumentType<IMapping>,
              IMapping: DocumentMapping
    {
        fn try_for_doc((index, _): (Index<'a>, &'b IDoc)) -> Result<Self> {
            Self::try_for_mapping((index, IDoc::mapping()))
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
