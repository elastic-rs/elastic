use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use geojson::Geometry;
use super::mapping::*;
use ::mapping::{ ElasticType, ElasticFieldMapping };

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
/// use elastic_types::geo::shape::ElasticGeoShape;
/// # fn main() {
/// let point: ElasticGeoShape<DefaultGeoShapeMapping> = ElasticGeoShape::new(
///     Geometry::new(
///         Value::Point(vec![ 1.0, 1.0 ])
///     )
/// );
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ElasticGeoShape<M> where M: ElasticFieldMapping<()> + ElasticGeoShapeMapping {
    value: Geometry,
    phantom: PhantomData<M>
}
impl <M> ElasticGeoShape<M> where M: ElasticFieldMapping<()> + ElasticGeoShapeMapping {
    /// Creates a new geo shape with the given mapping.
    pub fn new<I: Into<Geometry>>(geo: I) -> ElasticGeoShape<M> {
        ElasticGeoShape {
            value: geo.into(),
            phantom: PhantomData
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
    pub fn remap<MInto: ElasticFieldMapping<()> + ElasticGeoShapeMapping>(self) -> ElasticGeoShape<MInto> {
        ElasticGeoShape::<MInto>::new(self.value)
    }
}

impl <M> ElasticType<M, ()> for ElasticGeoShape<M> where M: ElasticFieldMapping<()> + ElasticGeoShapeMapping { }

impl <M> From<Geometry> for ElasticGeoShape<M> where M: ElasticFieldMapping<()> + ElasticGeoShapeMapping {
    fn from(geo: Geometry) -> Self {
        ElasticGeoShape::<M>::new(geo)
    }
}

impl <M> Serialize for ElasticGeoShape<M> where M: ElasticFieldMapping<()> + ElasticGeoShapeMapping {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
    S: Serializer {
        self.value.serialize(serializer)
    }
}

impl <M: ElasticFieldMapping<()> + ElasticGeoShapeMapping> Deserialize for ElasticGeoShape<M> {
    fn deserialize<D>(deserializer: &mut D) -> Result<ElasticGeoShape<M>, D::Error> where
    D: Deserializer {
        let t = try!(Geometry::deserialize(deserializer));

        Ok(ElasticGeoShape::<M>::new(t))
    }
}
