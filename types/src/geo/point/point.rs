use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use georust::{Coordinate, Point, ToGeo, Geometry};
use ::mapping::ElasticFieldType;
use super::mapping::{GeoPointMapping, DefaultGeoPointMapping, GeoPointFormatWrapper};
use super::GeoPointFormat;

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
/// 		point.x(),
///     point.y()
/// );
/// ```
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/geo-point.html)
#[derive(Debug, Clone, PartialEq)]
pub struct GeoPoint<F, M = DefaultGeoPointMapping<F>>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    /// The `x` and `y` coordinate for the point.
    value: Point,
    _f: PhantomData<F>,
    _t: PhantomData<M>,
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
            _f: PhantomData,
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
        GeoPoint::<F, M>::new(Point(Coordinate { x: x, y: y }))
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

impl<F, M> ElasticFieldType<M, GeoPointFormatWrapper<F>> for GeoPoint<F, M>
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
        GeoPoint::<F, M>::new(Point(point))
    }
}

impl<F, M> ToGeo for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn to_geo(&self) -> Geometry {
        Geometry::Point(self.value.clone())
    }
}

impl<F, M> Serialize for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        F::format::<S, M>(&self.value, serializer)
    }
}

impl<F, M> Deserialize for GeoPoint<F, M>
    where F: GeoPointFormat,
          M: GeoPointMapping<Format = F>
{
    fn deserialize<D>(deserializer: &mut D) -> Result<GeoPoint<F, M>, D::Error>
        where D: Deserializer
    {
        let point = try!(F::parse(deserializer));

        Ok(point.into())
    }
}
