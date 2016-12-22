# [`elastic_types`](https://docs.rs/elastic_types/*/elastic_types/) [![Latest Version](https://img.shields.io/crates/v/elastic_types.svg)](https://crates.io/crates/elastic_types)

`elastic_types` is a library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.
It provides rust implementations of the core [Elasticsearch datatypes](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping-types.html#_core_datatypes) (like `date`, `geo_point`).

It's especially helpful for the `date` and `geo_point` types, where serialisation for the various formats is provided for you.

This library makes heavy use of [`serde`](https://serde.rs/).
We also try not to reinvent the wheel wherever possible and rely on some common dependencies for types, such as [`chrono`](https://github.com/lifthrasiir/rust-chrono) for dates and [`rust-geo`](https://github.com/georust/rust-geo) for geometry.

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-types.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-types)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/2x2cmfi6hku72nmk?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic-types)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic-types/elastic_types/)
`current`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_types/*/elastic_types/)

## Example

On `nightly`, add `elastic_types` to your `Cargo.toml`:

```
[dependencies]
elastic_types = { version = "*", features = "nightly" }
elastic_types_derive = "*"
```

And reference it in your crate root:

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate elastic_types;
```

## Defining indexable types

Define a custom Elasticsearch type called `mytype`:

```rust
#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyType {
	pub my_date: Date<EpochMillis>,
	pub my_string: String,
	pub my_num: i32
}
```

You can then get the mapping for your type as `json`:

```rust
let mapping = TypeMapper::to_string(MyType::mapping()).unwrap();
```

```json
{
  "properties": {
    "my_date": {
      "type": "date",
      "format": "epoch_millis"
    },
    "my_string": {
      "type": "text",
      "fields": {
        "keyword": {
          "type": "keyword",
          "ignore_above": 256
        }
      }
    },
    "my_num": {
      "type": "integer"
    }
  }
}
```

The `stable` channel is also supported, see the [docs](#documentation) for details.

## Deserialising indexed types

Types that derive `ElasticType` are themselves serialisable, which can be very helpful when using 
types with special formats, like `date`.
Take the following document:

```json
{
  "id": 15,
  "timestamp": 1435935302478,
  "title": "my timestamped object"
}
```

Using the `Date<EpochMillis>` type for the `timestamp`, we can correctly deserialise the document as a strongly typed
object:

```rust
#[derive(Deserialize)]
struct MyType {
  id: i32,
  timestamp: Date<EpochMillis>,
  title: String
}

let de: MyType = serde_json::from_str(json).unwrap();

assert_eq!(2015, de.timestamp.year());
```

## Macros

### `elastic_types_derive`

Provides custom derive plugins for Elasticsearch datatypes and mappings in `elastic_types`.

### `elastic_date_macros`

Provides date-specific plugins for the date datatype in `elastic_types`.
