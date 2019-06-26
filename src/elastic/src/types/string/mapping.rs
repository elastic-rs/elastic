/*! Common mapping for the Elasticsearch `string` types. */

use super::{
    keyword::mapping::KeywordFieldMapping,
    text::mapping::{
        TextFieldMapping,
        TextMapping,
    },
};
use serde::{
    ser::SerializeStruct,
    Serialize,
    Serializer,
};
use std::collections::BTreeMap;

/** Default mapping for `String`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultStringMapping;
impl TextMapping for DefaultStringMapping {
    fn fields() -> Option<BTreeMap<&'static str, StringField>> {
        let mut fields = BTreeMap::new();

        let keyword = KeywordFieldMapping {
            ignore_above: Some(256),
            ..Default::default()
        };

        fields.insert("keyword", StringField::Keyword(keyword));

        Some(fields)
    }
}

/** The `index_options` parameter controls what information is added to the inverted index, for search and highlighting purposes. */
#[derive(Debug, Clone, Copy)]
pub enum IndexOptions {
    /** Only the doc number is indexed. Can answer the question Does this term exist in this field? */
    Docs,
    /**
    Doc number and term frequencies are indexed.
    Term frequencies are used to score repeated terms higher than single terms.
    */
    Freqs,
    /**
    Doc number, term frequencies, and term positions (or order) are indexed.
    Positions can be used for proximity or phrase queries.
    */
    Positions,
    /**
    Doc number, term frequencies, positions,
    and start and end character offsets (which map the term back to the original string) are indexed.
    Offsets are used by the postings highlighter.
    */
    Offsets,
}

impl Serialize for IndexOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            IndexOptions::Docs => "docs",
            IndexOptions::Freqs => "freqs",
            IndexOptions::Positions => "positions",
            IndexOptions::Offsets => "offsets",
        })
    }
}

/**
A string sub-field type.

String types can have a number of alternative field representations for different purposes.
*/
#[derive(Debug, Clone, Copy)]
pub enum StringField {
    /** A `token_count` sub field. */
    TokenCount(ElasticTokenCountFieldMapping),
    /** A `completion` suggester sub field. */
    Completion(ElasticCompletionFieldMapping),
    /** A `keyword` sub field. */
    Keyword(KeywordFieldMapping),
    /** A `text` sub field. */
    Text(TextFieldMapping),
}

impl Serialize for StringField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StringField::TokenCount(m) => m.serialize(serializer),
            StringField::Completion(m) => m.serialize(serializer),
            StringField::Keyword(m) => m.serialize(serializer),
            StringField::Text(m) => m.serialize(serializer),
        }
    }
}

/** A multi-field string mapping for a [token count](https://www.elastic.co/guide/en/elasticsearch/reference/master/token-count.html). */
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticTokenCountFieldMapping {
    /**
    The analyzer which should be used for analyzed string fields,
    both at index-time and at search-time (unless overridden by the `search_analyzer`).
    Defaults to the default index analyzer, or the `standard` analyzer.
    */
    pub analyzer: Option<&'static str>,
    /** Field-level index time boosting. Accepts a floating point number, defaults to `1.0`. */
    pub boost: Option<f32>,
    /**
    Should the field be stored on disk in a column-stride fashion,
    so that it can later be used for sorting, aggregations, or scripting?
    Accepts `true` (default) or `false`.
    */
    pub doc_values: Option<bool>,
    /** Should the field be searchable? Accepts `not_analyzed` (default) and `no`. */
    pub index: Option<IndexAnalysis>,
    /**
    Whether or not the field value should be included in the `_all` field?
    Accepts true or false.
    Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
    Otherwise defaults to `true`.
    */
    pub include_in_all: Option<bool>,
    /**
    Controls the number of extra terms that are indexed to make range queries faster.
    Defaults to `32`.
    */
    pub precision_step: Option<u32>,
    /**
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    */
    pub store: Option<bool>,
}

impl Serialize for ElasticTokenCountFieldMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 8)?;

        state.serialize_field("type", "token_count")?;

        ser_field!(state, "analyzer", self.analyzer);
        ser_field!(state, "boost", self.boost);
        ser_field!(state, "doc_values", self.doc_values);
        ser_field!(state, "index", self.index);
        ser_field!(state, "include_in_all", self.include_in_all);
        ser_field!(state, "precision_step", self.precision_step);
        ser_field!(state, "store", self.store);

        state.end()
    }
}

