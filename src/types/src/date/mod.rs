/*!
Implementation of the Elasticsearch `date` type.

Dates in Elasticsearch are exposed as a formatted `string` which can contain a `date` and/or a `time` component.

All dates used by `elastic_types` are expected to be given in `Utc`, and if no time is supplied, then 12:00am will be used instead.
Where performance is paramount, the `EpochMillis` date format will parse and format dates the fastest.

# Date types

## `Date<M>`

The `Date<M>` type and `chrono`s `DateTime<Utc>` are the main `date` field types you add to document types.
If the mapping and format aren't important, use `DateTime<Utc>`.
If you need to specify mapping properties like `boost`, or use a specific format like `epoch_millis`, use `Date<M>`.

## `DateValue` and `FormattableDateValue<F>`

The `DateValue` and `FormattableDateValue<F>` types are used in methods to represent dates that either don't have a format or have a specific format respectively.
`Date` and `DateTime<Utc>` can freely convert to and from these types, so you probably won't need to interact with them directly.

# Examples

For defining your own date mapping, see [mapping details](mapping/trait.DateMapping.html#derive-mapping).

Map with a default `date`:

```
# extern crate chrono;
# extern crate elastic_types;
# use elastic_types::prelude::*;
# fn main() {
use chrono::{DateTime, Utc};

struct MyType {
    pub field: DateTime<Utc>
}
# }
```

For custom formats, the most ergonomic approach is to declare a type alias using the mapping and format:

```
# use elastic_types::prelude::*;
type Timestamp = Date<DefaultDateMapping<EpochMillis>>;

struct MyType {
    pub field: Timestamp
}
```

Map with a custom `date` mapping:

```
# extern crate serde;
# #[macro_use]
# extern crate elastic_types;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
# fn main() {
# use elastic_types::prelude::*;
# #[derive(Default)]
# struct MyDateMapping;
# impl DateMapping for MyDateMapping { type Format = EpochMillis; }
struct MyType {
    pub field: Date<MyDateMapping>
}
# }
```

## Creating Formats

To make it easier to build your own date formats, derive `ElasticDateFormat` on a unit struct.
This will convert an Elasticsearch format string into a `Vec<chrono::format::Item>` for efficient parsing and formatting at runtime:

```
# #[macro_use]
# extern crate elastic_types;
# #[macro_use]
# extern crate elastic_types_derive;
# extern crate chrono;
# use elastic_types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyy-MM-dd'T'HH:mm:ss")]
struct MyFormat;
# }
```

You can also manually implement `DateFormat` and write your own arbitrary format/parse logic:

```
# extern crate elastic_types;
# extern crate chrono;
# use elastic_types::prelude::*;
# use elastic_types::date::ParseError;
# fn main() {
use chrono::{DateTime, Utc};

#[derive(Default, Clone)]
struct Rfc3339Format;
impl DateFormat for Rfc3339Format {
    fn name() -> &'static str { "yyyy-MM-dd'T'HH:mm:ssZ" }

    fn format<'a>(date: &'a DateValue) -> FormattedDate<'a> {
        date.to_rfc3339().into()
    }

    fn parse(date: &str) -> Result<DateValue, ParseError> {
        let date = DateTime::parse_from_rfc3339(date)?;

        Ok(DateTime::from_utc(date.naive_local(), Utc).into())
    }
}
# }
```

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
*/

pub mod mapping;

mod format;
mod formats;
mod impls;
pub use self::{
    format::*,
    formats::*,
    impls::*,
};

pub mod prelude {
    /*!
    Includes all types for the `date` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        format::{
            DateFormat,
            DateValue,
            FormattableDateValue,
            FormattedDate,
        },
        formats::*,
        impls::*,
        mapping::*,
        DefaultDateFormat,
    };
}
