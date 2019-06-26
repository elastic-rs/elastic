use serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
};
use std::{
    collections::{
        BTreeMap,
        HashMap,
        HashSet,
    },
    hash::Hash,
    marker::PhantomData,
};

use super::field::{
    FieldMapping,
    FieldType,
    SerializeFieldMapping,
    StaticSerialize,
};

pub trait DefaultFieldType {}

/** A mapping implementation for a non-core type, or anywhere it's ok for Elasticsearch to infer the mapping at index-time. */
#[derive(Debug, PartialEq, Default, Clone)]
pub struct DefaultMapping;
impl FieldMapping<()> for DefaultMapping {
    type SerializeFieldMapping = SerializeFieldMapping<DefaultMapping, ()>;

    fn data_type() -> &'static str {
        "object"
    }
}

impl StaticSerialize for SerializeFieldMapping<DefaultMapping, ()> {
    fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 1)?;

        state.serialize_field("type", DefaultMapping::data_type())?;

        state.end()
    }
}

/**
A type that inherits its mapping from an inner value.
*/
pub trait WrappedFieldType<TMapping, TPivot> {}

/**
Mapping for a wrapped value, like an array or optional type.

In Elasticsearch, arrays and optional types aren't special, anything can be indexed as an array or null.
So the mapping for an array or optional type is just the mapping for the type it contains.
*/
#[derive(Debug, Default, Clone)]
pub struct WrappedMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
    _m: PhantomData<(TMapping, TPivot)>,
}

impl<TMapping, TPivot> FieldMapping<TPivot> for WrappedMapping<TMapping, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
    type SerializeFieldMapping = TMapping::SerializeFieldMapping;

    fn data_type() -> &'static str {
        TMapping::data_type()
    }
}

impl<TMapping, TPivot> StaticSerialize
    for SerializeFieldMapping<WrappedMapping<TMapping, TPivot>, TPivot>
where
    TMapping: FieldMapping<TPivot>,
{
    fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TMapping::SerializeFieldMapping::static_serialize(serializer)
    }
}

impl<TField> FieldType<DefaultMapping, ()> for TField where TField: DefaultFieldType {}

/** Mapping implementation for a standard binary tree map. */
impl<K, V> DefaultFieldType for BTreeMap<K, V>
where
    K: AsRef<str> + Ord + Serialize,
    V: Serialize,
{
}

/** Mapping implementation for a standard hash map. */
impl<K, V> DefaultFieldType for HashMap<K, V>
where
    K: AsRef<str> + Eq + Hash + Serialize,
    V: Serialize,
{
}

impl<TField, TMapping, TPivot> FieldType<WrappedMapping<TMapping, TPivot>, TPivot> for TField
where
    TField: WrappedFieldType<TMapping, TPivot>,
    TMapping: FieldMapping<TPivot>,
{
}

impl<TField, TMapping, TPivot> WrappedFieldType<TMapping, TPivot> for Vec<TField>
where
    TField: FieldType<TMapping, TPivot>,
    TMapping: FieldMapping<TPivot>,
{
}

impl<TField, TMapping, TPivot> WrappedFieldType<TMapping, TPivot> for HashSet<TField>
where
    TField: FieldType<TMapping, TPivot>,
    TMapping: FieldMapping<TPivot>,
{
}

impl<TField, TMapping, TPivot> WrappedFieldType<TMapping, TPivot> for Option<TField>
where
    TField: FieldType<TMapping, TPivot>,
    TMapping: FieldMapping<TPivot>,
{
}
