//! Elasticsearch API Client
//!
//! A modular and efficient native client for the Elasticsearch REST API.
//!
//! # Supported Versions
//!
//!  `elastic`       | Elasticsearch
//!  --------------- | -------------
//!  `0.x`           | `5.x`
//!
//! The client provides a flexible API with a default happy-path so you can customise the
//! way you use it.
//! It depends heavily on the following crates:
//!
//! - [`reqwest`/`hyper`](https://github.com/seanmonstar/reqwest) as the default HTTP layer
//! - [`serde`/`serde_json`](https://serde.rs/) for serialisation.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/elastic).
//! To get stated, add `elastic` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic = "*"
//! ```
//!
//! The following optional dependencies may also be useful:
//!
//! ```ignore
//! elastic_derive = "*"
//! json_str = "*"
//! serde = "*"
//! serde_json = "*"
//! serde_derive = "*"
//! ```
//!
//! Then reference in your crate root:
//!
//! ```
//! extern crate elastic;
//!
//! // Optional
//! extern crate serde;
//! extern crate serde_json;
//! #[macro_use]
//! extern crate serde_derive;
//! #[macro_use]
//! extern crate elastic_derive;
//! #[macro_use]
//! extern crate json_str;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! ## Making requests
//!
//! Each endpoint in the Elasticsearch REST API is provided as a strongly-typed
//! structure.
//! Use a `Client` instance to send one of these requests and read the response:
//!
//! ```no_run
//! use elastic::prelude::*;
//!
//! // Create a client with default params (host: 'http://localhost:9200')
//! let client = Client::new(RequestParams::default()).unwrap();
//!
//! // A ping request (HEAD '/')
//! let req = PingRequest::new();
//!
//! // Send the ping request and unwrap the response
//! let response = client.request(req).send().unwrap();
//! ```
//!
//! The `Client` will use a default set of request parameters that are passed to each request.
//! Properties like the host and query parameters can be configured:
//!
//! ```no_run
//! # use elastic::prelude::*;
//! let params = RequestParams::new("http://es_host:9200").url_param("pretty", true);
//!
//! let client = Client::new(params).unwrap();
//! ```
//!
//! Individual requests can override these parameter values:
//!
//! ```no_run
//! # use elastic::prelude::*;
//! # let params = RequestParams::new("http://es_host:9200");
//! # let client = Client::new(params).unwrap();
//! # let req = PingRequest::new();
//! let response = client.request(req)
//!                      .params(|p| p.url_param("pretty", false))
//!                      .send()
//!                      .unwrap();
//! ```
//!
//! For more details, see the [`client`](client/index.html) and [`requests`](client/requests/index.html) modules.
//!
//! ## Getting Responses
//!
//! Call `response` on a sent request to get a strongly typed `SearchResponse` or `GetResponse`:
//!
//! ```no_run
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # struct MyType {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub timestamp: Date<DefaultDateFormat>
//! # }
//! # let params = RequestParams::new("http://es_host:9200");
//! # let client = Client::new(params).unwrap();
//! # let req = PingRequest::new();
//! let response = client.request(req)
//!                      .send()
//!                      .and_then(|res| res.response::<SearchResponse<MyType>>());
//! # }
//! ```
//!
//! Call `raw` on a sent request to get a raw `HttpResponse`:
//!
//! ```no_run
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # let params = RequestParams::new("http://es_host:9200");
//! # let client = Client::new(params).unwrap();
//! # let req = PingRequest::new();
//! let response = client.request(req)
//!                      .send()
//!                      .map(|res| res.raw());
//! # }
//! ```
//!
//! The `HttpResponse` implements `Read` so you can buffer out the raw
//! response data.
//!
//! For more details see the [`client`](client/index.html) and [`responses`](client/responses/index.html) module.
//!
//! ## Defining document types
//!
//! The Mapping API is provided as a custom derive plugin and Rust traits.
//! Derive `Serialize`, `Deserialize` and `ElasticType` on your document types:
//!
//! ```no_run
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! #[derive(Serialize, Deserialize, ElasticType)]
//! struct MyType {
//!     pub id: i32,
//!     pub title: String,
//!     pub timestamp: Date<DefaultDateFormat>
//! }
//! # }
//! ```
//!
//! Use your document type to build index requests:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # struct MyType {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub timestamp: Date<DefaultDateFormat>
//! # }
//! let doc = MyType {
//!     id: 1,
//!     title: String::from("A title"),
//!     timestamp: Date::now()
//! };
//!
//! let index = Index::from("index");
//! let id = Id::from(doc.id.to_string());
//!
//! // A tuple of (Index, Id, MyType) can be converted into an IndexRequest
//! let req = IndexRequest::try_for_doc((index, id, &doc)).unwrap();
//! # }
//! ```
//!
//! Use your document type to build mapping requests:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # #[macro_use]
//! # extern crate elastic_derive;
//! # extern crate elastic;
//! # use elastic::prelude::*;
//! # fn main() {
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # struct MyType {
//! #     pub id: i32,
//! #     pub title: String,
//! #     pub timestamp: Date<DefaultDateFormat>
//! # }
//! let index = Index::from("index");
//! let mapping = MyType::mapping();
//!
//! // A tuple of (Index, MyTypeMapping) can be converted into a MappingRequest
//! let req = IndicesPutMappingRequest::try_for_mapping((index, mapping)).unwrap();
//! # }
//! ```
//!
//! For more details on document types, see the [`types`](types/index.html) module.
//!
//! # Crate design
//!
//! This crate is mostly a meta-package composed of a number of smaller pieces including:
//!
//! - `elastic_reqwest` HTTP transport
//! - `elastic_requests` API request builders
//! - `elastic_responses` API response parsers
//! - `elastic_types` tools for document and mapping APIs
//!
//! This crate glues these libraries together with some simple assumptions
//! about how they're going to be used.
//!
//! # Links
//!
//! - [Elasticsearch Docs](https://www.elastic.co/guide/en/elasticsearch/reference/master/index.html)
//! - [Github](https://github.com/elastic-rs/elastic)

