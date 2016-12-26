use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use super::object::{ObjectMapping, ObjectFormat};
use super::field::FieldType;

/// The additional fields available to an indexable Elasticsearch type.
///
/// This trait is implemented for the type being mapped, rather than the mapping
/// type itself.
pub trait DocumentType<M>
    where M: ObjectMapping,
          Self: Serialize
{
    /// Get the mapping for this type.
    ///
    /// This is a convenience method that returns the `name` of the bound `ObjectMapping`.
    fn name() -> &'static str {
        M::name()
    }
}

impl<T, M> FieldType<M, ObjectFormat> for T
    where T: DocumentType<M>,
          M: ObjectMapping
{
}

/// A wrapper type for serialising user types.
#[derive(Default)]
pub struct Document<M>
    where M: ObjectMapping
{
    _m: PhantomData<M>,
}

impl<M> From<M> for Document<M>
    where M: ObjectMapping
{
    fn from(_: M) -> Self {
        Document::<M>::default()
    }
}

impl<M> Serialize for Document<M>
    where M: ObjectMapping
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        M::serialize_type(serializer)
    }
}
