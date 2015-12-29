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


## Roadmap

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones)

- Codegen API endpoints
- Implement core Elasticsearch types
- io layer
- Doc APIs
- Query DSL proof-of-concept to test design
- Everything else
