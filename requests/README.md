# [`elastic_requests`](https://docs.rs/elastic_requests/*/elastic_requests/) [![Latest Version](https://img.shields.io/crates/v/elastic_requests.svg)](https://crates.io/crates/elastic_requests)

`elastic_requests` is a strongly-typed, code-generated implementation of the Elasticsearch REST API for Rust.

This library doesn't provide HTTP transport directly, it's used by [`elastic_hyper`](https://github.com/elastic-rs/elastic-hyper) for that.

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-requests.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-requests)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic-requests/elastic_requests/)
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_requests/*/elastic_requests/)

The goal is to be zero-allocation where possible, so request types are just wrappers around potentially owned data.
A structure is generated for each REST endpoint, that generate url paths from the given parameters.

## Example

Add `elastic_requests` to your `Cargo.toml`:

```
[dependencies]
elastic_requests = "*"
```

And reference it in your crate root:

```rust
extern crate elastic_requests as requests;
```

There's a request type for each REST API endpoint with constructor functions for each valid set of parameters:

```rust
let req = requests::SearchRequest::for_index_ty(
	"myindex", "mytype", 
	json_str!({
		query: { 
			match_all: {}
		}
	})
);

assert_eq!("/myindex/mytype/_search", *req.url);
```

Parameters can be supplied as owned or borrowed strings and the body as an owned or borrowed byte array:

```rust
let index_suffix = get_a_suffix();

let req = requests::SimpleSearchRequest::for_index_ty(
	format!("index-{}", index_suffix), "mytype"
);
```

There's also a more general `HttpRequest` structure that represents a typical request.
All request types implement `Into<HttpRequest>` for owned or borrowed references, so you can work with an arbitrary request through this type bound:

```rust
fn do_something_with_a_request<'a, I: Into<HttpRequest<'a>>>(req: I) {}

// Use a borrowed request
do_something_with_a_request(&req);

// Take ownership of the request
do_something_with_a_request(req);
```

`HttpRequest<'static>` implements `Send` so it can be shared across threads.

## Codegen

The types in this library are generated from the [Elasticsearch REST API spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec).
This can be run from the `codegen` directory:

```
$ cd codegen
$ cargo run > ../src/genned.rs
```