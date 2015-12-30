# elasticsearch-rs
Elasticsearch client for Rust

## Status

Platform  | Status
------------- | -------------
Linux / OSX  | [![Build Status](https://travis-ci.org/KodrAus/elasticsearch-rs.svg?branch=master)](https://travis-ci.org/KodrAus/elasticsearch-rs)
Windows  | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

Currently only compiling on the `nightly` channel. Support for `stable` will be added.

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over asynchronous io.

I'd like to follow a similar pattern to the Query DSL as the C# client does, where type info is always available when constructing queries. With Rust, it should be possible to produce a synax that's closer to the actual JSON spec, possibly using macros.

## Design

The client is divided into a few crates by utility. These will probably be moved into their own repositories to tidy up build/test, but for now it's conventient to develop them together.

### elastic_codegen

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used be `elastic_hyper` to build the low-level client, but could also be used to generate other implementations, like `elastic_rotor`.

### elastic_hyper

Provides a [hyper]("https://github.com/hyperium/hyper") implementation of the Elasticsearch REST API. This is the 'low-level' client that works purely through JSON. This crate is responsible for the `gen` in `elastic_codegen` and builds its own source and tests.

### elastic_types

Provides rust implementations of the main Elasticsearch types (like `date`) and responses. Required by the high-level client and useful for the low-level client, especially if you're mostly just using the [Document API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs.html?q=document).

### elastic_client

Provides the high-level, strongly-typed client on top of the low-level client and `elastic_types`. The direction of this crate isn't really decided yet, but I'm liking the idea of using closures to thread type info through a query builder, like the C# client.

## Roadmap

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones)

- Codegen API endpoints
- Implement core Elasticsearch types
- io layer
- Doc APIs
- Query DSL proof-of-concept to test design
- Everything else
