use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use serde_json::Value;

use super::object::ObjectFormat;

/// The base representation of an Elasticsearch data type.
///
/// `FieldType` is the main `trait` you need to care about when building your own Elasticsearch types.
/// Each type has two generic arguments that help define its mapping:
///
/// - A mapping type, which implements `FieldMapping`
/// - A format type, which is usually `()`. Types with multiple formats, like `Date`, can use the format in the type definition.
///
/// # Links
///
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping-types.html)
pub trait FieldType<M, F = ObjectFormat>
    where M: FieldMapping<F>,
          F: Default,
          Self: Serialize
{
    /// Get the mapping for this type.
    fn mapping() -> M {
        M::default()
    }
}

/// The base requirements for mapping an Elasticsearch data type.
///
/// Each type has its own implementing structures with extra type-specific mapping parameters.
/// If you're building your own Elasticsearch types, see `DocumentTypeMapping`,
/// which is a specialization of `FieldMapping<()>`.
pub trait FieldMapping<F>
    where Self: Default,
          F: Default
{
    /// A type that when serialised will produce the mapping for this field.
    type Field: Serialize + Default;

    /// Get the type name for this mapping, like `date` or `string`.
    fn data_type() -> &'static str {
        "object"
    }
}

/// A wrapper type for serialising fields.
#[derive(Default)]
pub struct Field<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    _m: PhantomData<(M, F)>,
}

impl<M, F> From<M> for Field<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    fn from(_: M) -> Self {
        Field::<M, F>::default()
    }
}

/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
#[derive(Debug, Clone, Copy)]
pub enum IndexAnalysis {
    /// This option applies only to string fields, for which it is the default.
    /// The string field value is first analyzed to convert the string into terms
    /// (e.g. a list of individual words), which are then indexed.
    /// At search time, the query string is passed through (usually) the same analyzer
    /// to generate terms in the same format as those in the index.
    /// It is this process that enables full text search.
    Analyzed,
    /// Add the field value to the index unchanged, as a single term.
    /// This is the default for all fields that support this option except for string fields.
    /// `not_analyzed` fields are usually used with term-level queries for structured search.
    NotAnalyzed,
    /// Do not add this field value to the index. With this setting, the field will not be queryable.
    No,
}

impl Serialize for IndexAnalysis {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            IndexAnalysis::Analyzed => "analyzed",
            IndexAnalysis::NotAnalyzed => "not_analyzed",
            IndexAnalysis::No => "no",
        })
    }
}

/// A mapping implementation for a non-core type, or anywhere it's ok for Elasticsearch to infer the mapping at index-time.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct DefaultMapping;
impl FieldMapping<()> for DefaultMapping {
    type Field = Field<Self, ()>;
}

impl Serialize for Field<DefaultMapping, ()> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 1));

        try!(serializer.serialize_struct_elt(&mut state, "type", DefaultMapping::data_type()));

        serializer.serialize_struct_end(state)
    }
}

/// Mapping for a wrapped value, like an array or optional type.
///
/// In Elasticsearch, arrays and optional types aren't special, anything can be indexed as an array or null.
/// So the mapping for an array or optional type is just the mapping for the type it contains.
#[derive(Debug, Default, Clone)]
pub struct WrappedMapping<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    _m: PhantomData<M>,
    _f: PhantomData<F>,
}

impl<M, F> FieldMapping<F> for WrappedMapping<M, F>
    where M: FieldMapping<F>,
          F: Default
{
    type Field = M::Field;

    fn data_type() -> &'static str {
        M::data_type()
    }
}

/// Mapping implementation for a `serde_json::Value`.
impl FieldType<DefaultMapping, ()> for Value {}

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
          F: Default
{
}

impl<T, M, F> FieldType<WrappedMapping<M, F>, F> for Option<T>
    where T: FieldType<M, F>,
          M: FieldMapping<F>,
          F: Default
{
}
