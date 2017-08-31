# [`elastic_types`](https://docs.rs/elastic_types/*/elastic_types/) [![Latest Version](https://img.shields.io/crates/v/elastic_types.svg)](https://crates.io/crates/elastic_types)

`elastic_types` is a library for statically-defined document mappings for Rust. It provides tools to define, encode and decode document types efficiently, however they're stored in Elasticsearch.

Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system. It provides rust implementations of the core [Elasticsearch datatypes](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping-types.html#_core_datatypes) (like `date`, `geo_point`).

It's especially helpful for the `date` and `geo_point` types, where serialisation for the various formats is provided for you.

This library makes heavy use of [`serde`](https://serde.rs/) for serialisation. We also try not to reinvent the wheel wherever possible and rely on some common dependencies for types, such as [`chrono`](https://github.com/lifthrasiir/rust-chrono) for dates and [`rust-geo`](https://github.com/georust/rust-geo) for geometry.

This library is the document serialisation provider for the higher-level [`elastic`](https://github.com/elastic-rs/elastic) client.

## Build Status
Platform  | Channel | Status
------------- | ---------------- | -------------
Linux / OSX   | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-types.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-types)
Windows       | Nightly          | [![Build status](https://ci.appveyor.com/api/projects/status/0fk6ogm3inh3wip2?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic-types-42k4g)

## Documentation

Version   | Docs
--------- | -------------
`current` | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://docs.rs/elastic_types/*/elastic_types/)

## Example

Add `elastic_types` to your `Cargo.toml`:

```
[dependencies]
elastic_types = "*"
elastic_types_derive = "*"
```

And reference it in your crate root:

```rust
#[macro_use]
extern crate elastic_types_derive;
extern crate elastic_types;
```

## Defining indexable types

Define a custom Elasticsearch type called `mytype`:

```rust
#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyType {
	pub timestamp: Timestamp,
	pub my_string: String,
	pub my_num: i32
}

type Timestamp = Date<DefaultDateMapping<EpochMillis>>;
```

You can then get the mapping for your type as `json`:

```rust
let mapping = serde_json::to_string(&IndexDocumentMapping::from(MyType::mapping())).unwrap();
```

Which looks like:

```json
{
  "properties": {
    "timestamp": {
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

## Deserialising indexed types

Types that derive `ElasticType` are themselves serialisable, which can be very helpful when using types with special formats, like `date`. Take the following document:

```json
{
  "id": 15,
  "timestamp": 1435935302478,
  "title": "my timestamped object"
}
```

Using the `Date` type for the `timestamp`, we can correctly deserialise the document as a strongly typed object:

```rust
#[derive(Deserialize)]
struct MyType {
  id: i32,
  timestamp: Timestamp,
  title: String
}

type Timestamp = Date<DefaultDateMapping<EpochMillis>>;

let de: MyType = serde_json::from_str(json).unwrap();

assert_eq!(2015, de.timestamp.year());
```

## Macros

### `elastic_types_derive`

Provides custom derive plugins for Elasticsearch datatypes and mappings in `elastic_types` and date-specific plugins for the date datatype.
