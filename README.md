# [`elastic`](https://docs.rs/elastic/*/elastic/) [![Latest Version](https://img.shields.io/crates/v/elastic.svg)](https://crates.io/crates/elastic) [![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/elastic-rs/Lobby)

`elastic` is an efficient, modular API client for [Elasticsearch](https://github.com/elastic/elasticsearch) written in [Rust](https://www.rust-lang.org).
The API is targeting the Elastic Stack `5.x`.

`elastic` provides strongly-typed documents and weakly-typed queries.

Quick reference:

- [crates](#crates)
- [simple examples](https://github.com/elastic-rs/elastic/tree/master/src/elastic/examples)
- [example apps](https://github.com/elastic-rs/elastic/tree/master/examples)

## Stability

This crate is still quite unstable and is likely to continue to churn breaking releases over the near future with not-so-detailed changelogs.

If you run into any problems upgrading in your own open source projects feel free to [open up an issue](https://github.com/elastic-rs/elastic/issues) and we'll give you a hand. The goal is definitely to offer a stable API eventually.

## Build Status
Platform  | Channel | Status (`master`) | Status (`vNext`)
------------- | ------------- | ------------- | ------------
Linux / OSX  | Stable/Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic) | [![Build Status](https://travis-ci.org/elastic-rs/elastic.svg?branch=vNext)](https://travis-ci.org/elastic-rs/elastic)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/csa78tcumdpnbur2?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic) | [![Build status](https://ci.appveyor.com/api/projects/status/csa78tcumdpnbur2/branch/vNext?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic/branch/vNext)

## Documentation

Version                | Docs
---------------------- | -------------
current (`master`)     | [![Documentation](https://img.shields.io/badge/docs-rustdoc-blue.svg)](https://docs.rs/elastic/*/elastic/)
unstable `vNext`       | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](http://elastic-rs.github.io/elastic/elastic/index.html)

## Example

Add `elastic` to your `Cargo.toml`:

```toml
[dependencies]
elastic = "*"
elastic_derive = "*"
serde_json = "*"
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
let client = SyncClientBuilder::new().build()?;

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
	pub id: i32,
	pub title: String,
	pub timestamp: Date<DefaultDateMapping<EpochMillis>>,
	pub content: Text<DefaultTextMapping>,
}
```

And you can start using `MyDocument` in `Client` request methods.

See the [docs](https://docs.rs/elastic/*/elastic/types/index.html) for more details.

## Alternatives

If you'd like to use a strongly-typed Query DSL builder see [`rs-es`](https://github.com/benashford/rs-es). This client does the hard work of providing an idiomatic Rust API for interacting with Elasticsearch. It has the advantage of letting you know your queries will parse at compile-time instead of runtime.

## Goals

To provide a full-featured and efficient Elasticsearch client for Rust over asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue. `elastic` is aimed at people who need to work with Elasticsearch and are considering using Rust, as well as users that are already using Rust. We want to offer a solution to interacting with Elasticsearch that's compelling from both within and outside the Rust ecosystem.

The REST API is covered by a simple inline JSON macro like `serde_json`'s [`json!`](https://docs.serde.rs/serde_json/macro.json.html) macro or the [`json_str!`](https://github.com/KodrAus/json_str) macro so it's always possible to build any query. This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Dev Tools](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) could just be copy+pasted into your Rust source.

The core focus of this project is on strong typing over your document types and query responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules as necessary.

## Development

`elastic` targets the `stable` channel, so it doesn't use any unstable features, but we'd like to track where improvements can be made by unstable features once they stabilise. There is another [GitHub Project](https://github.com/orgs/elastic-rs/projects/8) to record these possible enhancements.

The `elastic` crate brings a few independent crates together into a cohesive API. It aims to provide the glue between them and offer some good defaults. If you have a more specialised use-case, you can pick and choose the crates that will best support it. See the [crates](#crates) section for a full list.

`elastic` sits on a stack with hard dependencies on the following libraries:

- `reqwest`/`hyper` for HTTP transport
- `serde` for serialisation

There hasn't been much effort put into abstracting these dependencies at this stage, and `elastic` can't stabilise until these libraries and a few others do.

### Branches

The `master` branch should always be just about current with what's released on `crates.io`. Any non-breaking changes will be merged straight into `master` and released.

The `vNext` branch is where breaking changes for upcoming releases are collected. Once we're ready for a new breaking release, we'll merge `vNext` into `master` and push out a new release.

If you'd like to work on a new feature, base off `master`, unless you depend on code that's already in `vNext`. If the feature can be implemented in a non-breaking way then we'll merge it in to `master` for you. If it can't then we'll merge it in to `vNext`.

### Methodology

The following is a simple set of guidelines that the codebase should follow. It's mostly a reminder to ourselves and not a hard set of rules.

#### Usability

- Make it difficult for callers to mess up invariants
- Avoid duplicating effort between crates
- Keep caller code obvious and auditable
- Avoid unnecessary dependencies so callers don't end up depending on stuff they don't want to
- Avoid panicking

#### Performance

- Minimise heap allocations and copies where possible while staying ergonomic
- Cover features with micro-benchmarks to support performance investigation and catch unexpected regressions

#### Testing

- Try cover the whole public API surface so breaking changes result in broken tests/examples
- Features should fail to compile if used incorrectly rather than having surprising behaviour at runtime

#### Documentation

- Types should have detailed docs with general examples
- Type methods should have examples and document any panics/error cases
- Modules should have general guidance for the types they contain
- Make it easy to navigate between related types. `elastic` uses a lot of generic code that can be hard to follow, so we need to work hard to help the user follow what's happening

## Navigating the repository

`elastic` bundles up a couple of crates into a single client. This might make it difficult to find your way around the codebase when following items or finding out where a change should live.

### `elastic`

This is the main crate that bundles up `elastic_requests`, `elastic_types`, `elastic_requests` and `elastic_responses`.

### `elastic_reqwest`

A synchronous [`reqwest`](https://github.com/seanmonstar/reqwest) implementation of the Elasticsearch REST API.

### `elastic_requests`

Zero-copy request types for the REST API endpoints. These are automatically generated from the official spec.

### `elastic_responses`

Idiomatic support for inspecting Elasticsearch responses and iterating over hits.

### `elastic_types`

A library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.

## License

Licensed under either of these:
 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
