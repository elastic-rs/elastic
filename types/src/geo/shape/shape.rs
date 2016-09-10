use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use geojson::Geometry;
use super::mapping::*;
use ::mapping::ElasticType;

/// Geo shape type with a given mapping.
///
/// Defining a `geo_shape` with a mapping:
///
/// ```
/// # extern crate elastic_types;
/// extern crate geojson;
/// use geojson::{ Geometry, Value };
///
/// use elastic_types::geo::shape::mapping::DefaultGeoShapeMapping;
/// use elastic_types::geo::shape::GeoShape;
/// # fn main() {
/// let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(
///     Geometry::new(
///         Value::Point(vec![ 1.0, 1.0 ])
///     )
/// );
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GeoShape<M> where 
M: GeoShapeMapping {
    value: Geometry,
    _m: PhantomData<M>
}

impl <M> GeoShape<M> where 
M: GeoShapeMapping {
    /// Creates a new geo shape with the given mapping.
    pub fn new<I: Into<Geometry>>(geo: I) -> GeoShape<M> {
        GeoShape {
            value: geo.into(),
            _m: PhantomData
        }
    }

    /// Get the value of the geo shape.
    pub fn get(&self) -> &Geometry {
        &self.value
    }

    /// Set the value of the geo shape.
    pub fn set<I: Into<Geometry>>(&mut self, geo: I) {
        self.value = geo.into()
    }

    /// Change the mapping of this geo shape.
    pub fn remap<MInto: GeoShapeMapping>(self) -> GeoShape<MInto> {
        GeoShape::<MInto>::new(self.value)
    }
}

impl<'a, M> PartialEq<Geometry> for GeoShape<M> where
M: GeoShapeMapping {
	fn eq(&self, other: &Geometry) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &Geometry) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<GeoShape<M>> for Geometry where
M: GeoShapeMapping {
	fn eq(&self, other: &GeoShape<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &GeoShape<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

impl <M> ElasticType<M, GeoShapeFormat> for GeoShape<M> where 
M: GeoShapeMapping { }

impl <M> From<Geometry> for GeoShape<M> where
M: GeoShapeMapping {
    fn from(geo: Geometry) -> Self {
        GeoShape::<M>::new(geo)
    }
}

impl <M> Serialize for GeoShape<M> where 
M: GeoShapeMapping {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
    S: Serializer {
        self.value.serialize(serializer)
    }
}

impl <M> Deserialize for GeoShape<M> where
M: GeoShapeMapping {
    fn deserialize<D>(deserializer: &mut D) -> Result<GeoShape<M>, D::Error> where
    D: Deserializer {
        let t = try!(Geometry::deserialize(deserializer));

        Ok(GeoShape::<M>::new(t))
    }
}
