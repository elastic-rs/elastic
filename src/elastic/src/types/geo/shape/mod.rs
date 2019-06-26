/*!
Implementation of the Elasticsearch `geo_shape` type.

Geo shape is a wrapper for storing [geojson](http://geojson.org/) structures in Elasticsearch.

# Examples

For defining your own geo shape mapping, see [mapping details](mapping/trait.GeoShapeMapping.html#derive-mapping).

Map with a default `geo_shape`:

```
# use elastic::types::geo::shape::prelude::*;
# use elastic::types::geo::shape::mapping::*;
struct MyType {
    pub field: GeoShape<DefaultGeoShapeMapping>
}
```

Map with a custom `geo_shape`:

```
# #[macro_use] use elastic::types::prelude::*;
# use std::marker::PhantomData;
# fn main() {
# use elastic::types::prelude::*;
# use elastic::types::geo::shape::prelude::*;
# #[derive(Default)]
# struct MyGeoShapeMapping;
# impl GeoShapeMapping for MyGeoShapeMapping {}
struct MyType {
    pub field: GeoShape<MyGeoShapeMapping>
}
# }
```

Map a custom type as a `geo_shape` field:

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
struct MyGeoShapeField(String);

impl GeoShapeFieldType<DefaultGeoShapeMapping> for MyGeoShapeField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/geo-shape.html)
*/

pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for the `geo_shape` types.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::*,
        mapping::*,
    };
}
