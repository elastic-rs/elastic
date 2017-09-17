/*!
Implementation boilerplate for mappable fields.

Most of these types have a generic `TPivot` parameter.
The idea is to use a concrete type for `TPivot` so non-overlapping blanket implementations can be added for `TMapping`.
*/

use std::marker::PhantomData;
use std::borrow::Borrow;
use std::ops::Deref;
use serde::Serialize;

/** The base representation of an Elasticsearch data type. */
pub trait FieldType<TMapping, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default
{
    /** Get a serialisable instance of the type mapping as a field. */
    fn field_mapping() -> TMapping {
        TMapping::default()
    }
}

/** The base representation of an Elasticsearch data type mapping. */
pub trait FieldMapping<TPivot>
    where Self: Default,
          TPivot: Default
{
    /** Prevents infinite recursion when resolving `Serialize` on nested mappings. */
    type DocumentField: Serialize + Default;

    fn data_type() -> &'static str {
        "object"
    }
}

/** Captures traits required for conversion between a field with mapping and a default counterpart. */
pub trait StdField<TStd>
    where Self: PartialEq<TStd> + Deref<Target = TStd> + Borrow<TStd>,
          TStd: PartialEq<Self>
{
}

// TODO: Rename `DocumentField` to `SerializeFieldMapping`

/**
A wrapper type used to work around conflicting implementations of `Serialize` for the various mapping traits.

Serialising `DocumentField` will produce the mapping for the given type, suitable as the mapping of a field for a document.
Individual implementations of `Serialize` for `DocumentField` are spread throughout other modules.
*/
#[derive(Default)]
pub struct DocumentField<TMapping, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default
{
    _m: PhantomData<(TMapping, TPivot)>,
}

impl<TMapping, TPivot> From<TMapping> for DocumentField<TMapping, TPivot>
    where TMapping: FieldMapping<TPivot>,
          TPivot: Default
{
    fn from(_: TMapping) -> Self {
        DocumentField::<TMapping, TPivot>::default()
    }
}
