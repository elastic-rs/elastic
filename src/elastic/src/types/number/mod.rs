/*!
Implementation of the Elasticsearch `number` types.

Numeric types come in a number of flavours that correspond to primitives in Rust:

Rust | Elasticsearch
------ | ------------------
`i64` |  `long`
`i32` | `integer`
`i16` | `short`
`i8` | `byte`
`f64` | `double`
`f32` | `float`

For mapping a number with the default mapping, you can use the Rust primitive.
If you need to use a custom mapping, then there is an `Elastic*` type for each number.

# Examples

For defining your own number mapping, see [mapping details](mapping/index.html#derive-mapping).

Map with a default `number` (`integer` in this case):

```
struct MyType {
    pub field: i32
}
```

Map with a custom `number` (`integer` in this case):

```
# #[macro_use] use elastic::types::prelude::*;
# fn main() {
# #[derive(Default)]
# struct MyIntegerMapping;
# impl IntegerMapping for MyIntegerMapping { }
struct MyType {
    pub field: Integer<MyIntegerMapping>
}
# }
```

Map a custom type as a `number` field (`integer` in this case):

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
struct MyIntegerField(i32);

impl IntegerFieldType<DefaultIntegerMapping> for MyIntegerField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/number.html)
*/

pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for the `number` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::*,
        mapping::*,
    };
}
