/*!
Elasticsearch Core Types

An implementation of Elasticsearch data types and document mapping.

This library provides tools for defining and using Elasticsearch type mappings,
where correctness is enforced by Rust's type system.
The mapping information is then used when serialising and deserialising your types.
Annotating type fields with mapping metadata has no impact at runtime.

This library makes extensive use of [`serde`](https://serde.rs/).

# Supported Versions

 `elastic_types` | Elasticsearch
 --------------- | -------------
 `0.x`           | `5.x`

# Usage

This crate is on [crates.io](https://crates.io/crates/elastic_types).

To get started, add `elastic_types` and `elastic_types_derive` to your `Cargo.toml`:

```ignore
[dependencies]
elastic_types = version = "*"
elastic_types_derive = "*"
```

And reference it in your crate root:

```ignore
#[macro_use]
extern crate elastic_types_derive;
extern crate elastic_types;
```

## Map Your Types

Derive `ElasticType` on your Elasticsearch-mappable types:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
#[derive(Serialize, ElasticType)]
pub struct MyType {
    pub my_date: Date<DefaultDateFormat>,
    pub my_num: i32
}
# fn main() {
# }
```

You can then serialise your mapping as json using an [`IndexDocumentMapping`](document/struct.IndexDocumentMapping.html)
wrapper:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde_json;
# extern crate serde;
# use elastic_types::prelude::*;
# #[derive(Serialize, Deserialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateFormat>,
#     pub my_string: String,
#     pub my_num: i32
# }
# fn main() {
let mapping = serde_json::to_string(&IndexDocumentMapping::from(MyType::mapping())).unwrap();
# }
```

This will produce the following result:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde_json;
# extern crate serde;
# use elastic_types::prelude::*;
# #[derive(Serialize, Deserialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateFormat>,
#     pub my_num: i32
# }
# fn main() {
# let mapping = serde_json::to_string(&IndexDocumentMapping::from(MyType::mapping())).unwrap();
# let json = json_str!(
{
    "properties": {
        "my_date": {
            "type": "date",
            "format": "basic_date_time"
        },
        "my_num": {
            "type": "integer"
        }
    }
}
# );
# assert_eq!(json, mapping);
# }
```

### Mapping structs as fields

Of course, structs that derive `ElasticType` can also be used as fields in other Elasticsearch types:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
# #[derive(Serialize, Deserialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateFormat>,
#     pub my_num: i32
# }
#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyOtherType {
    pub my_type: MyType
}
# fn main() {
# }
```

Our mapping for `MyOtherType` then looks like:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde_json;
# extern crate serde;
# use elastic_types::prelude::*;
# #[derive(Serialize, Deserialize, ElasticType)]
# pub struct MyType {
#     pub my_date: Date<DefaultDateFormat>,
#     pub my_num: i32
# }
# #[derive(Serialize, Deserialize, ElasticType)]
# pub struct MyOtherType {
#     pub my_type: MyType
# }
# fn main() {
# let mapping = serde_json::to_string(&IndexDocumentMapping::from(MyOtherType::mapping())).unwrap();
# let json = json_str!(
{
    "properties": {
        "my_type": {
            "type": "nested",
            "properties": {
                "my_date": {
                    "type": "date",
                    "format": "basic_date_time"
                },
                "my_num": {
                    "type": "integer"
                }
            }
        }
    }
}
# );
# assert_eq!(json, mapping);
# }
```

### Mapping `Option` and `Vec`

Elasticsearch doesn't differentiate between nullable types or collections, so it's also possible
to derive mapping from `Option` or `Vec` types:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyType {
    pub my_date: Option<Date<DefaultDateFormat>>,
    pub my_num: Vec<i32>
}
# fn main() {
# }
```

This produces the same mapping as before.
See the [`document`](document/index.html) mod for more details.

### Overloading default mapping

You can override the default mapping for Elasticsearch's core datatypes by implementing
the appropriate trait. In the below example, we create a custom `boolean` mapping:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
#[derive(Default)]
struct MyMapping;
impl BooleanMapping for MyMapping {
    fn boost() -> Option<f32> { Some(1.04) }
}
# fn main() {
# }
```

For more details about the supported core datatypes and how to use them, see [here](#types).

## Serialise Your Types

Types that derive `ElasticType` are themselves serialisable, which can be very helpful when using
types like `date` with special formats.
Take the following document:

```ignore
{
    "id": 15,
    "timestamp": 1435935302478,
    "title": "my timestamped object"
}
```

Using the `Date<EpochMillis>` type for the `timestamp`, we can correctly deserialise the document as a strongly typed
object:

```
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# extern crate serde_json;
# use elastic_types::prelude::*;
#[derive(Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    timestamp: Date<EpochMillis>,
    title: String
}

# fn main() {
# let json = "{\"id\": 15,\"timestamp\": 1435935302478,\"title\": \"my timestamped object\"}";
let de: MyType = serde_json::from_str(json).unwrap();

assert_eq!(2015, de.timestamp.year());
# }
```

## A Complete Example

Before digging in to the API, consider the following complete example for defining and mapping a
type called `Article` on `nightly`.
As `json`, the `Article` type should look something like this:

```ignore
{
    "id": 1,
    "title": "An article",
    "content": "Some prose for this article.",
    "timestamp": 1435935302478,
    "geoip": {
        "ip": "10.0.0.1",
        "loc": [ -71.34, 41.12 ]
    }
}
```

Our `Cargo.toml` specifies the dependencies as above:

```ignore
[dependencies]
elastic_types = "*"
elastic_types_derive = "*"
```

And our `main.rs` contains the following:

```
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate elastic_types;

use elastic_types::prelude::*;

// Our main datatype, `article`

#[derive(Serialize, Deserialize, ElasticType)]
struct Article {
    pub id: i32,
    pub title: String,
    pub content: Text<ContentMapping>,
    pub timestamp: Option<Date<EpochMillis, TimestampMapping>>,
    pub geoip: GeoIp
}


// A second datatype, `geoip`

#[derive(Serialize, Deserialize, ElasticType)]
struct GeoIp {
    pub ip: ::std::net::Ipv4Addr,
    pub loc: GeoPoint<DefaultGeoPointFormat>
}


// Mappings for our datatype fields

#[derive(Default)]
struct ContentMapping;
impl TextMapping for ContentMapping {
    fn analyzer() -> Option<&'static str> {
        Some("content_text")
    }
}

#[derive(Default)]
struct TimestampMapping;
impl DateMapping for TimestampMapping {
    type Format = EpochMillis;

    fn null_value() -> Option<Date<EpochMillis, Self>> {
        Some(Date::now())
    }
}

fn main() {
    println!("\"{}\":{{ {} }}",
        Article::name(),
        serde_json::to_string(&IndexDocumentMapping::from(Article::mapping())).unwrap()
    );
}
```

The above example defines a `struct` called `Article` with a few fields:

- A default `integer` field called `id`
- A default `string` field called `title`
- A `text` field with a custom analyser called `content`
- A `date` field with the `epoch_millis` format that defaults to the time the index was created called `timestamp`
- An object field called `GeoIp` with default `ip` and `geo_point` fields.

Go ahead and run that sample and see what it outputs.
In case you're interested, it'll look something like this (minus the whitespace):

```ignore
"article": {
    "properties": {
        "id":{
            "type": "integer"
        },
        "title": {
            "type":"text",
            "fields": {
                "keyword": {
                    "type": "keyword",
                    "ignore_above": 256
                }
            }
        },
        "content": {
            "type": "text",
            "analyzer": "content_text"
        },
        "timestamp": {
            "type": "date",
            "format": "epoch_millis",
            "null_value": "1435935302478"
        },
        "geoip": {
            "type": "nested",
            "properties": {
                "ip": {
                    "type": "ip"
                },
                "loc": {
                    "type": "geo_point"
                }
            }
        }
    }
}
```

The mapping is constructed by inspecting the type parameters of the fields on `Article` and `GeoIp` at compile-time.
This mapping is then serialised by [`serde`](https://serde.rs) at runtime.

# Types

Types in Elasticsearch are a combination of _source_ and _mapping_.
The source is the data (like `42` or `"my string"`) and the mapping is metadata about how to
interpret and use the data (like the format of a date string).

The approach `elastic_types` takes to types is to bundle the mapping up as a _Zero Sized Type_,
which is then bound to a field type as a generic parameter. For example:

```ignore
Boolean<MyMapping>
```

The source type is `boolean` and the mapping is `MyMapping`.

All Elasticsearch types implement the base `FieldType<M: FieldMapping<F>, F>` trait
where `M` is the mapping and `F` is a type-specific format.
All document types implement `DocumentType` with an associated `Mapping: DocumentMapping` type.

The following table illustrates the types provided by `elastic_types`:

 Elasticsearch Type  | Rust Type (Default Mapping) | Crate     | Rust Type (Custom Mapping)                                                       | Format Type
 ------------------- | --------------------------- | --------- | -------------------------------------------------------------------------------- | -----------------
 `object`            | -                           | -         | type implementing [`DocumentType<M>`](document/index.html)                       | -
 `integer`           | `i32`                       | `std`     | [`Integer<M>`](number/index.html)                                                | -
 `long`              | `i64`                       | `std`     | [`Long<M>`](number/index.html)                                                   | -
 `short`             | `i16`                       | `std`     | [`Short<M>`](number/index.html)                                                  | -
 `byte`              | `i8`                        | `std`     | [`Byte<M>`](number/index.html)                                                   | -
 `float`             | `f32`                       | `std`     | [`Float<M>`](number/index.html)                                                  | -
 `double`            | `f64`                       | `std`     | [`Double<M>`](number/index.html)                                                 | -
 `keyword`           | -                           | -         | [`Keyword<M>`](string/index.html)                                                | -
 `text`              | `String`                    | `std`     | [`Text<M>`](string/index.html)                                                   | -
 `boolean`           | `bool`                      | `std`     | [`Boolean<M>`](boolean/index.html)                                               | -
 `ip`                | `Ipv4Addr`                  | `std`     | [`Ip<M>`](ip/index.html)                                                         | -
 `date`              | `DateTime<Utc>`             | `chrono`  | [`Date<M>`](date/index.html)                                                  | `DateFormat`
 `geo_point`         | `Point`                     | `geo`     | [`GeoPoint<F, M>`](geo/point/index.html)                                         | `GeoPointFormat`
 `geo_shape`         | -                           | `geojson` | [`GeoShape<M>`](geo/shape/index.html)                                            | -

## Mapping

Having the mapping available at compile-time captures the fact that a mapping is static and tied
to the data type.

Where there's a `std` type that's equivalent to an Elasticsearch type (like `i32` for `integer`),
a default mapping is implemented for that type.
That means you can use primitives in your structs and have them mapped to the correct type in Elasticsearch.
If you want to provide your own mapping for a `std` type, there's also a struct provided by `elastic_types`
that wraps the `std` type but also takes an explicit mapping (like `Integer` which implements `Deref<Target = i32>`).

Where there isn't a `std` type available (like `date`), an external crate is used and an implementation of
that type is provided (like `Date`, which implements `Deref<Target = chrono::DateTime<Utc>>`).

## Formats

For some types (like `Date`), it's helpful to have an extra generic parameter that describes the
way the data can be interpreted. For most types the format isn't exposed, because there aren't any alternative formats available.
This is a particularly helpful feature for serialisation.

# Links

- [Elasticsearch Mapping Concepts](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
- [Elasticsearch Type Concepts](https://www.elastic.co/guide/en/elasticsearch/reference/current/_basic_concepts.html#_type)
- [Github](https://github.com/elastic-rs/elastic-types)
*/

#![deny(warnings)]
#![deny(missing_docs)]

extern crate geohash;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate elastic_types_derive;

extern crate chrono;
extern crate geo as georust;
extern crate geojson;

#[cfg(test)]
#[macro_use]
extern crate json_str;

#[macro_use]
mod private;

pub mod document;
pub mod date;
pub mod boolean;
pub mod geo;
pub mod ip;
pub mod number;
pub mod string;

#[doc(hidden)]
pub mod derive;

pub mod prelude {
    /*!
    Includes all data types.
    
    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use document::prelude::*;

    pub use boolean::prelude::*;
    pub use date::prelude::*;
    pub use geo::prelude::*;
    pub use ip::prelude::*;
    pub use number::prelude::*;
    pub use string::prelude::*;
}

// This is a simple workaround for paths needed by `elastic_types_derive`.
mod elastic_types {
    pub use derive;
}
