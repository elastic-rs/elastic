/*! Mapping for the Elasticsearch `text` type. */

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

/** A field that will be mapped as `text`. */
pub trait TextFieldType<TMapping> {}

/**
The base requirements for mapping a `string` type.

Custom mappings can be defined by implementing `TextMapping`.

# Examples

Define a custom `TextMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
#[derive(Default)]
struct MyStringMapping;
impl TextMapping for MyStringMapping {
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
# impl TextMapping for MyStringMapping {
#     //Overload the mapping functions here
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let json = json!(
{
    "type": "text",
    "boost": 1.5
}
# );
# let mapping = elastic::types::__derive::standalone_field_ser(MyStringMapping).unwrap();
# assert_eq!(json, mapping);
# }
```
*/
pub trait TextMapping {
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
    Should global ordinals be loaded eagerly on refresh?
    Accepts `true` or `false` (default).
    Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations.
    */
    fn eager_global_ordinals() -> Option<bool> {
        None
    }

    /**
    Can the field use in-memory fielddata for sorting, aggregations, or scripting?
    Accepts `true` or `false` (default).
    */
    fn fielddata() -> Option<bool> {
        None
    }

    /**
    Expert settings which allow to decide which values to load in memory when `fielddata` is enabled.
    By default all values are loaded.
    */
    fn fielddata_frequency_filter() -> Option<FieldDataFrequencyFilter> {
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
    # impl TextMapping for MyStringMapping {
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

    /** Should the field be searchable? Accepts `true` (default) or `false`. */
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
    The number of fake term position which should be inserted between each element of an array of strings.
    Defaults to the `position_increment_gap` configured on the analyzer which defaults to `100`.
    `100` was chosen because it prevents phrase queries with reasonably large slops (less than `100`)
    from matching terms across field values.
    */
    fn position_increment_gap() -> Option<u32> {
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
    The analyzer that should be used at search time when a phrase is encountered.
    Defaults to the `search_analyzer` setting.
    */
    fn search_quote_analyzer() -> Option<&'static str> {
        None
    }

    /**
    Which scoring algorithm or similarity should be used.
    Defaults to `"classic"`, which uses TF/IDF.
    */
    fn similarity() -> Option<&'static str> {
        None
    }

    /**
    Whether term vectors should be stored for an `analyzed` field.
    Defaults to `No`.
    */
    fn term_vector() -> Option<TermVector> {
        None
    }
}

/** Default mapping for `text`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultTextMapping;
impl TextMapping for DefaultTextMapping {}

/** Term vectors contain information about the terms produced by the analysis process. */
#[derive(Debug, Clone, Copy)]
pub enum TermVector {
    /** No term vectors are stored. (default) */
    No,
    /** Just the terms in the field are stored. */
    Yes,
    /** Terms and positions are stored. */
    WithPositions,
    /** Terms and character offsets are stored. */
    WithOffsets,
    /** Terms, positions, and character offsets are stored. */
    WithPositionsOffsets,
}

impl Serialize for TermVector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            TermVector::No => "no",
            TermVector::Yes => "yes",
            TermVector::WithPositions => "with_positions",
            TermVector::WithOffsets => "with_offsets",
            TermVector::WithPositionsOffsets => "with_positions_offsets",
        })
    }
}

/** Fielddata for term frequency as a percentage range. */
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FieldDataFrequencyFilter {
    /** The min frequency percentage. */
    pub min: Option<f32>,
    /** The max frequency percentage. */
    pub max: Option<f32>,
    /** The minimum number of docs a segment should contain. */
    pub min_segment_size: Option<i32>,
}

impl Serialize for FieldDataFrequencyFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 3)?;

        ser_field!(state, "min", self.min);
        ser_field!(state, "max", self.max);
        ser_field!(state, "min_segment_size", self.min_segment_size);

        state.end()
    }
}

