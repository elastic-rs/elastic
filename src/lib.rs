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
    use super::types::prelude::{ FieldType, Document, DocumentType, DocumentMapping };

    /// A trait for converting a serialisable type into a request.
    pub trait FromType<'a, IDoc>
        where IDoc: Serialize 
    {
        // TODO: Support an Id
        //      May need to use builders for this
        // TODO: Return Result<Self, Error>
        fn from_ty<IIndex, IType>(index: IIndex, ty: IType, doc: &IDoc) -> Self
            where IIndex: Into<Index<'a>>,
                  IType: Into<Type<'a>>;
    }

    /// A trait for converting a document into a request.
    pub trait FromDoc<'a, IDoc, IMapping> {
        // TODO: Return Result<Self, Error>
        fn from_doc<IIndex>(index: IIndex, doc: &IDoc) -> Self
            where IIndex: Into<Index<'a>>;
    }

    /// A trait for converting a document mapping into a request.
    pub trait FromMapping<'a, IMapping> {
        // TODO: Return Result<Self, Error>
        fn from_mapping<IIndex>(index: IIndex, mapping: IMapping) -> Self
            where IIndex: Into<Index<'a>>;
    }

    impl <'a, IDoc> FromType<'a, IDoc> for IndexRequest<'a>
        where IDoc: Serialize 
    {
        fn from_ty<IIndex, IType>(index: IIndex, ty: IType, doc: &IDoc) -> Self
            where IIndex: Into<Index<'a>>,
                  IType: Into<Type<'a>>
        {
            let doc = serde_json::to_string(&doc).unwrap();

            Self::for_index_ty(index, ty, doc)
        }
    }

    impl <'a, IDoc, IMapping> FromDoc<'a, IDoc, IMapping> for IndexRequest<'a>
        where IDoc: DocumentType<IMapping>, 
              IMapping: DocumentMapping
    {
        fn from_doc<IIndex>(index: IIndex, doc: &IDoc) -> Self
            where IIndex: Into<Index<'a>> 
        {
            let ty = IDoc::name();

            Self::from_ty(index, ty, doc)
        }
    }

    impl <'a, IMapping> FromMapping<'a, IMapping> for IndicesPutMappingRequest<'a>
        where IMapping: DocumentMapping
    {
        fn from_mapping<IIndex>(index: IIndex, mapping: IMapping) -> Self
            where IIndex: Into<Index<'a>>
        {
            let mapping = serde_json::to_string(&Document::from(mapping)).unwrap();

            Self::for_index_ty(index, IMapping::name(), mapping)
        }
    }

    impl <'a, IDoc, IMapping> FromDoc<'a, IDoc, IMapping> for IndicesPutMappingRequest<'a>
        where IDoc: DocumentType<IMapping>, 
              IMapping: DocumentMapping
    {
        fn from_doc<IIndex>(index: IIndex, _: &IDoc) -> Self
            where IIndex: Into<Index<'a>> 
        {
            Self::from_mapping(index, IDoc::mapping())
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