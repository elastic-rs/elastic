use serde::Serialize;
use serde_json;
use super::client::requests::{Index, Id, Body, IndexRequest, IndicesPutMappingRequest};
use super::types::prelude::*;

use super::error::*;

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

impl<'a, 'b, T, M> TryForDoc<(Index<'a>, &'b T), M> for IndicesPutMappingRequest<'a>
    where T: DocumentType<M>,
          M: DocumentMapping
{
    fn try_for_doc((index, _): (Index<'a>, &'b T)) -> Result<Self> {
        Self::try_for_mapping((index, T::mapping()))
    }
}
