/*!
Elasticsearch API Client

A modular and efficient native client for the Elasticsearch REST API.

# Supported Versions

 `elastic`       | Elasticsearch
 --------------- | -------------
 `0.x`           | `5.x`

The client provides a flexible API with a default happy-path so you can customise the
way you use it.
It depends heavily on the following crates:

- [`reqwest`/`hyper`][reqwest] as the default HTTP layer
- [`serde`/`serde_json`][serde] for serialisation.

`elastic` is designed to scale up to the complexity of Elasticsearch's API, and with the complexity
of the environments Elasticsearch is deployed in.

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
json_str = "*"
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

## Creating a client

The [`Client`][Client] type is used to make interact with an Elasticsearch cluster.
The `Client` will use a default set of request parameters that are passed to each request.
Properties like the host and query parameters can be configured for all requests:

```no_run
use elastic::prelude::*;
use elastic::http::header::Authorization;

let builder = ClientBuilder::new()
    .base_url("http://es_host:9200")
    .params(|p| p
        .url_param("pretty", true)
        .header(Authorization("let me in".to_owned())));

let client = builder.build().unwrap();
```

Individual requests can override these parameter values:

```no_run
# extern crate elastic;
# extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() {
let client = ClientBuilder::new().build().unwrap();

let response = client.search::<Value>()
                     .params(|p| p.url_param("pretty", true))
                     .send()
                     .unwrap();
# }
```

For more details, see the [`client`][client-mod] and [`requests`][requests-mod] modules.

## Making requests

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
# fn main() {
#[derive(Serialize, Deserialize, ElasticType)]
struct MyType {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateFormat>
}
# }
```

Call [`Client.put_mapping`][Client.put_mapping] to ensure an index has the right mapping for your document types:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = ClientBuilder::new().build().unwrap();;
client.put_mapping::<MyType>(index("myindex"))
      .send()
      .unwrap();
# }
```

Then call [`Client.index_document`][Client.index_document] to index documents in Elasticsearch:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateFormat>
# }
# let client = ClientBuilder::new().build().unwrap();;
let doc = MyType {
    id: 1,
    title: String::from("A title"),
    timestamp: Date::now()
};

let response = client.index_document(index("myindex"), id(doc.id), doc)
                     .send()
                     .unwrap();
# }
```

Call [`Client.get_document`][Client.get_document] to retrieve a single document from an index:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateFormat>
# }
# let client = ClientBuilder::new().build().unwrap();;
let response = client.get_document::<MyType>(index("myindex"), id(1))
                     .send()
                     .unwrap();

if let Some(doc) = response.source {
    println!("id: {}", doc.id);
}
# }
```

For more details on document types, see the [`types`][types-mod] module.

### Searching documents

Call [`Client.search`][Client.search] to execute [Query DSL][docs-search] queries:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] extern crate json_str;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Debug, Serialize, Deserialize, ElasticType)]
# struct MyType { }
# let client = ClientBuilder::new().build().unwrap();;
let response = client.search::<MyType>()
                     .index("myindex")
                     .body(json_str!({
                         query: {
                            query_string: {
                                query: "*"
                            }
                         }
                     }))
                     .send()
                     .unwrap();

// Iterate through the hits (of type `MyType`)
for hit in response.hits() {
    println!("{:?}", hit);
}
# }
```

# Crate design

This crate is mostly a meta-package composed of a number of smaller pieces including:

- [`elastic_reqwest`][elastic-reqwest] HTTP transport
- [`elastic_requests`][elastic-requests] API request builders
- [`elastic_responses`][elastic-responses] API response parsers
- [`elastic_types`][elastic-types] tools for document and mapping APIs

This crate glues these libraries together with some simple assumptions about how they're going to be used.

# Links

- [Elasticsearch Docs][docs-root]
- [Github][github]

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://serde.rs/
[crates-io]: https://crates.io/crates/elastic
[github]: https://github.com/elastic-rs/elastic

[elastic-reqwest]: https://github.com/elastic-rs/elastic-reqwest
[elastic-requests]: https://github.com/elastic-rs/elastic-requests
[elastic-responses]: https://github.com/elastic-rs/elastic-responses
[elastic-types]: https://github.com/elastic-rs/elastic-types

[docs-root]: https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html
[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[docs-search]: http://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html

[Client]: client/struct.Client.html
[Client.put_mapping]: client/struct.Client.html#method.put_mapping
[Client.index_document]: client/struct.Client.html#method.index_document
[Client.get_document]: client/struct.Client.html#method.get_document
[Client.search]: client/struct.Client.html#method.search
[client-mod]: client/index.html
[requests-mod]: client/requests/index.html
[types-mod]: types/index.html
*/

#![deny(warnings)]
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate elastic_reqwest;
extern crate elastic_types;

pub mod error;

pub mod http {
    /*! 
    Raw HTTP modules.

    These types are re-exported from `reqwest` and used in parts of `elastic`s public API.
    They may eventually be wrapped and made implementation details.
    */

    pub use reqwest::header;
    pub use reqwest::Body;
}

pub mod client;
pub mod types;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use client::{ClientBuilder, Client, RequestParams, into_response, into_raw};
    pub use client::requests::*;
    pub use client::responses::*;
    pub use types::prelude::*;
}
