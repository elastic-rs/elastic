/*!
Implementation boilerplate for mappable fields.

Most of these types have a generic `TPivot` parameter.
The idea is to use a concrete type for `TPivot` so non-overlapping blanket implementations can be added for `TMapping`.
*/

use serde::ser::{
    Serialize,
    Serializer,
};
use std::{
    borrow::Borrow,
    marker::PhantomData,
    ops::Deref,
};

pub trait StaticSerialize {
    fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

/** The base representation of an Elasticsearch data type. */
pub trait FieldType<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
}

/** The base representation of an Elasticsearch data type mapping. */
pub trait FieldMapping<TPivot> {
    /** Prevents infinite recursion when resolving `Serialize` on nested mappings. */
    type SerializeFieldMapping: StaticSerialize;

    fn data_type() -> &'static str;
}

/** Captures traits required for conversion between a field with mapping and a default counterpart. */
pub trait StdField<TStd>
where
    Self: PartialEq<TStd> + Deref<Target = TStd> + Borrow<TStd>,
    TStd: PartialEq<Self>,
{
}

/**
A wrapper type used to work around conflicting implementations of `Serialize` for the various mapping traits.

Serialising `SerializeFieldMapping` will produce the mapping for the given type, suitable as the mapping of a field for a document.
Individual implementations of `Serialize` for `SerializeFieldMapping` are spread throughout other modules.
*/
pub struct SerializeFieldMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
    _m: PhantomData<(TMapping, TPivot)>,
}

impl<TMapping, TPivot> Default for SerializeFieldMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
    fn default() -> Self {
        SerializeFieldMapping {
            _m: Default::default(),
        }
    }
}

impl<TMapping, TPivot> Serialize for SerializeFieldMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
    Self: StaticSerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::static_serialize(serializer)
    }
}

#[cfg(test)]
pub(crate) fn serialize<TMapping, TPivot>(_: TMapping) -> SerializeFieldMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
    SerializeFieldMapping<TMapping, TPivot>: Serialize,
{
    SerializeFieldMapping::default()
}
