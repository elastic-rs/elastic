/*!
Elasticsearch API Client

A modular and efficient native client for the Elasticsearch REST API.

# Supported Versions

 `elastic`       | Elasticsearch
 --------------- | -------------
 `0.x`           | `5.x`

This crate depends heavily on the following crates:

- [`reqwest`/`hyper`][reqwest] as the default HTTP layer
- [`serde`/`serde_json`][serde] for serialisation
- [`futures`/`tokio`][tokio] for async io.

`elastic` is designed to scale up to the complexity of Elasticsearch's API, and with the complexity of the environments Elasticsearch is deployed in.

# Usage

This crate is on [crates.io][crates-io].
To get stated, add `elastic` to your `Cargo.toml`:

```ignore
[dependencies]
elastic = "*"
elastic_derive = "*"
```

The following optional dependencies may also be useful:

```ignore
serde = "*"
serde_json = "*"
serde_derive = "*"
```

Then reference in your crate root:

```
# fn main() {}
extern crate elastic;
#[macro_use]
extern crate elastic_derive;
```

# Examples

## Creating a synchronous client

The [`SyncClient`][SyncClient] type is an easy way to interact with an Elasticsearch cluster.
A synchronous client can be created through the [`SyncClientBuilder`][SyncClientBuilder].

The builder allows you to configure default parameters for all requests:

```no_run
# extern crate elastic;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
use elastic::prelude::*;
use elastic::http::header::Authorization;

let builder = SyncClientBuilder::new()
    .static_node("http://es_host:9200")
    .params_fluent(|p| p
        .url_param("pretty", true)
        .header(Authorization("let me in".to_owned())));

let client = builder.build()?;
# Ok(())
# }
```

Individual requests can override these parameter values:

```no_run
# extern crate elastic;
# extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;

let response = client.search::<Value>()
                     .params_fluent(|p| p.url_param("pretty", false))
                     .send()?;
# Ok(())
# }
```

`elastic` also offers an [`AsyncClient`][AsyncClient].
For more details, see the [`client`][client-mod] and [`requests`][requests-mod] modules.

## Making requests

_For a list of common client methods, see [here][request-builders]._

Each endpoint in the Elasticsearch REST API is provided as a strongly-typed structure.
The client offers high-level request builders for some common Elasticsearch operations.

### Getting and Indexing documents

The [Document Mapping API][docs-mapping] is provided as a custom derive plugin and set of Rust traits.
Derive `Serialize`, `Deserialize` and `ElasticType` on your document types:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
#[derive(Serialize, Deserialize, ElasticType)]
struct MyType {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateMapping>
}
# Ok(())
# }
```

Call [`Client.document_put_mapping`][Client.document_put_mapping] to ensure an index has the right mapping for your document types:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = SyncClientBuilder::new().build()?;
client.document_put_mapping::<MyType>(index("myindex"))
      .send()?;
# Ok(())
# }
```

Then call [`Client.document_index`][Client.document_index] to index documents in Elasticsearch:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClientBuilder::new().build()?;
let doc = MyType {
    id: 1,
    title: String::from("A title"),
    timestamp: Date::now()
};

let response = client.document_index(index("myindex"), id(doc.id), doc)
                     .send()?;
# Ok(())
# }
```

Call [`Client.document_get`][Client.document_get] to retrieve a single document from an index:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClientBuilder::new().build()?;
let response = client.document_get::<MyType>(index("myindex"), id(1))
                     .send()?;

if let Some(doc) = response.into_document() {
    println!("id: {}", doc.id);
}
# Ok(())
# }
```

For more details on document types, see the [`types`][types-mod] module.

### Searching documents

Call [`Client.search`][Client.search] to execute [Query DSL][docs-search] queries:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Debug, Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = SyncClientBuilder::new().build()?;
let response = client.search::<MyType>()
                     .index("myindex")
                     .body(json!({
                         "query": {
                            "query_string": {
                                "query": "*"
                            }
                         }
                     }))
                     .send()?;

// Iterate through the hits (of type `MyType`)
for hit in response.hits() {
    println!("{:?}", hit);
}
# Ok(())
# }
```

# Crate design

This crate is mostly a meta-package composed of a number of smaller pieces including:

- `elastic_requests` API request builders
- `elastic_responses` API response parsers
- `elastic_types` tools for document and mapping APIs

This crate glues these libraries together with some simple assumptions about how they're going to be used.

# Links

- [Elasticsearch Docs][docs-root]
- [Github][github]

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://serde.rs/
[tokio]: https://tokio.rs
[crates-io]: https://crates.io/crates/elastic
[github]: https://github.com/elastic-rs/elastic

[docs-root]: https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html
[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[docs-search]: http://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html

[SyncClient]: client/type.SyncClient.html
[SyncClientBuilder]: client/struct.SyncClientBuilder.html
[AsyncClient]: client/type.AsyncClient.html
[Client]: client/struct.Client.html
[Client.document_put_mapping]: client/struct.Client.html#method.document_put_mapping
[Client.document_index]: client/struct.Client.html#method.document_index
[Client.document_get]: client/struct.Client.html#method.document_get
[Client.search]: client/struct.Client.html#method.search
[client-mod]: client/index.html
[requests-mod]: client/requests/index.html
[types-mod]: types/index.html
[request-builders]: client/index.html#request-builders
*/

//#![deny(warnings, missing_docs)]
#![allow(unknown_lints, doc_markdown)]

extern crate bytes;
extern crate elastic_requests;
extern crate elastic_responses;
extern crate elastic_types;
#[macro_use]
extern crate error_chain;
extern crate fluent_builder;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate url;
extern crate uuid;

pub mod error;
pub use error::Error;

mod private {
    pub trait Sealed {}
}

pub mod http;
pub mod client;
pub mod types;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use client::prelude::*;
    pub use types::prelude::*;
}

#[cfg(test)]
mod tests {
    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}
}