/** A multi-field string mapping. */
#[derive(Debug, Default, Clone, Copy)]
pub struct TextFieldMapping {
    /**
    The analyzer which should be used for analyzed string fields,
    both at index-time and at search-time (unless overridden by the `search_analyzer`).
    Defaults to the default index analyzer, or the `standard` analyzer.
    */
    pub analyzer: Option<&'static str>,
    /**
    Should global ordinals be loaded eagerly on refresh?
    Accepts `true` or `false` (default).
    Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations.
    */
    pub eager_global_ordinals: Option<bool>,
    /**
    Can the field use in-memory fielddata for sorting, aggregations, or scripting?
    Accepts `true` or `false` (default).
    */
    pub fielddata: Option<bool>,
    /**
    Expert settings which allow to decide which values to load in memory when `fielddata` is enabled.
    By default all values are loaded.
    */
    pub fielddata_frequency_filter: Option<FieldDataFrequencyFilter>,
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
    The number of fake term position which should be inserted between each element of an array of strings.
    Defaults to the `position_increment_gap` configured on the analyzer which defaults to `100`.
    `100` was chosen because it prevents phrase queries with reasonably large slops (less than `100`)
    from matching terms across field values.
    */
    pub position_increment_gap: Option<u32>,
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
    The analyzer that should be used at search time when a phrase is encountered.
    Defaults to the `search_analyzer` setting.
    */
    pub search_quote_analyzer: Option<&'static str>,
    /**
    Which scoring algorithm or similarity should be used.
    Defaults to `"classic"`, which uses TF/IDF.
    */
    pub similarity: Option<&'static str>,
    /**
    Whether term vectors should be stored for an `analyzed` field.
    Defaults to `No`.
    */
    pub term_vector: Option<TermVector>,
}

impl Serialize for TextFieldMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 16)?;

        state.serialize_field("type", DefaultTextMapping::data_type())?;

        ser_field!(state, "analyzer", self.analyzer);
        ser_field!(state, "eager_global_ordinals", self.eager_global_ordinals);
        ser_field!(state, "fielddata", self.fielddata);
        ser_field!(
            state,
            "fielddata_frequency_filter",
            self.fielddata_frequency_filter
        );
        ser_field!(state, "include_in_all", self.include_in_all);
        ser_field!(state, "ignore_above", self.ignore_above);
        ser_field!(state, "index", self.index);
        ser_field!(state, "index_options", self.index_options);
        ser_field!(state, "norms", self.norms);
        ser_field!(state, "position_increment_gap", self.position_increment_gap);
        ser_field!(state, "store", self.store);
        ser_field!(state, "search_analyzer", self.search_analyzer);
        ser_field!(state, "search_quote_analyzer", self.search_quote_analyzer);
        ser_field!(state, "similarity", self.similarity);
        ser_field!(state, "term_vector", self.term_vector);

        state.end()
    }
}

mod private {
    use super::{
        TextFieldType,
        TextMapping,
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
    pub struct TextPivot;

    impl<TField, TMapping> FieldType<TMapping, TextPivot> for TField
    where
        TField: TextFieldType<TMapping> + Serialize,
        TMapping: TextMapping,
    {
    }

    impl<TMapping> FieldMapping<TextPivot> for TMapping
    where
        TMapping: TextMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, TextPivot>;

        fn data_type() -> &'static str {
            "text"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, TextPivot>
    where
        TMapping: FieldMapping<TextPivot> + TextMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 18)?;

            state.serialize_field("type", TMapping::data_type())?;

            ser_field!(state, "boost", TMapping::boost());
            ser_field!(state, "analyzer", TMapping::analyzer());
            ser_field!(
                state,
                "eager_global_ordinals",
                TMapping::eager_global_ordinals()
            );
            ser_field!(state, "fielddata", TMapping::fielddata());
            ser_field!(
                state,
                "fielddata_frequency_filter",
                TMapping::fielddata_frequency_filter()
            );
            ser_field!(state, "fields", TMapping::fields());
            ser_field!(state, "include_in_all", TMapping::include_in_all());
            ser_field!(state, "ignore_above", TMapping::ignore_above());
            ser_field!(state, "index", TMapping::index());
            ser_field!(state, "index_options", TMapping::index_options());
            ser_field!(state, "norms", TMapping::norms());
            ser_field!(
                state,
                "position_increment_gap",
                TMapping::position_increment_gap()
            );
            ser_field!(state, "store", TMapping::store());
            ser_field!(state, "search_analyzer", TMapping::search_analyzer());
            ser_field!(
                state,
                "search_quote_analyzer",
                TMapping::search_quote_analyzer()
            );
            ser_field!(state, "similarity", TMapping::similarity());
            ser_field!(state, "term_vector", TMapping::term_vector());

            state.end()
        }
    }
}
