# `elastic_requests`

`elastic_requests` is a strongly-typed, code-generated implementation of the Elasticsearch REST API for Rust.
The goal is to be zero-allocation where possible, so request types are just wrappers around potentially owned data.
A structure is generated for each REST endpoint, that generate url paths from the given parameters.

There's also a more general `HttpRequest` structure that represents a typical request.
All request types implement `Into<HttpRequest>`, so you can work with an arbitrary request through this type bound.

```
$ cd codegen
$ cargo run > out.rs
```