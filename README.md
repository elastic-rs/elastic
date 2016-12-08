# `elastic_requests`

`elastic_requests` is a strongly-typed, code-generated implementation of the Elasticsearch REST API for Rust.

This library doesn't provide HTTP transport directly, it's used by [`elastic_hyper`](https://github.com/elastic-rs/elastic-hyper) for that.

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
extern crate elastic_requests as elastic;
```

There's a request type for each REST API endpoint with constructor functions for each valid set of parameters:

```rust
let req = elastic::SearchRequestParams::index_ty(
	"myindex", "mytype", 
	json_str!({
		query: { 
			match_all: {}
		}
	})
);

assert_eq!("/test_index/test_ty/_search", req.url());
```

Parameters can be supplied as owned or borrowed strings and the body as an owned or borrowed byte array:

```rust
let index_suffix = get_a_suffix();

let req = elastic::SearchRequestParams::index_ty(
	format!("index-{}", index_suffix), "mytype", 
	"{ 'query': { 'match_all': { } } }"
);
```

There's also a more general `HttpRequest` structure that represents a typical request.
All request types implement `Into<HttpRequest>` for _borrowed references_, so you can work with an arbitrary request through this type bound:

```rust
fn do_something_with_a_request<'a, I: Into<HttpRequest<'a>>>(req: I) {}

do_something_with_a_request(&req);
```

## Codegen

The types in this library are generated from the [Elasticsearch REST API spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec).
This can be run from the `codegen` directory:

```
$ cd codegen
$ cargo run > ../src/genned.rs
```