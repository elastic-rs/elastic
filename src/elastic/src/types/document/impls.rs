use super::mapping::{
    ObjectFieldType,
    ObjectMapping,
    PropertiesMapping,
};
use serde::ser::SerializeStruct;
use serde_json::Value;
use std::{
    borrow::Cow,
    marker::PhantomData,
};

#[doc(inline)]
pub use crate::params::{
    Id,
    Index,
    Type,
};

/**
The default name for document types in a single document index.
*/
pub const DEFAULT_DOC_TYPE: &'static str = "_doc";

/**
An indexable Elasticsearch type.

This trait is implemented for the type being mapped, rather than the mapping
type itself.
*/
pub trait DocumentType: ObjectFieldType {
    /** Get a serialisable instance of the type mapping as a field. */
    fn field_mapping() -> FieldDocumentMapping<<Self as ObjectFieldType>::Mapping> {
        FieldDocumentMapping::default()
    }

    /** Get a serialisable instance of the type mapping as an indexable type */
    fn index_mapping() -> IndexDocumentMapping<<Self as ObjectFieldType>::Mapping> {
        IndexDocumentMapping::default()
    }

    /** Get the name of the index this document belongs to. */
    fn index(&self) -> Index;

    /** Get the name of the type this document belongs to. */
    fn ty(&self) -> Type;

    /** Try get an id for this document. */
    fn partial_id(&self) -> Option<Id>;

    /** Try get a statically known index this document belongs to. */
    fn partial_static_index() -> Option<Index<'static>>;

    /** Try get a statically known type this document belongs to. */
    fn partial_static_ty() -> Option<Type<'static>>;
}

/**
An indexable Elasticsearch type with a static index.
*/
pub trait StaticIndex: DocumentType {
    fn static_index() -> Index<'static> {
        Self::partial_static_index().expect("missing static index")
    }
}

/**
An indexable Elasticsearch type with a static document type.
*/
pub trait StaticType: DocumentType {
    fn static_ty() -> Type<'static> {
        Self::partial_static_ty().expect("missing static type")
    }
}

/**
A wrapper type for serialising user types as fields.
*/
#[derive(Clone, Copy)]
pub struct FieldDocumentMapping<TMapping>(PhantomData<TMapping>);

#[cfg(test)]
impl<TMapping> FieldDocumentMapping<TMapping>
where
    TMapping: ObjectMapping + Default,
{
    fn into_mapping(&self) -> TMapping {
        TMapping::default()
    }
}

impl<TMapping> Default for FieldDocumentMapping<TMapping>
where
    TMapping: ObjectMapping,
{
    fn default() -> Self {
        FieldDocumentMapping(Default::default())
    }
}

/**
A wrapper type for serialising user types.

Serialising `Document` will produce the mapping for the given type,
suitable as the mapping for
[Put Mapping](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html)
or [Create Index](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html).
*/
pub struct IndexDocumentMapping<TMapping>
where
    TMapping: ObjectMapping,
{
    _m: PhantomData<TMapping>,
}

impl<TMapping> Default for IndexDocumentMapping<TMapping>
where
    TMapping: ObjectMapping,
{
    fn default() -> Self {
        IndexDocumentMapping {
            _m: Default::default(),
        }
    }
}

/** Mapping for an anonymous json object. */
#[derive(Default)]
pub struct ValueObjectMapping;

impl ObjectMapping for ValueObjectMapping {
    type Properties = EmptyPropertiesMapping;
}

impl ObjectFieldType for Value {
    type Mapping = ValueObjectMapping;
}

/** Mapping for an anonymous json object. */
#[derive(Default)]
pub struct EmptyPropertiesMapping;

impl PropertiesMapping for EmptyPropertiesMapping {
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

impl<'a, TObject, TMapping> ObjectFieldType for &'a TObject
where
    TObject: ObjectFieldType<Mapping = TMapping>,
    TMapping: ObjectMapping,
{
    type Mapping = TMapping;
}

impl<'a, TDocument> DocumentType for &'a TDocument
where
    TDocument: DocumentType,
{
    fn index(&self) -> Index {
        (*self).index()
    }

    fn ty(&self) -> Type {
        (*self).ty()
    }

    fn partial_id(&self) -> Option<Id> {
        (*self).partial_id()
    }

    fn partial_static_index() -> Option<Index<'static>> {
        TDocument::partial_static_index()
    }

    fn partial_static_ty() -> Option<Type<'static>> {
        TDocument::partial_static_ty()
    }
}

impl<'a, TObject, TMapping> ObjectFieldType for Cow<'a, TObject>
where
    TObject: ObjectFieldType<Mapping = TMapping> + Clone,
    TMapping: ObjectMapping,
{
    type Mapping = TMapping;
}

impl<'a, TDocument> DocumentType for Cow<'a, TDocument>
where
    TDocument: DocumentType + Clone,
{
    fn index(&self) -> Index {
        self.as_ref().index()
    }

    fn ty(&self) -> Type {
        self.as_ref().ty()
    }

    fn partial_id(&self) -> Option<Id> {
        self.as_ref().partial_id()
    }

    fn partial_static_index() -> Option<Index<'static>> {
        TDocument::partial_static_index()
    }

    fn partial_static_ty() -> Option<Type<'static>> {
        TDocument::partial_static_ty()
    }
}

