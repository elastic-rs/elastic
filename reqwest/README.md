# [`elastic_reqwest`](https://docs.rs/elastic_reqwest/*/elastic_reqwest/) [![Latest Version](https://img.shields.io/crates/v/elastic_reqwest.svg)](https://crates.io/crates/elastic_reqwest)

Provides a no-fuss, synchronous [`reqwest`](https://github.com/seanmonstar/reqwest) implementation of the Elasticsearch REST API. The `reqwest` client is simple to use; there's basically no setup needed besides creating a `reqwest::Client` object to use for requests. The `reqwest` client is general-purpose and suitable for any scenario where on-demand requests are sufficient. It also splits the request process into a few logical methods that could be easily split across asynchronous boundaries.

This library is the HTTP backend for the higher-level [`elastic`](https://github.com/elastic-rs/elastic) client.

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-reqwest.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-reqwest)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/yvsqsyt4ioxa11g8?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic-hyper)

## Documentation

Version  | Docs
------------- | -------------
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_reqwest/*/elastic_reqwest/)

## Alternatives

If you'd prefer to call Elasticsearch using a strongly-typed Query DSL builder, see [`rs-es`](https://github.com/benashford/rs-es).

For a higher-level client that supports strongly-typed response parsing, document mapping see [`elastic`](https://github.com/elastic-rs/elastic). It uses `elastic_reqwest` as its HTTP layer.

## Example

The `elastic_reqwest` client is a thin layer over `reqwest`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
- For query serialisation, the [`json_str`](https://github.com/KodrAus/json_str) crate provides the `json_str!` macro for creating ad-hoc API queries.
- For type serialisation / deserialisation, see [`elastic_types`](https://github.com/elastic-rs/elastic-types).

Currently targeting the `master` Elasticsearch branch, aiming for `5.x`.
This will be stabilised through features in the future.

Add `elastic_reqwest` and `json_str` to your `Cargo.toml`:

```
[dependencies]
elastic_reqwest = "*"
reqwest = "*"

# Optional for request bodies
json_str = "*"
```

Ping the availability of your cluster:

```rust
extern crate elastic_reqwest as cli;
extern crate reqwest;

use cli::{ElasticClient, ParseResponse, parse};
use cli::req::PingRequest;
use cli::res::PingResponse;

let (client, params) = cli::default().unwrap();

let http_res = client.elastic_req(&params, PingRequest::new()).unwrap();

let parse_res = parse::<PingResponse>().from_response(http_res).unwrap();
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
extern crate elastic_reqwest as cli;
extern crate reqwest;

use cli::{ElasticClient, ParseResponse, parse};
use cli::req::SearchRequest;
use cli::res::SearchResponse;
 
let (client, params) = cli::default().unwrap();

let search = {
    let body = json_str!({
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
    });
    
    SearchRequest::for_index_ty("myindex", "mytype", body)
};

let http_res = client.elastic_req(&params, search).unwrap();

let search_res = parse::<SearchResponse>().from_response(http_res).unwrap();
```
