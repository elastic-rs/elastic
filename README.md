# elasticsearch-rs
Yet another work in progress Elasticsearch client for Rust.

Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Nightly | [![Build Status](https://travis-ci.org/KodrAus/elasticsearch-rs.svg?branch=master)](https://travis-ci.org/KodrAus/elasticsearch-rs)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

Currently only compiling on the `nightly` channel. Support for `stable` will be added.

If the build is red, you can check the Travis build history to find the last `nightly` version that worked. Failures are usually because of changes to dependencies upstream.

## Example

The `elastic_hyper` client is a thin layer over `hyper`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
For serialisation though, the `elastic_macros` crate provides the `json!` macro for serialising abitrary rust-like code to json. 
The deserialisation story is a work in progress.

Add `elastic_hyper` and `elastic_macros` to your `Cargo.toml`:

```
[dependencies]
elastic_hyper = "*"
elastic_macros = "*"
```

Ping the availability of your cluster:

```rust
extern crate elastic_hyper as elastic;

let mut client = hyper::Client::new();
elastic::ping::head(&mut client, elastic::RequestParams::default(), "http://localhost:9200").unwrap();
```

A simple `query_string` query:

```rust
extern crate elastic_hyper as elastic;

// Requests take a standard hyper http client
let mut client = Client::new();

// Optional headers and url query parameters can be added
// Execute a querystring request on a local Elasticsearch instance
let mut res = elastic::search::post(
	&mut client, elastic::RequestParams::default(),
	"http://localhost:9200",
	json_str!({
		query: {
			query_string: {
				query: "*"
			}
		}
	})
).unwrap();
```

See the [samples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples), [elastic_hyper](#elastic_hyper) and [elastic_macros](#elastic_macros) for more details.

If you'd prefer to call Elasticsearch using a Query DSL builder, see [rs-es](https://github.com/benashford/rs-es).

`elastic_types` is a library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them.

Add `elastic_types` to your `Cargo.toml`:

```
[dependencies]
elastic_types = "*"
```

Define a custom Elasticsearch type called `my_type`:

```rust
//Define a struct for your type
//Elasticsearch core types are provided out-of-the-box
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct MyType {
	pub my_date1: DateTime,
	pub my_date2: DateTime<EpochMillis>,
	pub my_string: String,
	pub my_string2: ElasticString<DefaultStringMapping>,
	pub my_num: i32
}

//Define the object mapping for the type
#[derive(Default, Clone)]
struct MyTypeMapping;
impl ElasticObjectMapping for MyTypeMapping {
	//Mapping meta-parameters are exposed as functions
	fn include_in_all() -> Option<bool> {
		Some(false)
	}
}
impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner, [my_date1, my_date2, my_string1, my_string2, my_num]);
```

Compiler-plugins to [automatically derive mapping](https://github.com/KodrAus/elasticsearch-rs/issues/83) will be added in the future.

Get the mapping for your type:

```rust
let mytype = MyType::default();

//Build a serialiser and map our type
let mut writer = Vec::new();
{
	let mut ser = Serializer::new(&mut writer);
	let _ = TypeMapper::map(&mytype, &mut ser).unwrap();
}
let mapping = String::from_utf8(writer).unwrap();
```

See the [elastic_types](#elastic_types) and [elastic_macros](#elastic_macros) for more details.

## Roadmap

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones).

- [ ] Implement core Elasticsearch types
- [ ] Implement Elasticsearch response types
- [ ] Rotor Client
- [x] Codegen API endpoints
- [x] Hyper Client
- [x] Doc APIs
- [x] Query DSL proof-of-concept to test design

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue.

The REST API is provided by an [inline JSON macro](http://kodraus.github.io/rustdoc/elastic_macros/#json-parsing) so it's efficient and always in line with whatever version of Elasticsearch you're targeting.

This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Sense](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) can literally just be copy+pasted into your Rust code.

The core focus of this project is on strong typing over the core types and responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules in the `elastic_hyper` and `elastic_rotor` clients and `elastic_types` as necessary.

## Design

The client is divided into a few crates by utility. These will probably be moved into their own repositories to tidy up build/test, but for now it's conventient to develop them together.

### elastic_codegen

[Docs](http://kodraus.github.io/rustdoc/elastic_codegen/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/codegen)

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used by `elastic_hyper` to build the client, but could also be used to generate other implementations, like `elastic_rotor` for an asynchronous client.

### elastic_hyper

[![Latest Version](https://img.shields.io/crates/v/elastic_hyper.svg)](https://crates.io/crates/elastic_hyper)

[Docs](http://kodraus.github.io/rustdoc/elastic_hyper/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/hyper) |
[Samples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples)

Provides a synchronous [hyper](https://github.com/hyperium/hyper) implementation of the Elasticsearch REST API. The `hyper` client is simple to use; there's basically no setup needed besides creating a `hyper::Client` object to use for requests. The `hyper` client is general-purpose, and suitable for any scenario where on-demand requests are sufficient.

### elastic_rotor

[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/rotor)

_In Progress_

Will provide an asynchronous [rotor-http](https://github.com/tailhook/rotor-http) implementation of the Elasticsearch REST API. This client is an active work in progress, as is `rotor` itself so things will change a lot. The `rotor` client is more complex than the `hyper` one, providing connection pooling and long-lived requests. It'll be best suited to streaming scenarios, or where Elasticsearch connections will be used heavily.

The crate will allow you to use connections in two ways; add connections as state machines to your own `mio` loop, or use an out-of-the-box connection pool.

### elastic_macros

[![Latest Version](https://img.shields.io/crates/v/elastic_macros.svg)](https://crates.io/crates/elastic_macros)

[Docs](http://kodraus.github.io/rustdoc/elastic_macros/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/macros)

Provides compiler plugins and macros for working with other `elastic` crates. Macros relevant to specific crates are feature-gated, but you don't normally need to worry about this.

### elastic_types

[Docs](http://kodraus.github.io/rustdoc/elastic_types/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/types)

Provides rust implementations of the main [Elasticsearch types](https://www.elastic.co/guide/en/elasticsearch/reference/1.4/mapping-core-types.html) (like `date`) and responses/errors. This crate is not required for working with `elastic_hyper` or `elastic_rotor`, but does have a lot of utility, especially for designing your document types.

The `elastic_types` crate tries not to reinvent the wheel wherever possible and relies on some common dependencies for types, such as [chrono](https://github.com/lifthrasiir/rust-chrono) for dates and [rust-geo](https://github.com/georust/rust-geo) for geometry.

