/*! Mapping for Elasticsearch document types. */

use serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
};

/** A field that will be mapped as a nested document. */
pub trait ObjectFieldType {
    type Mapping: ObjectMapping;
}

/** Elasticsearch datatype name. */
pub const OBJECT_DATATYPE: &'static str = "object";

/** Elasticsearch datatype name. */
pub const DYNAMIC_DATATYPE: &'static str = "dynamic";

/** Elasticsearch datatype name. */
pub const NESTED_DATATYPE: &'static str = "nested";

/** The base requirements for mapping an `object` type. */
pub trait ObjectMapping {
    /** The source of properties for this document. */
    type Properties: PropertiesMapping;

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
    fn serialize_props<S>(state: &mut S) -> Result<(), S::Error>
    where
        S: SerializeStruct;
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
    where
        S: Serializer,
    {
        match *self {
            Dynamic::True => serializer.serialize_bool(true),
            Dynamic::False => serializer.serialize_bool(false),
            Dynamic::Strict => serializer.serialize_str("strict"),
        }
    }
}

mod private {
    use super::{
        ObjectFieldType,
        ObjectMapping,
        PropertiesMapping,
        OBJECT_DATATYPE,
    };
    use crate::types::{
        document::{
            FieldDocumentMapping,
            IndexDocumentMapping,
        },
        private::field::{
            FieldMapping,
            FieldType,
            SerializeFieldMapping,
            StaticSerialize,
        },
    };
    use serde::{
        ser::SerializeStruct,
        Serialize,
        Serializer,
    };
    use std::marker::PhantomData;

    #[derive(Default)]
    pub struct ObjectPivot;

    impl<TField, TMapping> FieldType<TMapping, ObjectPivot> for TField
    where
        TMapping: ObjectMapping,
        TField: ObjectFieldType<Mapping = TMapping>,
    {
    }

    impl<TMapping> FieldMapping<ObjectPivot> for TMapping
    where
        TMapping: ObjectMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, ObjectPivot>;

        fn data_type() -> &'static str {
            <Self as ObjectMapping>::data_type()
        }
    }

    struct Properties<TMapping>
    where
        TMapping: ObjectMapping,
    {
        _m: PhantomData<TMapping>,
    }

    impl<TMapping> Serialize for Properties<TMapping>
    where
        TMapping: ObjectMapping,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state =
                serializer.serialize_struct("properties", TMapping::Properties::props_len())?;
            TMapping::Properties::serialize_props(&mut state)?;
            state.end()
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, ObjectPivot>
    where
        TMapping: FieldMapping<ObjectPivot> + ObjectMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let ty = <TMapping as ObjectMapping>::data_type();
            let props_len = <TMapping as ObjectMapping>::Properties::props_len();

            let (is_object, has_props) = (ty == OBJECT_DATATYPE, props_len > 0);

            let props_len = match (is_object, has_props) {
                (true, true) => 5,
                (true, false) | (false, true) => 4,
                (false, false) => 3,
            };

            let mut state = serializer.serialize_struct("mapping", props_len)?;

            state.serialize_field("type", ty)?;

            ser_field!(state, "dynamic", TMapping::dynamic());
            ser_field!(state, "include_in_all", TMapping::include_in_all());

            if is_object {
                ser_field!(state, "enabled", TMapping::enabled());
            }

            if has_props {
                state.serialize_field("properties", &Properties::<TMapping> { _m: PhantomData })?;
            }

            state.end()
        }
    }

    impl<TMapping> Serialize for FieldDocumentMapping<TMapping>
    where
        TMapping: ObjectMapping,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            SerializeFieldMapping::<TMapping, ObjectPivot>::static_serialize(serializer)
        }
    }

    impl<TMapping> Serialize for IndexDocumentMapping<TMapping>
    where
        TMapping: ObjectMapping,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 1)?;

            state.serialize_field("properties", &Properties::<TMapping> { _m: PhantomData })?;

            state.end()
        }
    }
}
