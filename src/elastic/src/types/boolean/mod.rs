/*!
Implementation of the Elasticsearch `boolean` types.

# Examples

For defining your own boolean mapping, see [mapping details](mapping/trait.BooleanMapping.html#derive-mapping).

Map with a default `boolean`:

```
struct MyType {
    pub field: bool
}
```

Map with a custom `boolean`:

```
# #[macro_use] fn main() {
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyBooleanMapping;
# impl BooleanMapping for MyBooleanMapping { }
struct MyType {
    pub field: Boolean<MyBooleanMapping>
}
# }
```

Map a custom type as a `boolean` field:

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
struct MyBooleanField(bool);

impl BooleanFieldType<DefaultBooleanMapping> for MyBooleanField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/boolean.html)
*/
pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for the `boolean` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::*,
        mapping::*,
    };
}
