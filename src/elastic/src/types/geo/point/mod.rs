/*!
Implementation of the Elasticsearch `geo_point` type.

Geo points are an Elasticsearch specific geospatial type with an `x` (`lon`) and `y` (`lat`)
component.
`GeoPoint` is a good choice for storing and analysing geospatial points where geojson
compatibility isn't needed.

# Examples

For defining your own geo point mapping, see [mapping details](mapping/trait.GeoPointMapping.html#derive-mapping).

Map with a default `geo_point`:

```
# use elastic::types::geo::point::prelude::*;
struct MyType {
    pub field: GeoPoint<DefaultGeoPointMapping>
}
```

Map with a custom `geo_point`:

```
# #[macro_use] use std::marker::PhantomData;
# use elastic::types::prelude::*;
# fn main() {
# use elastic::types::prelude::*;
# use elastic::types::geo::point::prelude::*;
# #[derive(Default)]
# struct MyGeoPointMapping;
# impl GeoPointMapping for MyGeoPointMapping { type Format = GeoPointString; }
struct MyType {
    pub field: GeoPoint<MyGeoPointMapping>
}
# }
```

Map a custom type as a `geo_point` field:

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
struct MyGeoPointField(f32, f32);

impl GeoPointFieldType<DefaultGeoPointMapping<GeoPointObject>> for MyGeoPointField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/geo-point.html)
*/

use geo::{
    Coordinate as C,
    Geometry as G,
    Point as P,
};

type Coordinate = C<f64>;
type Point = P<f64>;
type Geometry = G<f64>;

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
    Includes all types for the `geo_point` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        format::*,
        formats::*,
        impls::*,
        mapping::*,
        DefaultGeoPointFormat,
    };
}
