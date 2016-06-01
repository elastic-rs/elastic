use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use geojson::{ PointType };
use super::mapping::*;
use ::mapping::{ ElasticType, ElasticFieldMapping };

macro_rules! geo_shape_type {
    ($t:ident, $m:ident, $n:ident) => (
    	/// Geo shape type with a given mapping.
    	#[derive(Debug, Default, Clone)]
		pub struct $t<M> where M: ElasticFieldMapping<()> + $m {
			value: $n,
			phantom: PhantomData<M>
		}
		impl <M> $t<M> where M: ElasticFieldMapping<()> + $m {
			/// Creates a new geo shape with the given mapping.
			pub fn new<I: Into<$n>>(geo: I) -> $t<M> {
				$t {
					value: geo.into(),
					phantom: PhantomData
				}
			}

			/// Get the value of the geo shape.
			pub fn get(&self) -> &$n {
				&self.value
			}

			/// Set the value of the geo shape.
			pub fn set<I: Into<$n>>(&mut self, geo: I) {
				self.value = geo.into()
			}

			/// Change the mapping of this geo shape.
			pub fn remap<MInto: ElasticFieldMapping<()> + $m>(self) -> $t<MInto> {
				$t::<MInto>::new(self.value)
			}
		}

		impl <M> ElasticType<M, ()> for $t<M> where M: ElasticFieldMapping<()> + $m { }

		impl <M> From<$n> for $t<M> where M: ElasticFieldMapping<()> + $m {
			fn from(num: $n) -> Self {
				$t::<M>::new(num)
			}
		}

		//Serialize elastic geo_shape.
		impl <M> Serialize for $t<M> where M: ElasticFieldMapping<()> + $m {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
            S: Serializer {
				self.value.serialize(serializer)
			}
		}

		//Deserialize elastic geo_shape.
		impl <M: ElasticFieldMapping<()> + $m> Deserialize for $t<M> {
			fn deserialize<D>(deserializer: &mut D) -> Result<$t<M>, D::Error> where
            D: Deserializer {
				let t = try!($n::deserialize(deserializer));

				Ok($t::<M>::new(t))
			}
		}
    )
}

geo_shape_type!(ElasticPoint, ElasticPointMapping, PointType);
