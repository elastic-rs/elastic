use std::sync::{Mutex, RwLock};
use std::borrow::Cow;
use std::marker::PhantomData;
use serde::ser::{Serialize, SerializeStruct};
use serde_json::Value;
use super::mapping::{DocumentMapping, PropertiesMapping};

/**
The additional fields available to an indexable Elasticsearch type.

This trait is implemented for the type being mapped, rather than the mapping
type itself.
*/
pub trait DocumentType {
    /** The mapping type for this document. */
    type Mapping: DocumentMapping;

    /**
    Get the name for this type.
    
    This is a convenience method that returns the `name` of the bound `DocumentMapping`.
    */
    fn name() -> &'static str {
        Self::Mapping::name()
    }

    /** Get a serialisable instance of the type mapping as a field. */
    fn field_mapping() -> Self::Mapping {
        Self::Mapping::default()
    }

    /** Get a serialisable instance of the type mapping as an indexable type */
    fn index_mapping() -> IndexDocumentMapping<Self::Mapping> {
        IndexDocumentMapping::default()
    }
}

/**
A wrapper type for serialising user types.

Serialising `Document` will produce the mapping for the given type,
suitable as the mapping for
[Put Mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html)
or [Create Index](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html).

# Examples

To serialise a document mapping, you can use its mapping type as a generic parameter in `IndexDocumentMapping<M>`.
For example, we can define an index type for the Create Index API that includes the mapping for `MyType`:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
#[derive(Serialize, ElasticType)]
pub struct MyType {
    pub my_date: Date<DefaultDateMapping>,
    pub my_string: String,
    pub my_num: i32
}

#[derive(Default, Serialize)]
pub struct MyIndex {
    pub mappings: Mappings
}

#[derive(Default, Serialize)]
pub struct Mappings {
    pub mytype: IndexDocumentMapping<MyTypeMapping>
}
# fn main() {
# }
```

Serialising `MyIndex` will produce the following json:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# extern crate serde_json;
# use elastic_types::prelude::*;
# #[derive(Serialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateMapping>,
#     pub my_string: String,
#     pub my_num: i32
# }
# #[derive(Default, Serialize)]
# pub struct MyIndex {
#     pub mappings: Mappings
# }
# #[derive(Default, Serialize)]
# pub struct Mappings {
#     pub mytype: IndexDocumentMapping<MyTypeMapping>
# }
# fn main() {
# let index = serde_json::to_string(&MyIndex::default()).unwrap();
# let json = json_str!(
{
    "mappings": {
        "mytype": {
            "properties": {
                "my_date": {
                    "type": "date",
                    "format": "basic_date_time"
                },
                "my_string": {
                    "type": "text",
                    "fields": {
                        "keyword":{
                            "type":"keyword",
                            "ignore_above":256
                        }
                    }
                },
                "my_num": {
                    "type": "integer"
                }
            }
        }
    }
}
# );
# assert_eq!(json, index);
# }
```

Alternatively, you can implement serialisation manually for `MyIndex` and avoid having
to keep field names up to date if the document type name changes:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# extern crate serde_json;
# use serde::{Serialize, Serializer};
# use serde::ser::SerializeStruct;
# use elastic_types::prelude::*;
#[derive(Serialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateMapping>,
#     pub my_string: String,
#     pub my_num: i32
# }
#[derive(Default, Serialize)]
pub struct MyIndex {
    mappings: Mappings
}

#[derive(Default)]
struct Mappings;
impl Serialize for Mappings {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = try!(serializer.serialize_struct("mappings", 1));

        try!(state.serialize_field(MyType::name(), &MyType::index_mapping()));

        state.end()
    }
}
# fn main() {
# let index = serde_json::to_string(&MyIndex::default()).unwrap();
# let json = json_str!(
# {
#     "mappings": {
#         "mytype": {
#             "properties": {
#                 "my_date": {
#                     "type": "date",
#                     "format": "basic_date_time"
#                 },
#                 "my_string": {
#                     "type": "text",
#                     "fields": {
#                         "keyword":{
#                             "type":"keyword",
#                             "ignore_above":256
#                         }
#                     }
#                 },
#                 "my_num": {
#                     "type": "integer"
#                 }
#             }
#         }
#     }
# }
# );
# assert_eq!(json, index);
# }
```
*/
#[derive(Default)]
pub struct IndexDocumentMapping<TMapping>
where
    TMapping: DocumentMapping,
{
    _m: PhantomData<TMapping>,
}

