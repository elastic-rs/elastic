/*! Mapping for the Elasticsearch `keyword` type. */

use crate::types::{
    private::field::FieldMapping,
    string::mapping::{
        IndexOptions,
        StringField,
    },
};
use serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
};
use std::collections::BTreeMap;

/** A field that will be mapped as a `keyword`. */
pub trait KeywordFieldType<TMapping> {}

/**
The base requirements for mapping a `string` type.

Custom mappings can be defined by implementing `KeywordMapping`.

# Examples

Define a custom `KeywordMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
#[derive(Default)]
struct MyStringMapping;
impl KeywordMapping for MyStringMapping {
    //Overload the mapping functions here
    fn boost() -> Option<f32> {
        Some(1.5)
    }
}
# fn main() {}
```

This will produce the following mapping:

```
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyStringMapping;
# impl KeywordMapping for MyStringMapping {
#     //Overload the mapping functions here
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let json = json!(
{
    "type": "keyword",
    "boost": 1.5
}
# );
# let mapping = elastic::types::__derive::standalone_field_ser(MyStringMapping).unwrap();
# assert_eq!(json, mapping);
# }
```
*/
pub trait KeywordMapping {
    /**
    The analyzer which should be used for analyzed string fields,
    both at index-time and at search-time (unless overridden by the `search_analyzer`).
    Defaults to the default index analyzer, or the `standard` analyzer.
    */
    fn analyzer() -> Option<&'static str> {
        None
    }

    /** Field-level index time boosting. Accepts a floating point number, defaults to `1.0`. */
    fn boost() -> Option<f32> {
        None
    }

    /**
    Should the field be stored on disk in a column-stride fashion,
    so that it can later be used for sorting, aggregations, or scripting?
    Accepts `true` (default) or `false`.
    */
    fn doc_values() -> Option<bool> {
        None
    }

    /**
    Should global ordinals be loaded eagerly on refresh?
    Accepts `true` or `false` (default).
    Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations.
    */
    fn eager_global_ordinals() -> Option<bool> {
        None
    }

    /**
    Multi-fields allow the same string value to be indexed in multiple ways for different purposes,
    such as one field for search and a multi-field for sorting and aggregations,
    or the same string value analyzed by different analyzers.

    # Examples

    Subfields are provided as simple `struct`s, so you don't need to define a separate type
    to map them:

    ```
    # #[macro_use] use std::collections::BTreeMap;
    # use elastic::types::prelude::*;
    # #[derive(Default)]
    # struct MyStringMapping;
    # impl KeywordMapping for MyStringMapping {
    fn fields() -> Option<BTreeMap<&'static str, StringField>> {
            let mut fields = BTreeMap::new();

        //Add a `token_count` as a sub field
        fields.insert("count", StringField::TokenCount(
            ElasticTokenCountFieldMapping::default())
        );

        //Add a `completion` suggester as a sub field
        fields.insert("comp", StringField::Completion(
            ElasticCompletionFieldMapping::default())
        );

        Some(fields)
        }
    # }
    # fn main() {}
    ```
    */
    fn fields() -> Option<BTreeMap<&'static str, StringField>> {
        None
    }

    /**
    Whether or not the field value should be included in the `_all` field?
    Accepts true or false.
    Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
    Otherwise defaults to `true`.
    */
    fn include_in_all() -> Option<bool> {
        None
    }

    /**
    The maximum number of characters to index.
    Any characters over this length will be ignored.
    */
    fn ignore_above() -> Option<u32> {
        None
    }

    /**
    Should the field be searchable? Accepts `true` (default) or `false`.
    */
    fn index() -> Option<bool> {
        None
    }

    /** What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`. */
    fn index_options() -> Option<IndexOptions> {
        None
    }

    /** Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`. */
    fn norms() -> Option<bool> {
        None
    }

    /**
    Accepts a `string` value which is substituted for any explicit null values.
    Defaults to `null`, which means the field is treated as missing.
    */
    fn null_value() -> Option<&'static str> {
        None
    }

    /**
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    */
    fn store() -> Option<bool> {
        None
    }

    /**
    The analyzer that should be used at search time on analyzed fields.
    Defaults to the analyzer setting.
    */
    fn search_analyzer() -> Option<&'static str> {
        None
    }

    /**
    Which scoring algorithm or similarity should be used.
    Defaults to `"classic"`, which uses TF/IDF.
    */
    fn similarity() -> Option<&'static str> {
        None
    }
}