/** A multi-field string mapping for a [completion suggester](https://www.elastic.co/guide/en/elasticsearch/reference/master/search-suggesters-completion.html#search-suggesters-completion). */
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticCompletionFieldMapping {
    /**
    The analyzer which should be used for analyzed string fields,
    both at index-time and at search-time (unless overridden by the `search_analyzer`).
    Defaults to the default index analyzer, or the `standard` analyzer.
    */
    pub analyzer: Option<&'static str>,
    /** The search analyzer to use, defaults to value of analyzer. */
    pub search_analyzer: Option<&'static str>,
    /** Enables the storing of payloads, defaults to `false`. */
    pub payloads: Option<bool>,
    /**
    Preserves the separators, defaults to `true`.
    If disabled, you could find a field starting with Foo Fighters,
    if you suggest for foof.
    */
    pub preserve_separators: Option<bool>,
    /**
    Enables position increments, defaults to `true`.
    If disabled and using stopwords analyzer,
    you could get a field starting with The Beatles, if you suggest for b.
    > Note: You could also achieve this by indexing two inputs, Beatles and The Beatles,
    no need to change a simple analyzer, if you are able to enrich your data.
    */
    pub preserve_position_increments: Option<bool>,
    /**
    Limits the length of a single input, defaults to `50` `UTF-16` code points.
    This limit is only used at index time to reduce the total number of characters per input
    string in order to prevent massive inputs from bloating the underlying datastructure.
    The most usecases won’t be influenced by the default value since prefix completions
    hardly grow beyond prefixes longer than a handful of characters.
    (Old name "max_input_len" is deprecated)
    */
    pub max_input_length: Option<u32>,
}

impl Serialize for ElasticCompletionFieldMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("mapping", 7)?;

        state.serialize_field("type", "completion")?;

        ser_field!(state, "analyzer", self.analyzer);
        ser_field!(state, "search_analyzer", self.search_analyzer);
        ser_field!(state, "payloads", self.payloads);
        ser_field!(state, "preserve_separators", self.preserve_separators);
        ser_field!(
            state,
            "preserve_position_increments",
            self.preserve_position_increments
        );
        ser_field!(state, "max_input_length", self.max_input_length);

        state.end()
    }
}

/** Should the field be searchable? Accepts `not_analyzed` (default) and `no`. */
#[derive(Debug, Clone, Copy)]
pub enum IndexAnalysis {
    /**
    This option applies only to string fields, for which it is the default.
    The string field value is first analyzed to convert the string into terms
    (e.g. a list of individual words), which are then indexed.
    At search time, the query string is passed through (usually) the same analyzer
    to generate terms in the same format as those in the index.
    It is this process that enables full text search.
    */
    Analyzed,
    /**
    Add the field value to the index unchanged, as a single term.
    This is the default for all fields that support this option except for string fields.
    `not_analyzed` fields are usually used with term-level queries for structured search.
    */
    NotAnalyzed,
    /** Do not add this field value to the index. With this setting, the field will not be queryable. */
    No,
}

