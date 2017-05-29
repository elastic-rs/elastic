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

- [`reqwest`/`hyper`](https://github.com/seanmonstar/reqwest) as the default HTTP layer
- [`serde`/`serde_json`](https://serde.rs/) for serialisation.

`elastic` is designed to scale up to the complexity of Elasticsearch's API, and with the complexity
of the environments Elasticsearch is deployed in.

# Usage

This crate is on [crates.io](https://crates.io/crates/elastic).
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

// Optional
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
```

# Examples

## Making requests

Each endpoint in the Elasticsearch REST API is provided as a strongly-typed structure.
The client offers high-level request builders for some common Elasticsearch operations, like `search`:

```no_run
# extern crate elastic;
# extern crate serde_json;
use serde_json::Value;
use elastic::prelude::*;

// Create a client with default params (host: 'http://localhost:9200')
let client = Client::new(RequestParams::default()).unwrap();

// Send the search request
let response = client.search::<Value>()
                     .index("myindex")
                     .ty("mytype")
                     .send()
                     .unwrap();

// Iterate through the hits (of type `Value`)
for hit in response.hits() {
    println!("{:?}", hit);
}
```

The `Client` will use a default set of request parameters that are passed to each request.
Properties like the host and query parameters can be configured for all requests:

```no_run
# use elastic::prelude::*;
let params = RequestParams::new("http://es_host:9200").url_param("pretty", true);

let client = Client::new(params).unwrap();
```

Individual requests can override these parameter values:

```no_run
# extern crate elastic;
# extern crate serde_json;
# use serde_json::Value;
use elastic::prelude::*;

let client = Client::new(RequestParams::default()).unwrap();

let response = client.search::<Value>()
                     .params(|p| p.url_param("pretty", true))
                     .send()
                     .unwrap();
```

For more details, see the [`client`](client/index.html) and [`requests`](client/requests/index.html) modules.

## Defining document types

The Mapping API is provided as a custom derive plugin and Rust traits.
Derive `Serialize`, `Deserialize` and `ElasticType` on your document types:

```no_run
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_derive;
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

Searching on a type that derives `ElasticType` will infer the right document type and deserialise appropriately:

```
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateFormat>
# }
let response = client.search::<MyType>()
                     .index("myindex")
                     .send()
                     .unwrap();
# }
```

Types that derive `ElasticType` can be indexed:

```
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateFormat>
# }
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

Types that derive `ElasticType` can be mapped:

```
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateFormat>
# }
let response = client.put_mapping::<MyType>(index("myindex"))
                     .send()
                     .unwrap();
# }
```

For more details on document types, see the [`types`](types/index.html) module.

# Crate design

This crate is mostly a meta-package composed of a number of smaller pieces including:

- [`elastic_reqwest`]() HTTP transport
- [`elastic_requests`]() API request builders
- [`elastic_responses`]() API response parsers
- [`elastic_types`]() tools for document and mapping APIs

This crate glues these libraries together with some simple assumptions
about how they're going to be used.

# Links

- [Elasticsearch Docs](https://www.elastic.co/guide/en/elasticsearch/reference/master/index.html)
- [Github](https://github.com/elastic-rs/elastic)
!*/

#![deny(warnings)]
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_types;

pub mod error;

pub mod http {
    /*! Raw HTTP modules. !*/

    pub use reqwest::header;
    pub use reqwest::Body;
}

pub mod client;
pub mod types;

pub mod prelude {
    /*! A glob import for convenience. !*/

    pub use client::{Client, RequestParams, into_response, into_raw};
    pub use client::requests::*;
    pub use client::responses::*;
    pub use types::prelude::*;
}
