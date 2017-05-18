use std::hash::Hash;
use std::marker::PhantomData;
use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use super::field::{DocumentField, FieldMapping, SerializeField};
use document::FieldType;

/// A mapping implementation for a non-core type, or anywhere it's ok for Elasticsearch to infer the mapping at index-time.
#[derive(Debug, PartialEq, Default, Clone)]
struct DefaultMapping;
impl FieldMapping<()> for DefaultMapping {}

impl SerializeField<()> for DefaultMapping {
    type Field = DocumentField<Self, ()>;
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

/// Mapping for a wrapped value, like an array or optional type.
///
/// In Elasticsearch, arrays and optional types aren't special, anything can be indexed as an array or null.
/// So the mapping for an array or optional type is just the mapping for the type it contains.
#[derive(Debug, Default, Clone)]
struct WrappedMapping<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    _m: PhantomData<(M, F)>,
}

impl<M, F> FieldMapping<F> for WrappedMapping<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    fn data_type() -> &'static str {
        M::data_type()
    }
}

impl<M, F> SerializeField<F> for WrappedMapping<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    type Field = M::Field;
}

impl<M, F> Serialize for DocumentField<WrappedMapping<M, F>, F>
    where M: FieldMapping<F>,
          F: Default
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        M::Field::default().serialize(serializer)
    }
}

/// Mapping implementation for a standard binary tree map.
impl<K, V> FieldType<DefaultMapping, ()> for BTreeMap<K, V>
    where K: AsRef<str> + Ord + Serialize,
          V: Serialize
{
}

/// Mapping implementation for a standard hash map.
impl<K, V> FieldType<DefaultMapping, ()> for HashMap<K, V>
    where K: AsRef<str> + Eq + Hash + Serialize,
          V: Serialize
{
}

impl<T, M, F> FieldType<WrappedMapping<M, F>, F> for Vec<T>
    where T: FieldType<M, F>,
          M: FieldMapping<F>,
          F: Default,
          DocumentField<M, F>: Serialize
{
}

impl<T, M, F> FieldType<WrappedMapping<M, F>, F> for Option<T>
    where T: FieldType<M, F>,
          M: FieldMapping<F>,
          F: Default,
          DocumentField<M, F>: Serialize
{
}