impl Serialize for IndexAnalysis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            IndexAnalysis::Analyzed => "analyzed",
            IndexAnalysis::NotAnalyzed => "not_analyzed",
            IndexAnalysis::No => "no",
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::collections::BTreeMap;

    use crate::types::{
        prelude::*,
        private::field,
    };

    #[derive(Default, Clone)]
    pub struct MyTextMapping;
    impl TextMapping for MyTextMapping {
        fn fields() -> Option<BTreeMap<&'static str, StringField>> {
            let mut fields = BTreeMap::new();

            fields.insert(
                "raw",
                StringField::Keyword(KeywordFieldMapping {
                    analyzer: Some("my_analyzer"),
                    ..Default::default()
                }),
            );

            fields.insert(
                "count",
                StringField::TokenCount(ElasticTokenCountFieldMapping::default()),
            );

            fields.insert(
                "comp",
                StringField::Completion(ElasticCompletionFieldMapping::default()),
            );

            Some(fields)
        }

        fn fielddata_frequency_filter() -> Option<FieldDataFrequencyFilter> {
            Some(FieldDataFrequencyFilter {
                min: Some(0.0),
                ..Default::default()
            })
        }

        fn analyzer() -> Option<&'static str> {
            Some("my_analyzer")
        }

        fn eager_global_ordinals() -> Option<bool> {
            Some(false)
        }

        fn fielddata() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn ignore_above() -> Option<u32> {
            Some(512)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn index_options() -> Option<IndexOptions> {
            Some(IndexOptions::Freqs)
        }

        fn norms() -> Option<bool> {
            Some(true)
        }

        fn position_increment_gap() -> Option<u32> {
            Some(1)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn search_analyzer() -> Option<&'static str> {
            Some("my_analyzer")
        }

        fn search_quote_analyzer() -> Option<&'static str> {
            Some("my_analyzer")
        }

        fn similarity() -> Option<&'static str> {
            Some("BM25")
        }

        fn term_vector() -> Option<TermVector> {
            Some(TermVector::Yes)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyKeywordMapping;
    impl KeywordMapping for MyKeywordMapping {
        fn fields() -> Option<BTreeMap<&'static str, StringField>> {
            let mut fields = BTreeMap::new();

            fields.insert(
                "text",
                StringField::Text(TextFieldMapping {
                    analyzer: Some("my_analyzer"),
                    ..Default::default()
                }),
            );

            fields.insert(
                "count",
                StringField::TokenCount(ElasticTokenCountFieldMapping::default()),
            );

            fields.insert(
                "comp",
                StringField::Completion(ElasticCompletionFieldMapping::default()),
            );

            Some(fields)
        }

        fn analyzer() -> Option<&'static str> {
            Some("my_analyzer")
        }

        fn doc_values() -> Option<bool> {
            Some(true)
        }

        fn eager_global_ordinals() -> Option<bool> {
            Some(false)
        }

        fn include_in_all() -> Option<bool> {
            Some(false)
        }

        fn ignore_above() -> Option<u32> {
            Some(256)
        }

        fn index() -> Option<bool> {
            Some(true)
        }

        fn index_options() -> Option<IndexOptions> {
            Some(IndexOptions::Docs)
        }

        fn norms() -> Option<bool> {
            Some(false)
        }

        fn null_value() -> Option<&'static str> {
            Some("my string")
        }

        fn store() -> Option<bool> {
            Some(false)
        }

        fn search_analyzer() -> Option<&'static str> {
            Some("my_analyzer")
        }

        fn similarity() -> Option<&'static str> {
            Some("classic")
        }
    }

    #[test]
    fn serialise_string_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultStringMapping)).unwrap();

        let expected = json!({
            "type":"text",
            "fields":{
                "keyword":{
                    "type":"keyword",
                    "ignore_above":256
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_text_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultTextMapping)).unwrap();

        let expected = json!({
            "type": "text"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_text_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyTextMapping)).unwrap();

        let expected = json!({
            "type":"text",
            "analyzer":"my_analyzer",
            "eager_global_ordinals":false,
            "fielddata":true,
            "fielddata_frequency_filter":{
                "min":0.0
            },
            "fields":{
                "comp":{
                    "type":"completion"
                },
                "count":{
                    "type":"token_count"
                },
                "raw":{
                    "type":"keyword",
                    "analyzer":"my_analyzer"
                }
            },
            "include_in_all":true,
            "ignore_above":512,
            "index":false,
            "index_options":"freqs",
            "norms":true,
            "position_increment_gap":1,
            "store":true,
            "search_analyzer":"my_analyzer",
            "search_quote_analyzer":"my_analyzer",
            "similarity":"BM25",
            "term_vector":"yes"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_keyword_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultKeywordMapping)).unwrap();

        let expected = json!({
            "type": "keyword"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_keyword_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyKeywordMapping)).unwrap();

        let expected = json!({
            "type": "keyword",
            "analyzer": "my_analyzer",
            "doc_values": true,
            "eager_global_ordinals": false,
            "fields": {
                "comp": {
                    "type": "completion"
                },
                "count": {
                    "type": "token_count"
                },
                "text": {
                    "type": "text",
                    "analyzer": "my_analyzer"
                }
            },
            "include_in_all": false,
            "ignore_above": 256,
            "index": true,
            "index_options": "docs",
            "norms": false,
            "null_value": "my string",
            "store": false,
            "search_analyzer": "my_analyzer",
            "similarity": "classic"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_field_filter() {
        let filter = FieldDataFrequencyFilter {
            min: None,
            max: None,
            min_segment_size: Some(500),
        };

        let ser = serde_json::to_value(&filter).unwrap();

        let expected = json!({
            "min_segment_size": 500
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_index_options() {
        let io_opts: Vec<String> = vec![
            IndexOptions::Docs,
            IndexOptions::Freqs,
            IndexOptions::Positions,
            IndexOptions::Offsets,
        ]
        .iter()
        .map(|i| serde_json::to_string(i).unwrap())
        .collect();

        let expected_opts = vec![r#""docs""#, r#""freqs""#, r#""positions""#, r#""offsets""#];

        let mut success = true;
        for i in 0..io_opts.len() {
            if expected_opts[i] != io_opts[i] {
                success = false;
                break;
            }
        }

        assert!(success);
    }

    #[test]
    fn serialise_mapping_terms_vector() {
        let v_opts: Vec<String> = vec![
            TermVector::No,
            TermVector::Yes,
            TermVector::WithPositions,
            TermVector::WithOffsets,
            TermVector::WithPositionsOffsets,
        ]
        .iter()
        .map(|i| serde_json::to_string(i).unwrap())
        .collect();

        let expected_opts = vec![
            r#""no""#,
            r#""yes""#,
            r#""with_positions""#,
            r#""with_offsets""#,
            r#""with_positions_offsets""#,
        ];

        let mut success = true;
        for i in 0..v_opts.len() {
            if expected_opts[i] != v_opts[i] {
                success = false;
                break;
            }
        }

        assert!(success);
    }

    #[test]
    fn serialise_mapping_keyword_field() {
        let mapping = StringField::Keyword(KeywordFieldMapping {
            analyzer: Some("my_analyzer"),
            doc_values: Some(true),
            eager_global_ordinals: Some(false),
            include_in_all: Some(true),
            ignore_above: Some(256),
            index: Some(false),
            index_options: Some(IndexOptions::Docs),
            norms: Some(true),
            store: Some(true),
            search_analyzer: Some("my_analyzer"),
            similarity: Some("my_analyzer"),
        });
        let ser = serde_json::to_value(&mapping).unwrap();

        let expected = json!({
            "type":"keyword",
            "analyzer":"my_analyzer",
            "doc_values":true,
            "eager_global_ordinals":false,
            "include_in_all":true,
            "ignore_above":256,
            "index":false,
            "index_options":"docs",
            "norms":true,
            "store":true,
            "search_analyzer":"my_analyzer",
            "similarity":"my_analyzer"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_text_field() {
        let mapping = StringField::Text(TextFieldMapping {
            fielddata_frequency_filter: Some(FieldDataFrequencyFilter {
                min: Some(0.0),
                ..Default::default()
            }),
            analyzer: Some("my_analyzer"),
            eager_global_ordinals: Some(true),
            fielddata: Some(false),
            include_in_all: Some(false),
            ignore_above: Some(512),
            index: Some(true),
            index_options: Some(IndexOptions::Freqs),
            norms: Some(true),
            position_increment_gap: Some(1),
            store: Some(false),
            search_analyzer: Some("my_analyzer"),
            search_quote_analyzer: Some("my_analyzer"),
            similarity: Some("BM25"),
            term_vector: Some(TermVector::No),
        });
        let ser = serde_json::to_value(&mapping).unwrap();

        let expected = json!({
            "type":"text",
            "analyzer":"my_analyzer",
            "eager_global_ordinals":true,
            "fielddata":false,
            "fielddata_frequency_filter":{
                "min":0.0
            },
            "include_in_all":false,
            "ignore_above":512,
            "index":true,
            "index_options":"freqs",
            "norms":true,
            "position_increment_gap":1,
            "store":false,
            "search_analyzer":"my_analyzer",
            "search_quote_analyzer":"my_analyzer",
            "similarity":"BM25",
            "term_vector":"no"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_token_count_field() {
        let mapping = StringField::TokenCount(ElasticTokenCountFieldMapping {
            analyzer: Some("my_analyzer"),
            boost: None,
            doc_values: Some(false),
            index: Some(IndexAnalysis::No),
            include_in_all: Some(true),
            precision_step: Some(15),
            store: Some(true),
        });
        let ser = serde_json::to_value(&mapping).unwrap();

        let expected = json!({
            "type": "token_count",
            "analyzer": "my_analyzer",
            "doc_values": false,
            "index": "no",
            "include_in_all": true,
            "precision_step": 15,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_completion_field() {
        let mapping = StringField::Completion(ElasticCompletionFieldMapping {
            analyzer: Some("my_analyzer"),
            search_analyzer: Some("my_analyzer"),
            payloads: Some(true),
            preserve_separators: Some(false),
            preserve_position_increments: Some(true),
            max_input_length: Some(512),
        });
        let ser = serde_json::to_value(&mapping).unwrap();

        let expected = json!({
            "type": "completion",
            "analyzer": "my_analyzer",
            "search_analyzer": "my_analyzer",
            "payloads": true,
            "preserve_separators": false,
            "preserve_position_increments": true,
            "max_input_length": 512
        });

        assert_eq!(expected, ser);
    }
}