impl<'a, TDocument> StaticIndex for &'a TDocument
where
    TDocument: StaticIndex,
{
    fn static_index() -> Index<'static> {
        TDocument::static_index()
    }
}

impl<'a, TDocument> StaticIndex for Cow<'a, TDocument>
where
    TDocument: StaticIndex + Clone,
{
    fn static_index() -> Index<'static> {
        TDocument::static_index()
    }
}

impl<'a, TDocument> StaticType for &'a TDocument
where
    TDocument: StaticType,
{
    fn static_ty() -> Type<'static> {
        TDocument::static_ty()
    }
}

impl<'a, TDocument> StaticType for Cow<'a, TDocument>
where
    TDocument: StaticType + Clone,
{
    fn static_ty() -> Type<'static> {
        TDocument::static_ty()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DocumentType,
        IndexDocumentMapping,
        StaticIndex,
        StaticType,
    };
    use crate::types::prelude::*;
    use serde_json::{
        self,
        Value,
    };
    use std::{
        borrow::Cow,
        collections::HashSet,
    };

    // Make sure we can derive with no `uses`.
    pub mod no_prelude {
        #![allow(dead_code)]

        #[derive(Serialize, ElasticType)]
        #[elastic(crate_root = "crate::types")]
        pub struct TypeWithNoPath {
            id: String,
        }

        #[derive(Default, ElasticDateFormat)]
        #[elastic(crate_root = "crate::types")]
        #[elastic(date_format = "yyyy")]
        pub struct DateFormatWithNoPath;
    }

    // Make sure we can derive in a function scope
    #[allow(dead_code)]
    fn fn_scope() {
        #[derive(Serialize, ElasticType)]
        #[elastic(crate_root = "crate::types")]
        pub struct TypeInFn {
            id: String,
        }

        #[derive(Default, ElasticDateFormat)]
        #[elastic(crate_root = "crate::types")]
        #[elastic(date_format = "yyyy")]
        pub struct DateFormatInFn;
    }

    #[derive(Clone, Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    pub struct SimpleType {
        pub field1: Date<DefaultDateMapping<EpochMillis>>,
        pub field2: SimpleNestedType,
    }

    #[derive(Clone, Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    pub struct SimpleNestedType {
        pub field: i32,
    }

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    #[elastic(
        index = "renamed_index",
        ty = "renamed_ty",
        id(expr = "self.id()"),
        mapping = "ManualCustomTypeMapping"
    )]
    pub struct CustomType {
        pub field: i32,
        #[serde(skip_serializing)]
        pub ignored_field: i32,
        #[serde(rename = "renamed_field")]
        pub field2: i32,
    }

    impl CustomType {
        fn id(&self) -> String {
            self.field.to_string()
        }
    }

    #[derive(PartialEq, Debug, Default)]
    pub struct ManualCustomTypeMapping;
    impl ObjectMapping for ManualCustomTypeMapping {
        type Properties = CustomType;
    }

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    pub struct Wrapped {
        pub field1: Vec<i32>,
        pub field2: Option<bool>,
        pub field3: &'static str,
        pub field4: Value,
        pub field5: Option<SimpleNestedType>,
        pub field6: Value,
        pub field7: HashSet<String>,
    }

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
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
    fn get_default_type_index() {
        assert_eq!("simpletype", SimpleType::static_index());
    }

    #[test]
    fn get_custom_type_index() {
        assert_eq!("renamed_index", CustomType::static_index());
    }

    #[test]
    fn get_default_type() {
        assert_eq!("_doc", SimpleType::static_ty());
    }

    #[test]
    fn get_custom_type() {
        assert_eq!("renamed_ty", CustomType::static_ty());
    }

    #[test]
    fn get_custom_type_id() {
        let doc = CustomType {
            field: 13,
            ignored_field: 0,
            field2: 1,
        };

        assert_eq!("13", doc.partial_id().unwrap());
    }

    #[test]
    fn derive_custom_type_mapping() {
        assert_eq!(
            ManualCustomTypeMapping,
            CustomType::field_mapping().into_mapping()
        );
    }

    #[test]
    fn serialise_document() {
        let ser = serde_json::to_value(&SimpleType::index_mapping()).unwrap();

        let expected = json!({
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
    fn serialise_document_cow() {
        let ser = serde_json::to_string(&Cow::<'static, SimpleType>::index_mapping()).unwrap();

        let expected = serde_json::to_string(&SimpleType::index_mapping()).unwrap();

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_with_no_props() {
        let ser = serde_json::to_value(&NoProps::index_mapping()).unwrap();

        let expected = json!({
            "properties": {}
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_document_for_custom_mapping() {
        let ser = serde_json::to_value(&CustomType::index_mapping()).unwrap();

        let expected = json!({
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
    fn serialise_mapping_with_wrapped_types() {
        let ser = serde_json::to_value(&Wrapped::index_mapping()).unwrap();

        let expected = json!({
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
                },
                "field6": {
                    "type": "nested"
                },
                "field7": {
                    "type": "text",
                    "fields": {
                        "keyword":{
                            "type": "keyword",
                            "ignore_above": 256
                        }
                    }
                }
            }
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_index_mapping() {
        let ser = serde_json::to_value(&Index::default()).unwrap();

        let expected = json!({
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
