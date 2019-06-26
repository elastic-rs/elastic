/*!
Implementation of the Elasticsearch `geo` types.

Use [`point::GeoPoint`](point/struct.GeoPoint.html) for indexing simple geo points with an `x` and `y` coordinate.

Use [`shape::GeoShape`](shape/struct.GeoShape.html) for indexing `geojson`.
*/

pub mod mapping;
pub mod point;
pub mod shape;

pub mod prelude {
    /*!
    Includes all types for the `geo_point` and `geo_shape` types.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        mapping::*,
        point::prelude::*,
        shape::prelude::*,
    };
}
