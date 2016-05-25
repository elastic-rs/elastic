use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use georust::{ Coordinate, Point, ToGeo, Geometry };
use ::mapping::{ ElasticFieldMapping, ElasticType };
use super::mapping::{ ElasticGeoPointMapping, DefaultGeoPointMapping };
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
/// use elastic_types::geo::point::{ ElasticGeoPoint, DefaultGeoPointFormat };
///
/// let point: ElasticGeoPoint<DefaultGeoPointFormat> = ElasticGeoPoint::build(1.0, 1.0);
/// ```
///
/// Defining a geo point using a named format:
///
/// ```
/// use elastic_types::geo::point::{ ElasticGeoPoint, GeoPointString };
///
/// let point: ElasticGeoPoint<GeoPointString> = ElasticGeoPoint::build(1.0, 1.0);
/// ```
///
/// Defining a geo point using a custom mapping:
///
/// ```
/// use elastic_types::geo::point::mapping::DefaultGeoPointMapping;
/// use elastic_types::geo::point::{ ElasticGeoPoint, GeoPointString };
///
/// let point: ElasticGeoPoint<GeoPointString, DefaultGeoPointMapping<_>> = ElasticGeoPoint::build(1.0, 1.0);
/// ```
///
/// Accessing the values of a geo point:
///
/// ```
/// use elastic_types::geo::point::{ ElasticGeoPoint, DefaultGeoPointFormat };
///
/// let point: ElasticGeoPoint<DefaultGeoPointFormat> = ElasticGeoPoint::build(1.0, 1.0);
///
/// //eg: (1.0,1.0)
/// println!("({},{})",
///		point.x(),
///     point.y()
/// );
/// ```
///
/// # Links
/// - _TODO: Check link_ [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo_point.html)
pub struct ElasticGeoPoint<F, T = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    /// The `x` and `y` coordinate for the point.
    pub value: Point,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    /// Creates a new `ElasticGeoPoint` from the given coordinate.
	///
	/// This function will consume the provided `Coordinate`.
	///
	/// # Examples
	///
	/// Create an `ElasticGeoPoint` from the given `geo::Coordinate`:
	///
	/// ```
	/// # extern crate elastic_types;
	/// # extern crate geo;
	/// # fn main() {
	/// use geo::Coordinate;
	/// use elastic_types::geo::point::{ ElasticGeoPoint, DefaultGeoPointFormat };
	///
	/// //Create a geo Coordinate struct
	/// let coord = Coordinate { x: 1.0, y: 1.0 };
	///
	/// //Give it to the ElasticGeoPoint struct
	/// let point: ElasticGeoPoint<DefaultGeoPointFormat> = ElasticGeoPoint::new(coord);
	/// # }
	/// ```
    pub fn new(point: Coordinate) -> ElasticGeoPoint<F, T> {
        ElasticGeoPoint {
            value: Point(point),
            phantom_f: PhantomData,
            phantom_t: PhantomData
        }
    }

    /// Creates an `ElasticGeoPoint` from the given `x` and `y` primitives:
	///
	/// ```
	/// use elastic_types::geo::point::{ ElasticGeoPoint, DefaultGeoPointFormat };
	///
	/// let point: ElasticGeoPoint<DefaultGeoPointFormat> = ElasticGeoPoint::build(1.0, 1.0);
	/// ```
    pub fn build(x: f64, y: f64) -> ElasticGeoPoint<F, T> {
        ElasticGeoPoint::<F, T>::new(Coordinate { x: x, y: y })
    }

    /// Get the `x` part of the coordinate.
    pub fn x(&self) -> f64 {
        self.value.x()
    }

    /// Get the `y` part of the coordinate.
    pub fn y(&self) -> f64 {
        self.value.y()
    }

    /// Change the format/mapping of this geo point.
    ///
	/// # Examples
	///
	/// ```
	/// use elastic_types::geo::point::{ ElasticGeoPoint, GeoPointString, GeoPointObject };
	///
	/// //Get a point formatted as a string
	/// let point: ElasticGeoPoint<GeoPointString> = ElasticGeoPoint::build(1.0, 1.0);
	///
	/// //Change the format to an object
	/// let otherpoint: ElasticGeoPoint<GeoPointObject> = point.remap();
	/// ```
	pub fn remap<FInto, TInto>(self) -> ElasticGeoPoint<FInto, TInto> where
	FInto: GeoPointFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticGeoPointMapping<FInto> {
		ElasticGeoPoint::<FInto, TInto>::new(self.value.0)
	}
}

impl <F, T> ElasticType<T, F> for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {

}

impl <F, T> From<Coordinate> for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn from(point: Coordinate) -> ElasticGeoPoint<F, T> {
		ElasticGeoPoint::<F, T>::new(point)
	}
}

impl <F, T> From<Point> for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn from(point: Point) -> ElasticGeoPoint<F, T> {
		ElasticGeoPoint::<F, T>::new(point.0)
	}
}

impl <F, T> ToGeo for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    fn to_geo(&self) -> Geometry {
        Geometry::Point(self.value.clone())
    }
}

impl <F, T> Serialize for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		F::format::<S, T>(&self.value, serializer)
	}
}

impl <F, T> Deserialize for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticGeoPoint<F, T>, D::Error> where
	D: Deserializer {
        let point = try!(F::parse(deserializer));

        Ok(point.into())
    }
}