/** Mapping for an anonymous json object. */
#[derive(Default)]
pub struct ValueDocumentMapping;

impl DocumentMapping for ValueDocumentMapping {
    fn name() -> &'static str {
        "value"
    }
}

impl DocumentType for Value {
    type Mapping = ValueDocumentMapping;
}

impl PropertiesMapping for ValueDocumentMapping {
    fn props_len() -> usize {
        0
    }

    fn serialize_props<S>(_: &mut S) -> Result<(), S::Error>
    where
        S: SerializeStruct,
    {
        Ok(())
    }
}

impl<'a, TDocument, TMapping> DocumentType for &'a TDocument
where
    TDocument: DocumentType<Mapping = TMapping> + Serialize,
    TMapping: DocumentMapping,
{
    type Mapping = TMapping;
}

impl<TDocument, TMapping> DocumentType for Mutex<TDocument>
where
    TDocument: DocumentType<Mapping = TMapping> + Serialize,
    TMapping: DocumentMapping,
{
    type Mapping = TMapping;
}

impl<TDocument, TMapping> DocumentType for RwLock<TDocument>
where
    TDocument: DocumentType<Mapping = TMapping> + Serialize,
    TMapping: DocumentMapping,
{
    type Mapping = TMapping;
}

impl<'a, TDocument, TMapping> DocumentType for Cow<'a, TDocument>
where
    TDocument: DocumentType<Mapping = TMapping> + Serialize + Clone,
    TMapping: DocumentMapping,
{
    type Mapping = TMapping;
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, RwLock};
    use std::borrow::Cow;
    use serde_json::{self, Value};
    use prelude::*;

    // Make sure we can derive with no `uses`.
    pub mod no_prelude {
        #![allow(dead_code)]

        #[derive(Serialize, ElasticType)]
        pub struct TypeWithNoPath {
            id: i32,
        }

        #[derive(Default, ElasticDateFormat)]
        #[elastic(date_format = "yyyy")]
        pub struct DateFormatWithNoPath;
    }

    // Make sure we can derive in a function scope
    #[allow(dead_code)]
    fn fn_scope() {
        #[derive(Serialize, ElasticType)]
        pub struct TypeInFn {
            id: i32,
        }

        #[derive(Default, ElasticDateFormat)]
        #[elastic(date_format = "yyyy")]
        pub struct DateFormatInFn;
    }

    #[derive(Clone, Serialize, ElasticType)]
    pub struct SimpleType {
        pub field1: Date<DefaultDateMapping<EpochMillis>>,
        pub field2: SimpleNestedType,
    }

    #[derive(Clone, Serialize, ElasticType)]
    pub struct SimpleNestedType {
        pub field: i32,
    }

    #[derive(Serialize, ElasticType)]
    #[elastic(mapping = "ManualCustomTypeMapping")]
    pub struct CustomType {
        pub field: i32,
        #[serde(skip_serializing)] pub ignored_field: i32,
        #[serde(rename = "renamed_field")] pub field2: i32,
    }

    #[derive(PartialEq, Debug, Default)]
    pub struct ManualCustomTypeMapping;
    impl DocumentMapping for ManualCustomTypeMapping {
        fn name() -> &'static str {
            "renamed_type"
        }
    }

    #[derive(Serialize, ElasticType)]
    pub struct Wrapped {
        pub field1: Vec<i32>,
        pub field2: Option<bool>,
        pub field3: &'static str,
        pub field4: Value,
        pub field5: Option<SimpleNestedType>,
    }

    #[derive(Serialize, ElasticType)]
    pub struct NoProps {}

    #[derive(Default, Serialize)]
    pub struct Index {
        mappings: Mappings,
    }

    #[derive(Default, Serialize)]
    pub struct Mappings {
        simpletype: IndexDocumentMapping<SimpleTypeMapping>,
    }

    #[test]
    fn use_doc_as_generic_without_supplying_mapping_param() {
        fn use_document<TDocument>()
        where
            TDocument: DocumentType,
        {
            assert!(true);
        }

        use_document::<SimpleType>();
    }

    #[test]
    fn get_type_name() {
        assert_eq!("simpletype", SimpleTypeMapping::name());
    }

    #[test]
    fn get_default_type_name() {
        assert_eq!("simpletype", SimpleType::name());
    }

    #[test]
    fn get_custom_type_name() {
        assert_eq!("renamed_type", CustomType::name());
    }

    #[test]
    fn get_value_type_name() {
        assert_eq!("value", Value::name());
    }

    #[test]
    fn derive_custom_type_mapping() {
        assert_eq!(ManualCustomTypeMapping, CustomType::field_mapping());
    }

    #[test]
    fn serialise_document() {
        let ser = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        let expected = json_str!({
            "properties":{
                "field1": {
                    "type": "date",
                    "format": "epoch_millis"
                },
                "field2": {
                    "type": "nested",
                    "properties": {
                        "field": {
                            "type": "integer"
                        }
                    }
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_borrowed() {
        let ser = serde_json::to_string(&<&'static SimpleType>::index_mapping()).unwrap();

        let expected = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_mutex() {
        let ser = serde_json::to_string(&Mutex::<SimpleType>::index_mapping()).unwrap();

        let expected = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_rwlock() {
        let ser = serde_json::to_string(&RwLock::<SimpleType>::index_mapping()).unwrap();

        let expected = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_cow() {
        let ser = serde_json::to_string(&Cow::<'static, SimpleType>::index_mapping()).unwrap();

        let expected = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_with_no_props() {
        let ser = serde_json::to_string(&NoProps::index_mapping()).unwrap();

        let expected = json_str!({
            "properties": {}
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_for_custom_mapping() {
        let ser = serde_json::to_string(&CustomType::index_mapping()).unwrap();

        let expected = json_str!({
            "properties": {
                "field": {
                    "type": "integer"
                },
                "renamed_field": {
                    "type": "integer"
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_for_value() {
        let ser = serde_json::to_string(&Value::index_mapping()).unwrap();

        let expected = json_str!({
            "properties": {}
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_with_wrapped_types() {
        let ser = serde_json::to_string(&Wrapped::index_mapping()).unwrap();

        let expected = json_str!({
            "properties": {
                "field1": {
                    "type": "integer"
                },
                "field2": {
                    "type": "boolean"
                },
                "field3": {
                    "type": "text",
                    "fields": {
                        "keyword":{
                            "type": "keyword",
                            "ignore_above": 256
                        }
                    }
                },
                "field4": {
                    "type": "nested"
                },
                "field5": {
                    "type": "nested",
                    "properties": {
                        "field": {
                            "type": "integer"
                        }
                    }
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_index_mapping() {
        let ser = serde_json::to_string(&Index::default()).unwrap();

        let expected = json_str!({
            "mappings": {
                "simpletype": {
                    "properties": {
                        "field1": {
                            "type": "date",
                            "format": "epoch_millis"
                        },
                        "field2": {
                            "type": "nested",
                            "properties": {
                                "field": {
                                    "type": "integer"
                                }
                            }
                        }
                    }
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_dynamic() {
        let d_opts: Vec<String> = vec![Dynamic::True, Dynamic::False, Dynamic::Strict]
            .iter()
            .map(|i| serde_json::to_string(i).unwrap())
            .collect();

        let expected_opts = vec![r#"true"#, r#"false"#, r#""strict""#];

        let mut success = true;
        for i in 0..d_opts.len() {
            if expected_opts[i] != d_opts[i] {
                success = false;
                break;
            }
        }

        assert!(success);
    }
}
