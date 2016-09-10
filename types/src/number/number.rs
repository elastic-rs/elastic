use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::*;
use ::mapping::ElasticType;

macro_rules! number_type {
    ($t:ident, $m:ident, $f:ident, $n:ident) => (
    	/// Number type with a given mapping.
    	#[derive(Debug, Default, Clone, PartialEq)]
		pub struct $t<M> where M: $m {
			value: $n,
			_m: PhantomData<M>
		}
		impl <M> $t<M> where M: $m {
			/// Creates a new number with the given mapping.
			pub fn new<I: Into<$n>>(num: I) -> $t<M> {
				$t {
					value: num.into(),
					_m: PhantomData
				}
			}

			/// Get the value of the number.
			pub fn get(&self) -> $n {
				self.value
			}

			/// Set the value of the number.
			pub fn set<I: Into<$n>>(&mut self, num: I) {
				self.value = num.into()
			}

			/// Change the mapping of this number.
			pub fn remap<MInto: $m>(self) -> $t<MInto> {
				$t::<MInto>::new(self.value)
			}
		}

		impl <M> ElasticType<M, $f> for $t<M> where M: $m { }

		impl <M> From<$n> for $t<M> where M: $m {
			fn from(num: $n) -> Self {
				$t::<M>::new(num)
			}
		}

        impl<'a, M> PartialEq<$n> for $t<M> where
        M: $m {
        	fn eq(&self, other: &$n) -> bool {
        		PartialEq::eq(&self.value, other)
        	}

        	fn ne(&self, other: &$n) -> bool {
        		PartialEq::ne(&self.value, other)
        	}
        }

        impl<'a, M> PartialEq<$t<M>> for $n where
        M: $m {
        	fn eq(&self, other: &$t<M>) -> bool {
        		PartialEq::eq(self, &other.value)
        	}

        	fn ne(&self, other: &$t<M>) -> bool {
        		PartialEq::ne(self, &other.value)
        	}
        }

		//Serialize elastic number.
		impl <M> Serialize for $t<M> where M: $m {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
            S: Serializer {
				self.value.serialize(serializer)
			}
		}

		//Deserialize elastic number.
		impl <M: $m> Deserialize for $t<M> {
			fn deserialize<D>(deserializer: &mut D) -> Result<$t<M>, D::Error> where
            D: Deserializer {
				let t = try!($n::deserialize(deserializer));

				Ok($t::<M>::new(t))
			}
		}
    )
}

number_type!(Integer, IntegerMapping, IntegerFormat, i32);
number_type!(Long, LongMapping, LongFormat, i64);
number_type!(Short, ShortMapping, ShortFormat, i16);
number_type!(Byte, ByteMapping, ByteFormat, i8);
number_type!(Float, FloatMapping, FloatFormat, f32);
number_type!(Double, DoubleMapping, DoubleFormat, f64);
