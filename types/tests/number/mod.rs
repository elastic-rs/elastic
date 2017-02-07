pub mod mapping;

use serde_json;

use elastic_types::prelude::*;
use ::number_fixtures::*;

#[test]
fn can_change_number_mapping() {
    fn takes_custom_mapping(_: Integer<MyIntegerMapping>) -> bool {
        true
    }

    let number: Integer<DefaultIntegerMapping> = Integer::new(1);

    assert!(takes_custom_mapping(number.remap()));
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
        }
    ];

    let expected_ser = vec![
        "1",
        "1",
        "1",
        "1",
        "1.01",
        "1.01"
    ];

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
