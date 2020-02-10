# [`elastic`](https://docs.rs/elastic/*/elastic/) [![Latest Version](https://img.shields.io/crates/v/elastic.svg)](https://crates.io/crates/elastic) [![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/elastic-rs/Lobby)

`elastic` is an efficient, modular API client for [Elasticsearch](https://github.com/elastic/elasticsearch) written in [Rust](https://www.rust-lang.org).
The API is targeting the Elastic Stack `7.x`.

`elastic` provides strongly-typed documents and weakly-typed queries.

Quick reference:

- [simple examples](https://github.com/elastic-rs/elastic/tree/master/src/elastic/examples)
- [example apps](https://github.com/elastic-rs/elastic/tree/master/examples)

Also check out the official [**elasticsearch**](https://github.com/elastic/elasticsearch-rs) crate! 

## Stability

This crate is still quite unstable and is likely to continue to churn breaking releases over the near future with not-so-detailed changelogs.

If you run into any problems upgrading in your own open source projects feel free to [open up an issue](https://github.com/elastic-rs/elastic/issues) and we'll give you a hand. The goal is definitely to offer a stable API eventually.

## Build Status
Platform  | Channel | Status (`master`)
------------- | ------------- | -------------
Linux / macOS  | Stable/Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/csa78tcumdpnbur2?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic)
## Documentation

Version                | Docs
---------------------- | -------------
current (`master`)     | [![Documentation](https://img.shields.io/badge/docs-rustdoc-blue.svg)](https://docs.rs/elastic/*/elastic/)

## Example

Add `elastic` to your `Cargo.toml`:

```toml
[dependencies]
elastic = "0.21.0-pre.5"
elastic_derive = "0.21.0-pre.5"
serde_json = "1"
```

Create a `SyncClient` and start making requests:

```rust
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;

// A reqwest HTTP client and default parameters.
// The builder includes the base node url (http://localhost:9200).
let client = SyncClient::builder().build()?;

let query = "some query string";

// A search request with a freeform body.
let res = client.search::<Value>()
                .index("_all")
                .body(json!({
                    "query": {
                        "query_string": {
                            "query": query
                        }
                    }
                }))
                .send()?;

// Iterate through the hits in the response.
for hit in res.hits() {
    println!("{:?}", hit);
}
```

`elastic` also offers an `AsyncClient` for use with the `tokio` asynchronous io stack.
See the [examples](https://github.com/elastic-rs/elastic/tree/master/examples) folder for complete samples.

### Building documents

Document mapping is derived at compile-time from your _Plain Old Rust Structures_. Just add a `#[derive(ElasticType)]` attribute:

```rust
#[derive(ElasticType, Serialize, Deserialize)]
struct MyDocument {
	#[elastic(id)]
	pub id: String,
	pub title: String,
	pub timestamp: Date<DefaultDateMapping<EpochMillis>>,
	pub content: Text<DefaultTextMapping>,
}
```

And you can start using `MyDocument` in `Client` request methods.

See the [docs](https://docs.rs/elastic/*/elastic/types/index.html) for more details.

## Alternatives

[elastic.co](https://www.elastic.co) [released](https://www.elastic.co/blog/rust-client-for-elasticsearch-alpha-release) an official client, [elasticsearch](https://github.com/elastic/elasticsearch-rs). Although it is still in an alpha stage (as of 2020-02-10), it is very comprehensive and generates most of its code from the official REST API specifications.

Additionally, if you'd like to use a strongly-typed Query DSL builder see [`rs-es`](https://github.com/benashford/rs-es). This client does the hard work of providing an idiomatic Rust API for interacting with Elasticsearch. It has the advantage of letting you know your queries will parse at compile-time instead of runtime.

## Goals

To provide a full-featured and efficient Elasticsearch client for Rust over asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue. `elastic` is aimed at people who need to work with Elasticsearch and are considering using Rust, as well as users that are already using Rust. We want to offer a solution to interacting with Elasticsearch that's compelling from both within and outside the Rust ecosystem.

The REST API is covered by a simple inline JSON macro like `serde_json`'s [`json!`](https://docs.serde.rs/serde_json/macro.json.html) so it's always possible to build any query. This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Dev Tools](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) could just be copy+pasted into your Rust source.

The core focus of this project is on strong typing over your document types and query responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules as necessary.

## License

Licensed under either of these:
 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
