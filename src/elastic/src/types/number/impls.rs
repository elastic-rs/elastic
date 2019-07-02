use super::mapping::*;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    borrow::Borrow,
    marker::PhantomData,
};

macro_rules! number_type {
    ($wrapper_ty:ident, $mapping_ty:ident, $field_trait:ident, $std_ty:ident) => {
        /** Number type with a given mapping. */
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct $wrapper_ty<TMapping>
        where
            TMapping: $mapping_ty,
        {
            value: $std_ty,
            _m: PhantomData<TMapping>,
        }

        impl<TMapping> $wrapper_ty<TMapping>
        where
            TMapping: $mapping_ty,
        {
            /** Creates a new number with the given mapping. */
            pub fn new<I: Into<$std_ty>>(num: I) -> $wrapper_ty<TMapping> {
                $wrapper_ty { value: num.into(), _m: PhantomData }
            }

            /** Change the mapping of this number. */
            pub fn remap<TNewMapping>(number: $wrapper_ty<TMapping>) -> $wrapper_ty<TNewMapping>
            where
                TNewMapping: $mapping_ty,
            {
                $wrapper_ty::new(number.value)
            }
        }

        impl<TMapping> $field_trait<TMapping> for $wrapper_ty<TMapping> where TMapping: $mapping_ty {}

        impl_mapping_type!($std_ty, $wrapper_ty, $mapping_ty);

        //Serialize elastic number.
        impl<TMapping> Serialize for $wrapper_ty<TMapping>
        where
            TMapping: $mapping_ty,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        //Deserialize elastic number.
        impl<'de, TMapping> Deserialize<'de> for $wrapper_ty<TMapping>
        where
            TMapping: $mapping_ty,
        {
            fn deserialize<D>(deserializer: D) -> Result<$wrapper_ty<TMapping>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let t = $std_ty::deserialize(deserializer)?;

                Ok($wrapper_ty::new(t))
            }
        }
    };
}

number_type!(Integer, IntegerMapping, IntegerFieldType, i32);
number_type!(Long, LongMapping, LongFieldType, i64);
number_type!(Short, ShortMapping, ShortFieldType, i16);
number_type!(Byte, ByteMapping, ByteFieldType, i8);
number_type!(Float, FloatMapping, FloatFieldType, f32);
number_type!(Double, DoubleMapping, DoubleFieldType, f64);

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::prelude::*;

    #[derive(Default)]
    struct MyIntegerMapping;
    impl IntegerMapping for MyIntegerMapping {}

    #[derive(Default)]
    struct MyLongMapping;
    impl LongMapping for MyLongMapping {}

    #[derive(Default)]
    struct MyShortMapping;
    impl ShortMapping for MyShortMapping {}

    #[derive(Default)]
    struct MyByteMapping;
    impl ByteMapping for MyByteMapping {}

    #[derive(Default)]
    struct MyFloatMapping;
    impl FloatMapping for MyFloatMapping {}

    #[derive(Default)]
    struct MyDoubleMapping;
    impl DoubleMapping for MyDoubleMapping {}

    #[test]
    fn can_change_number_mapping() {
        fn takes_custom_mapping(_: Integer<MyIntegerMapping>) -> bool {
            true
        }

        let number: Integer<DefaultIntegerMapping> = Integer::new(1);

        assert!(takes_custom_mapping(Integer::remap(number)));
    }

    #[test]
    fn serialise_elastic_numbers() {
        let ser = vec![
            {
                let num = Integer::<MyIntegerMapping>::new(1i32);
                serde_json::to_string(&num).unwrap()
            },
            {
                let num = Long::<MyLongMapping>::new(1i64);
                serde_json::to_string(&num).unwrap()
            },
            {
                let num = Short::<MyShortMapping>::new(1i16);
                serde_json::to_string(&num).unwrap()
            },
            {
                let num = Byte::<MyByteMapping>::new(1i8);
                serde_json::to_string(&num).unwrap()
            },
            {
                let num = Float::<MyFloatMapping>::new(1.01f32);
                serde_json::to_string(&num).unwrap()
            },
            {
                let num = Double::<MyDoubleMapping>::new(1.01f64);
                serde_json::to_string(&num).unwrap()
            },
        ];

        let expected_ser = vec!["1", "1", "1", "1", "1.01", "1.01"];

        let mut success = true;
        for i in 0..ser.len() {
            if expected_ser[i] != &ser[i] {
                success = false;
                break;
            }
        }

        assert!(success);
    }

    #[test]
    fn deserialise_elastic_numbers() {
        let int_de: Integer<MyIntegerMapping> = serde_json::from_str("1").unwrap();
        let long_de: Long<MyLongMapping> = serde_json::from_str("1").unwrap();
        let short_de: Short<MyShortMapping> = serde_json::from_str("1").unwrap();
        let byte_de: Byte<MyByteMapping> = serde_json::from_str("1").unwrap();
        let float_de: Float<MyFloatMapping> = serde_json::from_str("1.01").unwrap();
        let double_de: Double<MyDoubleMapping> = serde_json::from_str("1.01").unwrap();

        assert_eq!(
            (1i32, 1i64, 1i16, 1i8, 1.01f32, 1.01f64),
            (*int_de, *long_de, *short_de, *byte_de, *float_de, *double_de)
        );
    }
}
