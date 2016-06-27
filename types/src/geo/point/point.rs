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
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
#[derive(Debug, Clone, PartialEq)]
pub struct ElasticGeoPoint<F, M = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    /// The `x` and `y` coordinate for the point.
    value: Point,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<M>
}

impl <F, M> ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
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
    pub fn new(point: Coordinate) -> ElasticGeoPoint<F, M> {
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
    pub fn build(x: f64, y: f64) -> ElasticGeoPoint<F, M> {
        ElasticGeoPoint::<F, M>::new(Coordinate { x: x, y: y })
    }

    /// Get the underlying `Point` coordinate.
    pub fn get(&self) -> &Point {
        &self.value
    }

    /// Set the underlying `Point` coordinate.
    pub fn set(&mut self, point: Coordinate) {
        self.value = Point(point);
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
	pub fn remap<FInto, MInto>(self) -> ElasticGeoPoint<FInto, MInto> where
	FInto: GeoPointFormat,
	MInto: ElasticFieldMapping<FInto> + ElasticGeoPointMapping<FInto> {
		ElasticGeoPoint::<FInto, MInto>::new(self.value.0)
	}
}

impl <F, M> ElasticType<M, F> for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {

}

impl <F, M> From<Coordinate> for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn from(point: Coordinate) -> ElasticGeoPoint<F, M> {
		ElasticGeoPoint::<F, M>::new(point)
	}
}

impl <F, M> From<Point> for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn from(point: Point) -> ElasticGeoPoint<F, M> {
		ElasticGeoPoint::<F, M>::new(point.0)
	}
}

impl <F, M> ToGeo for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    fn to_geo(&self) -> Geometry {
        Geometry::Point(self.value.clone())
    }
}

impl<'a, F, M> PartialEq<Point> for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<()> + ElasticGeoPointMapping<F> {
	fn eq(&self, other: &Point) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &Point) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, F, M> PartialEq<ElasticGeoPoint<F, M>> for Point where
F: GeoPointFormat,
M: ElasticFieldMapping<()> + ElasticGeoPointMapping<F> {
	fn eq(&self, other: &ElasticGeoPoint<F, M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticGeoPoint<F, M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl <F, M> Serialize for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		F::format::<S, M>(&self.value, serializer)
	}
}

impl <F, M> Deserialize for ElasticGeoPoint<F, M> where
F: GeoPointFormat,
M: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticGeoPoint<F, M>, D::Error> where
	D: Deserializer {
        let point = try!(F::parse(deserializer));

        Ok(point.into())
    }
}
