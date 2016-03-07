# elasticsearch-rs
Yet another work in progress Elasticsearch client for Rust.

Platform  | Status
------------- | -------------
Linux / OSX  | [![Build Status](https://travis-ci.org/KodrAus/elasticsearch-rs.svg?branch=master)](https://travis-ci.org/KodrAus/elasticsearch-rs)
Windows  | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

Currently only compiling on the `nightly` channel. Support for `stable` will be added.

If the build is red, you can check the Travis build history to find the last `nightly` version that worked. Failures are usually because of changes to dependencies upstream.

## Example

The `elastic_hyper` client is a thin layer over `hyper`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
For serialisation though, the `elastic_macros` crate provides the `json!` macro for serialising abitrary rust-like code to json. 
The deserialisation story is a work in progress.

Ping the availability of your cluster:

```rust
extern crate elastic_hyper as elastic;

let mut client = hyper::Client::new();
elastic::ping::head(&mut client, "http://localhost:9200").unwrap();
```

A simple `query_string` query:

```rust
#![feature(plugin)]
#![plugin(elastic_macros)]
extern crate elastic_hyper as elastic;

let mut client = hyper::Client::new();
let res = elastic::search::post_index_type(&mut client, "http://localhost:9200", "bench_index", "docs", 
	json!({
		query: {
			query_string: {
				default_field: "title",
				query: "doc"
			}
		}
	})
).unwrap()
```

See the [samples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples), [elastic_hyper](#elastic_hyper) and [elastic_macros](#elastic_macros) for more details.

If you'd prefer to call Elasticsearch using a Query DSL builder, see [rs-es](https://github.com/benashford/rs-es).

## Roadmap

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones).

- [ ] Implement core Elasticsearch types
- [ ] Implement Elasticsearch response types
- [x] Codegen API endpoints
- [x] Client
- [x] Doc APIs
- [x] Query DSL proof-of-concept to test design

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io.

The REST API is provided by an [inline JSON macro](http://kodraus.github.io/rustdoc/elastic_macros/#json-parsing) so it's efficient and always in line with whatever version of Elasticsearch you're targeting.

This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Sense](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) can literally just be copy+pasted into your Rust code.

The core focus of this project is on strong typing over the core types and responses in Elasticsearch, rather than trying to map the entire Query DSL.

## Design

The client is divided into a few crates by utility. These will probably be moved into their own repositories to tidy up build/test, but for now it's conventient to develop them together.

### elastic_codegen

[Docs](http://kodraus.github.io/rustdoc/elastic_codegen/)
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/codegen)

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used by `elastic_hyper` to build the client, but could also be used to generate other implementations, like `elastic_rotor` for an asynchronous client.

### elastic_hyper

[Docs](http://kodraus.github.io/rustdoc/elastic_hyper/)
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/hyper)

Provides a [hyper](https://github.com/hyperium/hyper) implementation of the Elasticsearch REST API. This is the current client that works purely through JSON. This crate is responsible for the `gen` in `elastic_codegen` and builds its own source and tests.

### elastic_types

[Docs](http://kodraus.github.io/rustdoc/elastic_types/)
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/types)

Provides rust implementations of the main [Elasticsearch types](https://www.elastic.co/guide/en/elasticsearch/reference/1.4/mapping-core-types.html) (like `date`) and responses/errors. This crate is not required for working with `elastic_hyper`, but does have a lot of utility, especially for designing your document types.

The `elastic_types` crate tries not to reinvent the wheel wherever possible and relies on some common dependencies for types, such as [chrono](https://github.com/lifthrasiir/rust-chrono) for dates and [rust-geo](https://github.com/georust/rust-geo) for geometry.

### elastic_macros

[![Latest Version](https://img.shields.io/crates/v/elastic_macros.svg)](https://crates.io/crates/elastic_macros)

[Docs](http://kodraus.github.io/rustdoc/elastic_macros/)

Provides compiler plugins and macros for the `elastic_types` crate, such as parsing a date format to an array of [Items](https://github.com/lifthrasiir/rust-chrono/blob/master/src/format/mod.rs#L161) at compile-time for efficient runtime date parsing.
