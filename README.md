# Elastic

`elastic_*` is an ecosystem of community crates for interacting with [Elasticsearch](https://github.com/elastic/elasticsearch) from [Rust](https://www.rust-lang.org).
The API is targetting Elastic `5.x`.

Crate functionality covers:

- [transport](#elastic_hyper)
- [type mapping](#elastic_types)
- [codegen](#elastic_codegen)

Quick reference:

- [crates](#crates)
- [samples](https://github.com/elastic-rs/elastic-hyper/tree/master/samples)

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue.

The REST API is provided by an [inline JSON macro](https://github.com/KodrAus/json_str) so it's efficient and always in line with whatever version of Elasticsearch you're targeting.

This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Sense](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) can literally just be copy+pasted into your Rust code.

The core focus of this project is on strong typing over the core types and responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules in the `elastic_hyper` and `elastic_rotor` clients and `elastic_types` as necessary.

## Development

Development is active, but because functionality is split across crates it can be hard to track where the effort is going.
There is a [GitHub Project](https://github.com/orgs/elastic-rs/projects/1) to easily track priorities at the crate-level.

## Crates

### [`elastic_hyper`](https://github.com/elastic-rs/elastic-hyper)

`elastic_hyper` provides a synchronous [`hyper`](https://github.com/hyperium/hyper) implementation of the Elasticsearch REST API.

This crate lives in the [`elastic_hyper` repo](https://github.com/elastic-rs/elastic-hyper).

### [`elastic_types`](https://github.com/elastic-rs/elastic-types)

`elastic_types` is a library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.

This crate lives in the [`elastic_types` repo](https://github.com/elastic-rs/elastic-types).

### [`elastic_rotor`](https://github.com/elastic-rs/elastic-rotor)

`elastic_rotor` is an experimental REST API client that handles a single specific usecase: high throughput.

### [`elastic_codegen`](https://github.com/elastic-rs/elastic-codegen)

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used by `elastic_hyper` to build the client, but could also be used to generate other implementations.

## Alternatives

If you'd like to use a strongly-typed Query DSL builder see [`rs-es`](https://github.com/benashford/rs-es). This client does the hard work of providing an idiomatic Rust API for interacting with Elasticsearch.
