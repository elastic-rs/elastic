use std::hash::Hash;
use std::marker::PhantomData;
use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use super::field::{DocumentField, FieldType, FieldMapping};

pub trait DefaultFieldType {}

/** A mapping implementation for a non-core type, or anywhere it's ok for Elasticsearch to infer the mapping at index-time. */
#[derive(Debug, PartialEq, Default, Clone)]
struct DefaultMapping;
impl FieldMapping<()> for DefaultMapping
{
    type DocumentField = DocumentField<DefaultMapping, ()>;
}

impl Serialize for DocumentField<DefaultMapping, ()> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 1));

        try!(state.serialize_field("type", DefaultMapping::data_type()));

        state.end()
    }
}

pub trait WrappedFieldType<TMapping, TPivot> {}

/**
Mapping for a wrapped value, like an array or optional type.

In Elasticsearch, arrays and optional types aren't special, anything can be indexed as an array or null.
So the mapping for an array or optional type is just the mapping for the type it contains.
*/
#[derive(Debug, Default, Clone)]
pub struct WrappedMapping<TMapping, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default
{
    _m: PhantomData<(TMapping, TPivot)>,
}

impl<TMapping, TPivot> FieldMapping<TPivot> for WrappedMapping<TMapping, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default
{
    type DocumentField = TMapping::DocumentField;

    fn data_type() -> &'static str {
        TMapping::data_type()
    }
}

impl<TMapping, TPivot> Serialize for DocumentField<WrappedMapping<TMapping, TPivot>, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        TMapping::DocumentField::default().serialize(serializer)
    }
}

impl<TField> FieldType<DefaultMapping, ()> for TField
    where TField: DefaultFieldType
{ }

/** Mapping implementation for a standard binary tree map. */
impl<K, V> DefaultFieldType for BTreeMap<K, V>
    where K: AsRef<str> + Ord + Serialize,
          V: Serialize
{ }

/** Mapping implementation for a standard hash map. */
impl<K, V> DefaultFieldType for HashMap<K, V>
    where K: AsRef<str> + Eq + Hash + Serialize,
          V: Serialize
{ }

impl<TField, TMapping, TPivot> FieldType<WrappedMapping<TMapping, TPivot>, TPivot> for TField
    where TField: WrappedFieldType<TMapping, TPivot>,
          TMapping: FieldMapping<TPivot>,
          TPivot: Default,
{ }

impl<TField, TMapping, TPivot> WrappedFieldType<TMapping, TPivot> for Vec<TField>
    where TField: FieldType<TMapping, TPivot>,
          TMapping: FieldMapping<TPivot>,
          TPivot: Default,
{ }

impl<TField, TMapping, TPivot> WrappedFieldType<TMapping, TPivot> for Option<TField>
    where TField: FieldType<TMapping, TPivot>,
          TMapping: FieldMapping<TPivot>,
          TPivot: Default,
{ }
