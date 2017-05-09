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

pub mod error;

pub mod http {
    //! Raw HTTP modules.

    pub use reqwest::header;
}

pub mod client;
pub mod types;

pub mod prelude {
    //! A glob import for convenience.

    pub use super::client::{Client, RequestParams, into_response, into_raw};
    pub use super::client::requests::*;
    pub use super::client::responses::*;
    pub use super::types::prelude::*;
}
