use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use super::mapping::*;
use ::field::FieldType;

macro_rules! number_type {
    ($wrapper_ty:ident, $mapping_ty:ident, $format_ty:ident, $std_ty:ident) => (
        /// Number type with a given mapping.
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct $wrapper_ty<M> where M: $mapping_ty {
            value: $std_ty,
            _m: PhantomData<M>
        }
        impl <M> $wrapper_ty<M> where M: $mapping_ty {
            /// Creates a new number with the given mapping.
            pub fn new<I: Into<$std_ty>>(num: I) -> $wrapper_ty<M> {
                $wrapper_ty {
                    value: num.into(),
                    _m: PhantomData
                }
            }

            /// Change the mapping of this number.
            pub fn remap<MInto: $mapping_ty>(self) -> $wrapper_ty<MInto> {
                $wrapper_ty::<MInto>::new(self.value)
            }
        }

        impl <M> FieldType<M, $format_ty> for $wrapper_ty<M> where M: $mapping_ty { }

        impl_mapping_type!($std_ty, $wrapper_ty, $mapping_ty);

        //Serialize elastic number.
        impl <M> Serialize for $wrapper_ty<M> where M: $mapping_ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
                self.value.serialize(serializer)
            }
        }

        //Deserialize elastic number.
        impl <M: $mapping_ty> Deserialize for $wrapper_ty<M> {
            fn deserialize<D>(deserializer: D) -> Result<$wrapper_ty<M>, D::Error> where
            D: Deserializer {
                let t = try!($std_ty::deserialize(deserializer));

                Ok($wrapper_ty::<M>::new(t))
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
