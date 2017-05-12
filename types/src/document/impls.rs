use std::marker::PhantomData;
use serde::Serialize;
use serde::ser::SerializeStruct;
use super::mapping::{DocumentMapping};
use private::field::FieldMapping;

/// The additional fields available to an indexable Elasticsearch type.
///
/// This trait is implemented for the type being mapped, rather than the mapping
/// type itself.
pub trait DocumentType where Self: Serialize
{
    /// The mapping type for this document.
    type Mapping: DocumentMapping;

    /// Get the name for this type.
    ///
    /// This is a convenience method that returns the `name` of the bound `DocumentMapping`.
    fn name() -> &'static str {
        Self::Mapping::name()
    }
}

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
/// - [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-types.html)
pub trait FieldType<M, F>
    where M: FieldMapping<F>,
          F: Default,
          Self: Serialize
{
    /// Get the mapping for this type.
    fn mapping() -> M {
        M::default()
    }
}

/// A wrapper type for serialising user types.
///
/// Serialising `Document` will produce the mapping for the given type,
/// suitable as the mapping for
/// [Put Mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html)
/// or [Create Index](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html).
///
/// # Examples
///
/// To serialise a document mapping, you can use its mapping type as a generic parameter in `Document<M>`.
/// For example, we can define an index type for the Create Index API that includes the mapping for `MyType`:
///
/// ```
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// #[derive(Serialize, ElasticType)]
/// pub struct MyType {
///     pub my_date: Date<DefaultDateFormat>,
///     pub my_string: String,
///     pub my_num: i32
/// }
///
/// #[derive(Default, Serialize)]
/// pub struct MyIndex {
///     pub mappings: Mappings
/// }
///
/// #[derive(Default, Serialize)]
/// pub struct Mappings {
///     pub mytype: Document<MyTypeMapping>
/// }
/// # fn main() {
/// # }
/// ```
///
/// Serialising `MyIndex` will produce the following json:
///
/// ```
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # #[derive(Serialize, ElasticType)]
/// # pub struct MyType {
/// #     pub my_date: Date<DefaultDateFormat>,
/// #     pub my_string: String,
/// #     pub my_num: i32
/// # }
/// # #[derive(Default, Serialize)]
/// # pub struct MyIndex {
/// #     pub mappings: Mappings
/// # }
/// # #[derive(Default, Serialize)]
/// # pub struct Mappings {
/// #     pub mytype: Document<MyTypeMapping>
/// # }
/// # fn main() {
/// # let index = serde_json::to_string(&MyIndex::default()).unwrap();
/// # let json = json_str!(
/// {
///     "mappings": {
///         "mytype": {
///             "properties": {
///                 "my_date": {
///                     "type": "date",
///                     "format": "basic_date_time"
///                 },
///                 "my_string": {
///                     "type": "text",
///                     "fields": {
///                         "keyword":{
///                             "type":"keyword",
///                             "ignore_above":256
///                         }
///                     }
///                 },
///                 "my_num": {
///                     "type": "integer"
///                 }
///             }
///         }
///     }
/// }
/// # );
/// # assert_eq!(json, index);
/// # }
/// ```
///
/// Alternatively, you can implement serialisation manually for `MyIndex` and avoid having
/// to keep field names up to date if the document type name changes:
///
/// ```
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use serde::{Serialize, Serializer};
/// # use serde::ser::SerializeStruct;
/// # use elastic_types::prelude::*;
/// #[derive(Serialize, ElasticType)]
/// # pub struct MyType {
/// #     pub my_date: Date<DefaultDateFormat>,
/// #     pub my_string: String,
/// #     pub my_num: i32
/// # }
/// #[derive(Default, Serialize)]
/// pub struct MyIndex {
///     mappings: Mappings
/// }
///
/// #[derive(Default)]
/// struct Mappings;
/// impl Serialize for Mappings {
///     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
///         let mut state = try!(serializer.serialize_struct("mappings", 1));
///
///         try!(state.serialize_field(MyType::name(), &Document::from(MyType::mapping())));
///
///         state.end()
///     }
/// }
/// # fn main() {
/// # let index = serde_json::to_string(&MyIndex::default()).unwrap();
/// # let json = json_str!(
/// # {
/// #     "mappings": {
/// #         "mytype": {
/// #             "properties": {
/// #                 "my_date": {
/// #                     "type": "date",
/// #                     "format": "basic_date_time"
/// #                 },
/// #                 "my_string": {
/// #                     "type": "text",
/// #                     "fields": {
/// #                         "keyword":{
/// #                             "type":"keyword",
/// #                             "ignore_above":256
/// #                         }
/// #                     }
/// #                 },
/// #                 "my_num": {
/// #                     "type": "integer"
/// #                 }
/// #             }
/// #         }
/// #     }
/// # }
/// # );
/// # assert_eq!(json, index);
/// # }
/// ```
#[derive(Default)]
pub struct Document<M>
    where M: DocumentMapping
{
    _m: PhantomData<M>,
}

impl<M> From<M> for Document<M>
    where M: DocumentMapping
{
    fn from(_: M) -> Self {
        Document::<M>::default()
    }
}

/// A wrapper type used to work around conflicting implementations of `Serialize`
/// for the various mapping traits.
///
/// Serialising `Field` will produce the mapping for the given type,
/// suitable as the mapping of a field for a document.
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

/// Serialise a field mapping using the given serialiser.
#[inline]
pub fn field_ser<S, M, F>(state: &mut S, field: &'static str, _: M) -> Result<(), S::Error>
    where S: SerializeStruct,
          M: FieldMapping<F>,
          F: Default
{
    state.serialize_field(field, &M::Field::default())
}

/// Serialise a document mapping using the given serialiser.
#[inline]
pub fn doc_ser<S, M>(state: &mut S, field: &'static str, _: M) -> Result<(), S::Error>
    where S: SerializeStruct,
          M: DocumentMapping
{
    state.serialize_field(field, &Document::<M>::default())
}
