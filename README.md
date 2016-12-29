# [`elastic_reqwest`](https://docs.rs/elastic_hyper/*/elastic_hyper/) [![Latest Version](https://img.shields.io/crates/v/elastic_hyper.svg)](https://crates.io/crates/elastic_hyper)

Provides a synchronous [`reqwest`](https://github.com/seanmonstar/reqwest/) implementation of the Elasticsearch REST API. The `reqwest` client is simple to use; there's basically no setup needed besides creating a `elastic_reqwest::ElasticClient` object to use for requests. The `reqwest` client is general-purpose, and suitable for any scenario where on-demand requests are sufficient.

If you'd prefer to call Elasticsearch using a strongly-typed Query DSL builder, see [`rs-es`](https://github.com/benashford/rs-es).

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-hyper.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-hyper)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/yvsqsyt4ioxa11g8?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic-hyper)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic-hyper/elastic_hyper/)
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_hyper/*/elastic_hyper/)

## Example

The `elastic_reqwest` client is a thin layer over `reqwest`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
- For query serialisation, the [`json_str`](https://github.com/KodrAus/json_str) crate provides the `json_str!` macro for creating ad-hoc API queries.
- For type serialisation / deserialisation, see [`elastic_types`](https://github.com/elastic-rs/elastic-types).

Currently targeting the `master` Elasticsearch branch, aiming for `5.x`.
This will be stabilised through features in the future.

Add `elastic_reqwest` and `json_str` to your `Cargo.toml`:

```
[dependencies]
elastic_requests = "*"
elastic_reqwest = "*"
reqwest = "*"

# Optional
json_str = "*"
```

Ping the availability of your cluster:

```rust
extern crate elastic_requests as req;
extern crate elastic_reqwest as cli;
extern crate reqwest;

use cli::ElasticClient;
use req::PingRequest;

let (client, params) = cli::default();

client.elastic_req(&params, PingRequest::new()).unwrap();
```

Customise the location of the Elasticsearch cluster:
 
 ```rust
 let (mut client, mut params) = elastic::default();
 params.base_url = String::from("http://eshost:9200");
 ```

A query DSL query:

```rust
#[macro_use]
extern crate json_str;
extern crate elastic_requests as req;
extern crate elastic_reqwest as cli;
extern crate reqwest;

use cli::ElasticClient;
use req::SearchRequest;
 
let (client, params) = cli::default();

let search = SearchRequest::for_index_ty(
    "myindex", "mytype", 
    json_str!({
        query: {
            filtered: {
                query: {
                    match_all: {}
                },
                filter: {
                    geo_distance: {
                        distance: "20km",
                        location: {
                            lat: 37.776,
                            lon: -122.41
                        }
                    }
                }
            }
        }
    })
);

client.elastic_req(&params, search).unwrap();
```