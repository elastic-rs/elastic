# [`elastic`](https://docs.rs/elastic/*/elastic/) [![Latest Version](https://img.shields.io/crates/v/elastic.svg)](https://crates.io/crates/elastic)

`elastic_*` is an ecosystem of community crates for interacting with [Elasticsearch](https://github.com/elastic/elasticsearch) from [Rust](https://www.rust-lang.org).
The API is targetting Elastic `5.x`.

Crate functionality covers:

- [http client](#elastic_reqwest)
- [type mapping](#elastic_types)

Quick reference:

- [crates](#crates)
- [samples](https://github.com/elastic-rs/elastic/tree/master/examples)

> NOTE: This client is under active churn right now while the pieces are tied together.
The documentation will improve shortly.

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/t71058ht2qp732eh?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic)

[View the status of all `elastic` crates](https://travis-ci.org/elastic-rs)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic/elastic/)
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic/*/elastic/)

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue.

The REST API is provided by an [inline JSON macro](https://github.com/KodrAus/json_str) so it's efficient and always in line with whatever version of Elasticsearch you're targeting. This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Sense](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) can just be copy+pasted into your Rust code.

The core focus of this project is on strong typing over your document types and query responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules in the `elastic_reqwest` and `elastic_rotor` clients and `elastic_types` as necessary.

## Alternatives

If you'd like to use a strongly-typed Query DSL builder see [`rs-es`](https://github.com/benashford/rs-es). This client does the hard work of providing an idiomatic Rust API for interacting with Elasticsearch. It has the advantage of letting you know your queries will parse at compile-time instead of runtime.

## Example

See the [examples](https://github.com/elastic-rs/elastic/tree/master/examples) folder.

Add `elastic` and `json_str` to your `Cargo.toml`:

```toml
[dependencies]
elastic = "*"

# Optional
serde_json = "*"
json_str = "*"
```

And reference in your crate root:

```rust
#[macro_use]
extern crate json_str;
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;
```

Get a client instance:

```rust
let client = Client::new(RequestParams::default()).unwrap();
```

Create a search request:

```rust
let body = json_str!({
    query: {
        query_string: {
            query: "*"
        }
    }
});

let req = SearchRequest::for_index("_all", body);
```

Send the request and iterate through the returned hits:

```rust
let res: SearchResponse<Value> = client.request(req)
                                       .send()
                                       .and_then(|res| res.response())
                                       .unwrap();

for hit in res.hits() {
    println!("{:?}", hit);
}
```

## Development

Development is active, but because functionality is split across crates it can be hard to track where the effort is going.
There is a [GitHub Project](https://github.com/orgs/elastic-rs/projects/1) to easily track priorities at the crate-level.

The `elastic` crate brings these independent units together into a cohesive API. It aims to provide the glue between them and offer some typical defaults. If you have a more specialised use-case, you can pick and choose the crates that will best support it.

`elastic` sits on a stack with hard dependencies on the following libraries:

- `reqwest`/`hyper` for HTTP
- `serde` for serialisation

There hasn't been much effort put into abstracting these dependencies at this stage.

### Methodology

The following is a simple set of guidelines that the codebase should follow.

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
- Features should fail to compile if used incorrectly rather than having lots of usecases that need testing

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

### [`elastic_rotor`](https://github.com/elastic-rs/elastic-rotor)

An experimental REST API client that handles a single specific usecase: high throughput.
