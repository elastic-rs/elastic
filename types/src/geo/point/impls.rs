use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use georust::{ToGeo, Geometry as GeoEnum};
use super::mapping::{GeoPointFieldType, GeoPointMapping, DefaultGeoPointMapping};
use super::{Coordinate, Point, Geometry, GeoPointFormat};

/// An Elasticsearch `geo_point` type with a format.
///
/// The [format](format/index.html) is provided as a generic parameter.
/// This struct wraps up a `geo::Point` struct, which have an `x` and `y` floating point value.
///
/// # Examples
/// Defining a geo point using the default format:
///
/// ```
/// # use elastic_types::geo::point::{ GeoPoint, DefaultGeoPointFormat };
/// let point: GeoPoint<DefaultGeoPointFormat> = GeoPoint::build(1.0, 1.0);
/// ```
///
/// Defining a geo point using a named format:
///
/// ```
/// # use elastic_types::geo::point::{ GeoPoint, GeoPointString };
/// let point: GeoPoint<GeoPointString> = GeoPoint::build(1.0, 1.0);
/// ```
///
/// Defining a geo point using a custom mapping:
///
/// ```
/// # use elastic_types::geo::point::mapping::DefaultGeoPointMapping;
/// # use elastic_types::geo::point::{ GeoPoint, GeoPointString };
///
/// let point: GeoPoint<GeoPointString, DefaultGeoPointMapping<_>> = GeoPoint::build(1.0, 1.0);
/// ```
///
/// Accessing the values of a geo point:
///
/// ```
/// # use elastic_types::geo::point::{ GeoPoint, DefaultGeoPointFormat };
/// let point: GeoPoint<DefaultGeoPointFormat> = GeoPoint::build(1.0, 1.0);
///
/// //eg: (1.0,1.0)
/// println!("({},{})",
///         point.x(),
///     point.y()
/// );
/// ```
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
#[derive(Debug, Clone, PartialEq)]
pub struct GeoPoint<F, M = DefaultGeoPointMapping<F>>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    /// The `x` and `y` coordinate for the point.
    value: Point,
    _t: PhantomData<(M, F)>,
}

impl<F, M> GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    /// Creates a new `GeoPoint` from the given coordinate.
    ///
    /// This function will consume the provided `Coordinate`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate elastic_types;
    /// # extern crate geo;
    /// # fn main() {
    /// use geo::{ Point, Coordinate };
    /// use elastic_types::geo::point::{ GeoPoint, DefaultGeoPointFormat };
    ///
    /// //Create a geo Coordinate struct
    /// let coord = Coordinate { x: 1.0, y: 1.0 };
    ///
    /// //Give it to the GeoPoint struct
    /// let point: GeoPoint<DefaultGeoPointFormat> = GeoPoint::new(Point(coord));
    /// # }
    /// ```
    pub fn new<I>(point: I) -> GeoPoint<F, M>
        where I: Into<Point>
    {
        GeoPoint {
            value: point.into(),
            _t: PhantomData,
        }
    }

    /// Creates an `GeoPoint` from the given `x` and `y` primitives:
    ///
    /// ```
    /// # use elastic_types::geo::point::{ GeoPoint, DefaultGeoPointFormat };
    /// let point: GeoPoint<DefaultGeoPointFormat> = GeoPoint::build(1.0, 1.0);
    /// ```
    pub fn build(x: f64, y: f64) -> GeoPoint<F, M> {
        GeoPoint::<F, M>::new(Point::new(x, y))
    }

    /// Change the format/mapping of this geo point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::geo::point::{ GeoPoint, GeoPointString, GeoPointObject };
    /// //Get a point formatted as a string
    /// let point: GeoPoint<GeoPointString> = GeoPoint::build(1.0, 1.0);
    ///
    /// //Change the format to an object
    /// let otherpoint: GeoPoint<GeoPointObject> = point.remap();
    /// ```
    pub fn remap<FInto, MInto>(self) -> GeoPoint<FInto, MInto>
        where FInto: GeoPointFormat,
              MInto: GeoPointMapping<Format = FInto>
    {
        GeoPoint::<FInto, MInto>::new(self.value)
    }
}

impl<F, M> GeoPointFieldType<M, F> for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
}

impl_mapping_type!(Point, GeoPoint, GeoPointMapping, GeoPointFormat);

impl<F, M> From<Coordinate> for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn from(point: Coordinate) -> GeoPoint<F, M> {
        GeoPoint::<F, M>::new(Point::new(point.x, point.y))
    }
}

impl<F, M> ToGeo<f64> for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn to_geo(&self) -> Geometry {
        GeoEnum::Point(self.value.clone())
    }
}

impl<F, M> Serialize for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        F::format::<S, M>(&self.value, serializer)
    }
}

impl<'de, F, M> Deserialize<'de> for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn deserialize<D>(deserializer: D) -> Result<GeoPoint<F, M>, D::Error>
        where D: Deserializer<'de>
    {
        let point = try!(F::parse(deserializer));

        Ok(point.into())
    }
}
