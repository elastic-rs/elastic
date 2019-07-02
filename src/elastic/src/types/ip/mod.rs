/*!
Implementation of the Elasticsearch `ip` type.

# Examples

For defining your own ip mapping, see [mapping details](mapping/trait.IpMapping.html#derive-mapping).

Map with a default `ip`:

```
# use std::net::Ipv4Addr;
struct MyType {
    pub field: std::net::Ipv4Addr
}
```

Map with a custom `ip`:

```
#[macro_use]
# fn main() {
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyIpMapping;
# impl IpMapping for MyIpMapping {}
struct MyType {
    pub field: Ip<MyIpMapping>
}
# }
```

Map a custom type as an `ip` field:

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
struct MyIpField(String);

impl IpFieldType<DefaultIpMapping> for MyIpField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/ip.html)
*/

pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for the `ip` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::*,
        mapping::*,
    };
}
