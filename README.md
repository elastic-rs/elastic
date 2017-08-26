# [`elastic`](https://docs.rs/elastic/*/elastic/) [![Latest Version](https://img.shields.io/crates/v/elastic.svg)](https://crates.io/crates/elastic)

`elastic` is an efficient, modular API client for [Elasticsearch](https://github.com/elastic/elasticsearch) written in [Rust](https://www.rust-lang.org).
The API is targeting the Elastic Stack `5.x`.

`elastic` provides strongly-typed documents and weakly-typed queries.

Quick reference:

- [crates](#crates)
- [examples](https://github.com/elastic-rs/elastic/tree/master/elastic/examples)

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable/Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/csa78tcumdpnbur2?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic)

## Documentation

Version  | Docs
------------- | -------------
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic/*/elastic/)

## Example

Add `elastic` to your `Cargo.toml`:

```toml
[dependencies]
elastic = "*"
elastic_derive = "*"
serde_json = "*"
```

Create a `Client` and start making requests:

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
let client = SyncClientBuilder::new().build().unwrap();

// A search request with a freeform body.
let res = client.search::<Value>()
                .index("_all")
                .body({
                    json!({
                        "query": {
                            "query_string": {
                                "query": "*"
                            }
                        }
                    })
                })
                .send()?;

// Iterate through the hits in the response.
for hit in res.hits() {
    println!("{:?}", hit);
}
```

See the [examples](https://github.com/elastic-rs/elastic/tree/master/elastic/examples) folder for complete samples.

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

The REST API is covered by a simple inline JSON macro like `serde_json`'s [`json!`](https://docs.serde.rs/serde_json/macro.json.html) or [`json_str!`](https://github.com/KodrAus/json_str) so it's always possible to build any query. This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Dev Tools](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) could just be copy+pasted into your Rust source.

The core focus of this project is on strong typing over your document types and query responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules in the `elastic_reqwest` and `elastic_rotor` clients and `elastic_types` as necessary.

## Development

Development is active, but because functionality is split across crates it can be hard to track where the effort is going.
There is a [GitHub Project](https://github.com/orgs/elastic-rs/projects/1) to easily track priorities at the crate-level. Other cross-crate concerns are also tracked as projects.

`elastic` targets the `stable` channel, so it doesn't use any unstable features, but we'd like to track where improvements can be made by unstable features once they stabilise. There is another [GitHub Project](https://github.com/orgs/elastic-rs/projects/8) to record these possible enhancements.

The `elastic` crate brings a few independent crates together into a cohesive API. It aims to provide the glue between them and offer some good defaults. If you have a more specialised use-case, you can pick and choose the crates that will best support it. See the [crates](#crates) section for a full list.

`elastic` sits on a stack with hard dependencies on the following libraries:

- `reqwest`/`hyper` for HTTP transport
- `serde` for serialisation

There hasn't been much effort put into abstracting these dependencies at this stage, and `elastic` can't stabilise until these libraries and few others do.

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
- `elastic` should link back to the underlying crate when it re-exports a feature
- Crates should link to `elastic` when it re-exports that feature

## Crates

`elastic` bundles up a couple of crates into a single client. If you want to pick and choose functionality, you can work with these crates independently.

### [`elastic_reqwest`](https://github.com/elastic-rs/elastic-reqwest)

A synchronous [`reqwest`](https://github.com/seanmonstar/reqwest) implementation of the Elasticsearch REST API.

### [`elastic_requests`](https://github.com/elastic-rs/elastic-requests)

Zero-copy request types for the REST API endpoints. These are automatically generated from the official spec.

### [`elastic_responses`](https://github.com/elastic-rs/elastic-responses)

Idiomatic support for inspecting Elasticsearch responses and iterating over hits.

### [`elastic_types`](https://github.com/elastic-rs/elastic-types)

A library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.
