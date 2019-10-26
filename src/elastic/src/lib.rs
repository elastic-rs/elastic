/*!
Elasticsearch API Client

A modular and efficient native client for the Elasticsearch REST API.

# Supported Versions

 `elastic`          | Elasticsearch
 ------------------ | -------------
 `0.0.x` - `0.20.x` | `5.x`
 `0.21.x`           | `7.x`

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
elastic = "~0.21.0-pre.5"
elastic_derive = "~0.21.0-pre.5"
```

The following optional dependencies may also be useful:

```ignore
serde = "~1"
serde_json = "~1"
serde_derive = "~1"
```

Then reference in your crate root:

```
# fn main() -> Result<(), Box<dyn ::std::error::Error>> { Ok(()) }
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
# use elastic::prelude::*;
# use std::str::FromStr;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
use elastic::http::header::{self, AUTHORIZATION, HeaderValue};

let auth = HeaderValue::from_str("let me in")?;

let builder = SyncClientBuilder::new()
    .static_node("http://es_host:9200")
    .params_fluent(move |p| p
        .url_param("pretty", true)
        .header(AUTHORIZATION, auth.clone()));

let client = builder.build()?;
# Ok(())
# }
```

Individual requests can override these parameter values:

```no_run
# #[macro_use] extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
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
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
#[derive(Serialize, Deserialize, ElasticType)]
struct MyType {
    #[elastic(id(expr = "id.to_string()"))]
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateMapping>
}
# Ok(())
# }
```

Call [`Client.document().put_mapping()`][Client.document.put_mapping] to ensure an index has the right mapping for your document types:

```no_run
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = SyncClientBuilder::new().build()?;
client.document::<MyType>()
      .put_mapping()
      .send()?;
# Ok(())
# }
```

Then call [`Client.document().index()`][Client.document.index] to index documents in Elasticsearch:

```no_run
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: String,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClientBuilder::new().build()?;
let doc = MyType {
    id: "1".to_owned(),
    title: String::from("A title"),
    timestamp: Date::now()
};

let response = client.document()
                     .index(doc)
                     .send()?;
# Ok(())
# }
```

Call [`Client.document().get()`][Client.document.get] to retrieve a single document from an index:

```no_run
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: String,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClientBuilder::new().build()?;
let response = client.document::<MyType>()
                     .get(1)
                     .send()?;

if let Some(doc) = response.into_document() {
    println!("id: {}", doc.id);
}
# Ok(())
# }
```

For more details on document types, see the [`types`][types-mod] module.

### Searching documents

Call [`Client.doument().search()`][Client.document.search] to execute [Query DSL][docs-search] queries:

```no_run
# #[macro_use] extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use elastic::prelude::*;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Debug, Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = SyncClientBuilder::new().build()?;
let response = client.document::<MyType>()
                     .search()
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
[Client.document.put_mapping]: client/struct.DocumentClient.html#put-mapping-request
[Client.document.index]: client/struct.DocumentClient.html#index-document-request
[Client.document.get]: client/struct.DocumentClient.html#get-document-request
[Client.document.search]: client/struct.DocumentClient.html#search-request
[client-mod]: client/index.html
[requests-mod]: client/requests/index.html
[types-mod]: types/index.html
[request-builders]: client/index.html#request-builders
*/

#![deny(warnings, missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate crossbeam_channel as channel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate elastic_derive;

mod genned;

/// Common url params like `Id` and `Index`.
///
/// The parameter types are basically just a wrapper around a maybe
/// owned string.
/// They can all be constructed from a `String` or an `&str`, but some
/// parameters may have other implementations in the future.
pub mod params {
    pub use super::genned::params::*;
}

/// REST API endpoints.
///
/// Each type corresponds to a single HTTP method on a single endpoint.
/// Request types have constructor functions that take the form
/// `for_param_1_param_2_param_n`, and accept a `Body` parameter if the underlying
/// method is a `POST` or `PUT`.
/// Other request parameters accept any type that can be converted into the
/// parameter type, usually a `String` or `&str`.
///
/// Request types don't take ownership of their inputs unless you pass in owned
/// data.
/// That means if some function expects a `SearchRequest<'static>` then you can
/// either use a `SearchRequest` with owned `String` inputs, or one that uses only
/// `'static` inputs.
pub mod endpoints {
    pub use super::genned::{
        endpoints::*,
        http::Endpoint,
    };
}

pub mod error;

mod private {
    pub trait Sealed {}
}

pub mod client;
pub mod http;
pub mod types;

pub use self::{
    client::{
        AsyncClient,
        SyncClient,
    },
    error::Error,
};

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{
        client::prelude::*,
        endpoints::*,
        http::empty_body,
        params::*,
        types::prelude::*,
    };
}

#[cfg(test)]
mod tests {
    pub fn assert_send<T: Send>() {}
    pub fn assert_sync<T: Sync>() {}
}