/** Default mapping for `bool`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultKeywordMapping;
impl KeywordMapping for DefaultKeywordMapping {}

/** A multi-field string mapping. */
#[derive(Debug, Default, Clone, Copy)]
pub struct KeywordFieldMapping {
    /**
    The analyzer which should be used for analyzed string fields,
    both at index-time and at search-time (unless overridden by the `search_analyzer`).
    Defaults to the default index analyzer, or the `standard` analyzer.
    */
    pub analyzer: Option<&'static str>,
    /**
    Should the field be stored on disk in a column-stride fashion,
    so that it can later be used for sorting, aggregations, or scripting?
    Accepts `true` (default) or `false`.
    */
    pub doc_values: Option<bool>,
    /**
    Should global ordinals be loaded eagerly on refresh?
    Accepts `true` or `false` (default).
    Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations.
    */
    pub eager_global_ordinals: Option<bool>,
    /**
    Whether or not the field value should be included in the `_all` field?
    Accepts true or false.
    Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
    Otherwise defaults to `true`.
    */
    pub include_in_all: Option<bool>,
    /**
    The maximum number of characters to index.
    Any characters over this length will be ignored.
    */
    pub ignore_above: Option<u32>,
    /** Should the field be searchable? Accepts `true` (default) or `false`. */
    pub index: Option<bool>,
    /** What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`. */
    pub index_options: Option<IndexOptions>,
    /** Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`. */
    pub norms: Option<bool>,
    /**
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    */
    pub store: Option<bool>,
    /**
    The analyzer that should be used at search time on analyzed fields.
    Defaults to the analyzer setting.
    */
    pub search_analyzer: Option<&'static str>,
    /**
    Which scoring algorithm or similarity should be used.
    Defaults to `"classic"`, which uses TF/IDF.
    */
    pub similarity: Option<&'static str>,
}

impl Serialize for KeywordFieldMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 12)?;

        state.serialize_field("type", DefaultKeywordMapping::data_type())?;

        ser_field!(state, "analyzer", self.analyzer);
        ser_field!(state, "doc_values", self.doc_values);
        ser_field!(state, "eager_global_ordinals", self.eager_global_ordinals);
        ser_field!(state, "include_in_all", self.include_in_all);
        ser_field!(state, "ignore_above", self.ignore_above);
        ser_field!(state, "index", self.index);
        ser_field!(state, "index_options", self.index_options);
        ser_field!(state, "norms", self.norms);
        ser_field!(state, "store", self.store);
        ser_field!(state, "search_analyzer", self.search_analyzer);
        ser_field!(state, "similarity", self.similarity);

        state.end()
    }
}

mod private {
    use super::{
        KeywordFieldType,
        KeywordMapping,
    };
    use crate::types::private::field::{
        FieldMapping,
        FieldType,
        SerializeFieldMapping,
        StaticSerialize,
    };
    use serde::{
        ser::SerializeStruct,
        Serialize,
        Serializer,
    };

    #[derive(Default)]
    pub struct KeywordPivot;

    impl<TField, TMapping> FieldType<TMapping, KeywordPivot> for TField
    where
        TField: KeywordFieldType<TMapping> + Serialize,
        TMapping: KeywordMapping,
    {
    }

    impl<TMapping> FieldMapping<KeywordPivot> for TMapping
    where
        TMapping: KeywordMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, KeywordPivot>;

        fn data_type() -> &'static str {
            "keyword"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, KeywordPivot>
    where
        TMapping: FieldMapping<KeywordPivot> + KeywordMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 15)?;

            state.serialize_field("type", TMapping::data_type())?;

            ser_field!(state, "boost", TMapping::boost());
            ser_field!(state, "analyzer", TMapping::analyzer());
            ser_field!(state, "doc_values", TMapping::doc_values());
            ser_field!(
                state,
                "eager_global_ordinals",
                TMapping::eager_global_ordinals()
            );
            ser_field!(state, "fields", TMapping::fields());
            ser_field!(state, "include_in_all", TMapping::include_in_all());
            ser_field!(state, "ignore_above", TMapping::ignore_above());
            ser_field!(state, "index", TMapping::index());
            ser_field!(state, "index_options", TMapping::index_options());
            ser_field!(state, "norms", TMapping::norms());
            ser_field!(state, "null_value", TMapping::null_value());
            ser_field!(state, "store", TMapping::store());
            ser_field!(state, "search_analyzer", TMapping::search_analyzer());
            ser_field!(state, "similarity", TMapping::similarity());

            state.end()
        }
    }
}
