# [`elastic_hyper`](https://docs.rs/elastic_hyper/*/elastic_hyper/) [![Latest Version](https://img.shields.io/crates/v/elastic_hyper.svg)](https://crates.io/crates/elastic_hyper)

Provides a synchronous [`hyper`](https://github.com/hyperium/hyper) implementation of the Elasticsearch REST API. The `hyper` client is simple to use; there's basically no setup needed besides creating a `hyper::Client` object to use for requests. The `hyper` client is general-purpose, and suitable for any scenario where on-demand requests are sufficient.

If you'd prefer to call Elasticsearch using a strongly-typed Query DSL builder, see [`rs-es`](https://github.com/benashford/rs-es).

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-hyper.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-hyper)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/yvsqsyt4ioxa11g8?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic-hyper)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic-hyper/elastic_hyper/)
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_hyper/*/elastic_hyper/)

## Example

The `elastic_hyper` client is a thin layer over `hyper`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
- For query serialisation, the [`json_str`](https://github.com/KodrAus/json_str) crate provides the `json_str!` macro for creating ad-hoc API queries.
- For type serialisation / deserialisation, see [`elastic_types`](https://github.com/elastic-rs/elastic-types).

Currently targeting the `master` Elasticsearch branch, aiming for `5.x`.
This will be stabilised through features in the future.

Add `elastic_hyper` and `json_str` to your `Cargo.toml`:

```
[dependencies]
elastic_hyper = "*"
json_str = "*"
```

Ping the availability of your cluster:

```rust
#[macro_use]
extern crate json_str;
extern crate elastic_hyper as elastic;

let (mut client, params) = elastic::default();

elastic::ping::head(&mut client, &params).unwrap();
```

A simple `query_string` query:

```rust
#[macro_use]
extern crate json_str;
extern crate elastic_hyper as elastic;

let (mut client, params) = elastic::default();

let response = elastic::search::post(
  &mut client, &params,
  &json_str!({
    query: {
      query_string: {
        query: "*"
      }
    }
  })
).unwrap();
```