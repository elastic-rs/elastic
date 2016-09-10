# [`elastic_types`](http://kodraus.github.io/rustdoc/0-5-x/elastic_types/) [![Latest Version](https://img.shields.io/crates/v/elastic_types.svg)](https://crates.io/crates/elastic_types)

`elastic_types` is a library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.
It provides rust implementations of the core [Elasticsearch datatypes](https://www.elastic.co/guide/en/elasticsearch/reference/1.4/mapping-core-types.html) (like `date`, `geo_point`) and responses/errors.

It's especially helpful for the `date` and `geo_point` types, where serialisation for the various formats is provided for you.

This library makes heavy use of [`serde`](https://serde.rs/).
We also try not to reinvent the wheel wherever possible and rely on some common dependencies for types, such as [`chrono`](https://github.com/lifthrasiir/rust-chrono) for dates and [`rust-geo`](https://github.com/georust/rust-geo) for geometry.

## Build Status
Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/elastic-rs/elastic-types.svg?branch=master)](https://travis-ci.org/elastic-rs/elastic-types)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

## Documentation

Version  | Docs
------------- | -------------
`master`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic-types/elastic_types/)
`0.5`  | [![Documentation](https://img.shields.io/badge/docs-rustdoc-orange.svg)](https://elastic-rs.github.io/elastic_types/0.5/elastic_types/)

## Example

On `nightly`, add `elastic_types` to your `Cargo.toml`:

```
[dependencies]
elastic_types = { version = "*", features = "nightly" }
elastic_types_macros = "*"
```

Define a custom Elasticsearch type called `mytype`:

```rust
#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyType {
	pub my_date: Date<EpochMillis>,
	pub my_string: String,
	pub my_num: i32
}
```

This will create a struct for you called `ElasticTypeMapping`.
You can then get the mapping for your type:

```rust
let mapping = TypeMapper::to_string(MyTypeMapping).unwrap();
```

This will produce:

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

## Macros

### `elastic_types_macros`

Provides custom derive plugins for Elasticsearch datatypes and mappings in `elastic_types`.

### `elastic_date_macros`

Provides date-specific plugins for the date datatype in `elastic_types`.