#![deny(warnings)]
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_types;
extern crate elastic_responses;

mod impls;

pub mod error;

pub mod http {
    //! HTTP headers and status codes.

    pub use reqwest::header;
}

pub mod client;

pub mod types {
    //! Indexable documents and type mapping.
    //!
    //! This module contains tools for defining Elasticsearch-compatible
    //! document types.
    //! Document mapping is defined using Rust traits, which are added to fields
    //! as generic parameters.
    //! This has the following benefits:
    //!
    //! - Your `struct` is the one source of truth for serialisation and mapping.
    //! There's no extra mapping function to maintain.
    //! - Mapping is immutable and zero-cost. You don't pay anything at runtime
    //! for having mapping metadata available.
    //!
    //! # Document and data types
    //!
    //! Types in Elasticsearch are a combination of _source_ and _mapping_.
    //! The source is the data (like `42` or `"my string"`) and the mapping is metadata about how to
    //! interpret and use the data (like the format of a date string).
    //!
    //! The approach `elastic` takes to types is to bundle the mapping up as a _Zero Sized Type_,
    //! which is then bound to a field type as a generic parameter. For example:
    //!
    //! ```ignore
    //! some_field: Boolean<MyMapping>
    //! ```
    //!
    //! The source type is `boolean` and the mapping is `MyMapping`.
    //!
    //! Most datatypes also implement a default mapping for a common Rust type if you don't
    //! need to customise how a field is mapped:
    //!
    //! ```ignore
    //! some_field: bool
    //! ```
    //!
    //! See the table below for a complete list of supported datatypes and their default
    //! implementations.
    //!
    //! All Elasticsearch types implement the base `FieldType<M: FieldMapping<F>, F>` trait
    //! where `M` is the mapping and `F` is a type-specific format.
    //!
    //! ## Supported datatypes
    //!
    //! The following table illustrates the types provided by `elastic`:
    //!
    //!  Elasticsearch Type  | Rust Type (Default Mapping) | Crate     | Rust Type (Custom Mapping)                                                       | Format Type
    //!  ------------------- | --------------------------- | --------- | -------------------------------------------------------------------------------- | -----------------
    //!  `object`            | -                           | -         | type implementing [`DocumentType<M>`](document/index.html)                       | -
    //!  `integer`           | `i32`                       | `std`     | [`Integer<M>`](number/index.html)                                                | -
    //!  `long`              | `i64`                       | `std`     | [`Long<M>`](number/index.html)                                                   | -
    //!  `short`             | `i16`                       | `std`     | [`Short<M>`](number/index.html)                                                  | -
    //!  `byte`              | `i8`                        | `std`     | [`Byte<M>`](number/index.html)                                                   | -
    //!  `float`             | `f32`                       | `std`     | [`Float<M>`](number/index.html)                                                  | -
    //!  `double`            | `f64`                       | `std`     | [`Double<M>`](number/index.html)                                                 | -
    //!  `keyword`           | -                           | -         | [`Keyword<M>`](string/index.html)                                                | -
    //!  `text`              | `String`                    | `std`     | [`Text<M>`](string/index.html)                                                   | -
    //!  `boolean`           | `bool`                      | `std`     | [`Boolean<M>`](boolean/index.html)                                               | -
    //!  `ip`                | `Ipv4Addr`                  | `std`     | [`Ip<M>`](ip/index.html)                                                         | -
    //!  `date`              | `DateTime<UTC>`             | `chrono`  | [`Date<F, M>`](date/index.html)                                                  | `DateFormat`
    //!  `geo_point`         | `Point`                     | `geo`     | [`GeoPoint<F, M>`](geo/point/index.html)                                         | `GeoPointFormat`
    //!  `geo_shape`         | -                           | `geojson` | [`GeoShape<M>`](geo/shape/index.html)                                            | -
    //!
    //! ## Mapping
    //!
    //! Having the mapping available at compile-time captures the fact that a mapping is static and tied
    //! to the data type.
    //!
    //! Where there's a `std` type that's equivalent to an Elasticsearch type (like `i32` for `integer`),
    //! a default mapping is implemented for that type.
    //! That means you can use primitives in your structs and have them mapped to the correct type in Elasticsearch.
    //! If you want to provide your own mapping for a `std` type, there's also a struct provided by `elastic_types`
    //! that wraps the `std` type but also takes an explicit mapping (like `Integer` which implements `Deref<Target = i32>`).
    //!
    //! Where there isn't a `std` type available (like `date`), an external crate is used and an implementation of
    //! that type is provided (like `Date`, which implements `Deref<Target = chrono::DateTime<UTC>>`).
    //!
    //! ## Formats
    //!
    //! For some types (like `Date`), it's helpful to have an extra generic parameter that describes the
    //! way the data can be interpreted. For most types the format isn't exposed, because there aren't any alternative formats available.
    //! This is a particularly helpful feature for serialisation.
    //!
    //! # Examples
    //!
    //! ## Derive document mapping
    //!
    //! Document types must derive `serde`'s [serialisation traits]().
    //! Use simple generic types to annotate your Rust structures with Elasticsearch
    //! document field mappings:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! #[derive(Serialize, Deserialize, ElasticType)]
    //! struct MyType {
    //!     // Mapped as an `integer` field
    //!     id: i32,
    //!     // Mapped as a `text` field with a `keyword` subfield
    //!     title: String,
    //!     // Mapped as a `date` field with an `epoch_millis` format
    //!     timestamp: Date<EpochMillis>
    //! }
    //! # }
    //! ```
    //!
    //! You can use the `Document` type wrapper to serialise the mapping for your
    //! document types:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # extern crate serde_json;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize, ElasticType)]
    //! # struct MyType {}
    //! let doc = Document::from(MyType::mapping());
    //!
    //! let mapping = serde_json::to_string(&doc).unwrap();
    //! # }
    //! ```
    //!
    //! This will produce the following JSON:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # extern crate serde_json;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # #[macro_use]
    //! # extern crate json_str;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize, ElasticType)]
    //! # struct MyType {
    //! #     id: i32,
    //! #     title: String,
    //! #     timestamp: Date<EpochMillis>
    //! # }
    //! # let mapping = serde_json::to_string(&Document::from(MyType::mapping())).unwrap();
    //! # let expected = json_str!(
    //! {
    //!     "properties": {
    //!         "id": {
    //!             "type": "integer"
    //!         },
    //!         "title": {
    //!             "type": "text",
    //!             "fields": {
    //!                 "keyword": {
    //!                     "type": "keyword",
    //!                     "ignore_above": 256
    //!                 }
    //!             }
    //!         },
    //!         "timestamp": {
    //!             "type": "date",
    //!             "format": "epoch_millis"
    //!         }
    //!     }
    //! }
    //! # );
    //! # assert_eq!(expected, mapping);
    //! # }
    //! ```
    //!
    //! See the table above for a list of all supported datatypes and how to work
    //! with them.
    //!
    //! ## Define custom field data types
    //!
    //! Use traits to define your own field types and have them mapped as one of the
    //! core datatypes.
    //! In the below example, variants of `MyEnum` will be serialised as a string,
    //! which we map as a non-analysed `keyword` in Elasticsearch:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize)]
    //! enum MyEnum {
    //!     OptionA,
    //!     OptionB,
    //!     OptionC
    //! }
    //!
    //! // Map `MyEnum` as a `keyword` in Elasticsearch, but treat it as an enum in Rust
    //! impl FieldType<DefaultKeywordMapping, KeywordFormat> for MyEnum {}
    //! # }
    //! ```
    //!
    //! You can then use `MyEnum` on any document type:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize)]
    //! # enum MyEnum {}
    //! # impl FieldType<DefaultKeywordMapping, KeywordFormat> for MyEnum {}
    //! #[derive(Serialize, Deserialize, ElasticType)]
    //! struct MyType {
    //!     value: MyEnum
    //! }
    //! # }
    //! ```
    //!
    //! Serialising `MyType`s mapping will produce the following json:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # extern crate serde_json;
    //! # #[macro_use]
    //! # extern crate json_str;
    //! # use elastic::prelude::*;
    //! # #[derive(Serialize, Deserialize)]
    //! # enum MyEnum {}
    //! # impl FieldType<DefaultKeywordMapping, KeywordFormat> for MyEnum {}
    //! # #[derive(Serialize, Deserialize, ElasticType)]
    //! # struct MyType {
    //! #     value: MyEnum
    //! # }
    //! # fn main() {
    //! # let mapping = serde_json::to_string(&Document::from(MyType::mapping())).unwrap();
    //! # let expected = json_str!(
    //! {
    //!     "properties": {
    //!         "value": {
    //!             "type": "keyword"
    //!         }
    //!     }
    //! }
    //! # );
    //! # assert_eq!(expected, mapping);
    //! # }
    //! ```
    //!
    //! ## Convert documents into requests
    //!
    //! Documents and their mappings can be converted into index and
    //! mapping REST API requests.
    //!
    //! Convert a document and index type into an index request:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # extern crate serde_json;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize, ElasticType)]
    //! # struct MyType {}
    //! # fn get_doc() -> MyType { MyType {} }
    //! // Get an `Index` and an instance of some `ElasticType`
    //! let index = Index::from("my_index");
    //! let doc = get_doc();
    //!
    //! // Convert the index and document into an index request
    //! let req = IndexRequest::try_for_doc((index, &doc)).unwrap();
    //! # }
    //! ```
    //!
    //! Convert a document and index type into a mapping request:
    //!
    //! ```
    //! # extern crate elastic;
    //! # #[macro_use]
    //! # extern crate elastic_derive;
    //! # extern crate serde;
    //! # extern crate serde_json;
    //! # #[macro_use]
    //! # extern crate serde_derive;
    //! # use elastic::prelude::*;
    //! # fn main() {
    //! # #[derive(Serialize, Deserialize, ElasticType)]
    //! # struct MyType {}
    //! # fn get_doc() -> MyType { MyType {} }
    //! // Get an `Index` and an instance of some `ElasticType`
    //! let index = Index::from("my_index");
    //! let doc = get_doc();
    //!
    //! // Convert the index and document into an index request
    //! let req = IndicesPutMappingRequest::try_for_doc((index, &doc)).unwrap();
    //! # }
    //! ```
    //!
    //! For more conversions between documents and requests,
    //! see the [`TryForDoc`](../client/requests/trait.TryForDoc.html) and
    //! [`TryForMapping`](../client/requests/trait.TryForMapping.html) traits.

    pub use elastic_types::{document, field, boolean, date, geo, ip, number, string, prelude};
}

pub mod prelude {
    //! A glob import for convenience.

    pub use super::client::{Client, RequestParams, RequestBuilder, SearchRequestBuilder,
                            ResponseBuilder, into_response, into_raw};
    pub use super::client::requests::*;
    pub use super::client::responses::*;
    pub use super::types::prelude::*;
}
