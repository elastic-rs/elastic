/*! Mapping for Elasticsearch document types. */

use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

/** A field that will be mapped as a nested document. */
pub trait DocumentFieldType<M> where M: DocumentMapping {}

/** Elasticsearch datatype name. */
pub const OBJECT_DATATYPE: &'static str = "object";

/** Elasticsearch datatype name. */
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";

/** Elasticsearch datatype name. */
pub const NESTED_DATATYPE: &'static str = "nested";

/** The base requirements for mapping an `object` type. */
pub trait DocumentMapping
    where Self: PropertiesMapping + Default
{
    /** Get the indexed name for this mapping. */
    fn name() -> &'static str;

    /** Get the type name for this mapping, like `object` or `nested`. */
    fn data_type() -> &'static str {
        NESTED_DATATYPE
    }

    /**
    Whether or not new properties should be added dynamically to an existing object.
    Accepts `true` (default), `false` and `strict`.
    */
    fn dynamic() -> Option<Dynamic> {
        None
    }

    /**
    Whether the JSON value given for the object field should be parsed and indexed
    (`true`, default) or completely ignored (`false`).
    */
    fn enabled() -> Option<bool> {
        None
    }

    /**
    Sets the default `include_in_all` value for all the properties within the object.
    The object itself is not added to the `_all` field.
    */
    fn include_in_all() -> Option<bool> {
        None
    }
}

/**
Serialisation for the mapping of object properties.

This trait is designed to be auto-derived and can't be usefully implemented manually.
*/
pub trait PropertiesMapping {
    /**
    The number of mapped property fields for this type.

    This number should be the same as the number of fields being serialised by `serialize_props`.
    */
    fn props_len() -> usize;

    /**
    Serialisation for the mapped property fields on this type.

    You can use the `field_ser` function to simplify `serde` calls.
    */
    fn serialize_props<S>(state: &mut S) -> Result<(), S::Error> where S: SerializeStruct;
}

#[derive(Default)]
struct Properties<TMapping>
    where TMapping: DocumentMapping
{
    _m: PhantomData<TMapping>,
}

impl<TMapping> Serialize for Properties<TMapping>
    where TMapping: DocumentMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("properties", TMapping::props_len()));
        try!(TMapping::serialize_props(&mut state));
        state.end()
    }
}

/**
The dynamic setting may be set at the mapping type level, and on each inner object.
Inner objects inherit the setting from their parent object or from the mapping type.
*/
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
    /** Newly detected fields are added to the mapping. (default). */
    True,
    /** Newly detected fields are ignored. New fields must be added explicitly. */
    False,
    /** If new fields are detected, an exception is thrown and the document is rejected. */
    Strict,
}

impl Serialize for Dynamic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Dynamic::True => serializer.serialize_bool(true),
            Dynamic::False => serializer.serialize_bool(false),
            Dynamic::Strict => serializer.serialize_str("strict"),
        }
    }
}

mod private {
    use serde::{Serialize, Serializer};
    use serde::ser::SerializeStruct;
    use document::{DocumentType, IndexDocumentMapping};
    use private::field::{FieldType, DocumentField, FieldMapping};
    use super::{DocumentFieldType, Properties, DocumentMapping, OBJECT_DATATYPE};

    #[derive(Default)]
    pub struct DocumentPivot;

    impl<TField, TMapping> FieldType<TMapping, DocumentPivot> for TField
        where TMapping: DocumentMapping,
            TField: DocumentFieldType<TMapping>
    { }

    impl<TDocument, TMapping> DocumentFieldType<TMapping> for TDocument
        where TDocument: DocumentType<Mapping = TMapping>,
              TMapping: DocumentMapping
    {
    }

    impl<TMapping> FieldMapping<DocumentPivot> for TMapping
        where TMapping: DocumentMapping
    {
        type DocumentField = DocumentField<TMapping, DocumentPivot>;

        fn data_type() -> &'static str {
            <Self as DocumentMapping>::data_type()
        }
    }

    impl<TMapping> Serialize for DocumentField<TMapping, DocumentPivot>
        where TMapping: FieldMapping<DocumentPivot> + DocumentMapping
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            let ty = <TMapping as DocumentMapping>::data_type();
            let (is_object, has_props) = (ty == OBJECT_DATATYPE, TMapping::props_len() > 0);

            let props_len = match (is_object, has_props) {
                (true, true) => 5,
                (true, false) | (false, true) => 4,
                (false, false) => 3,
            };

            let mut state = try!(serializer.serialize_struct("mapping", props_len));

            try!(state.serialize_field("type", ty));

            ser_field!(state, "dynamic", TMapping::dynamic());
            ser_field!(state, "include_in_all", TMapping::include_in_all());

            if is_object {
                ser_field!(state, "enabled", TMapping::enabled());
            }

            if has_props {
                try!(state.serialize_field("properties", &Properties::<TMapping>::default()));
            }

            state.end()
        }
    }

    impl<TMapping> Serialize for IndexDocumentMapping<TMapping>
        where TMapping: DocumentMapping
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            let mut state = try!(serializer.serialize_struct("mapping", 1));

            try!(state.serialize_field("properties", &Properties::<TMapping>::default()));

            state.end()
        }
    }
}
