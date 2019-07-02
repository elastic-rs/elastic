use super::mapping::{
    BooleanFieldType,
    BooleanMapping,
    DefaultBooleanMapping,
};
use serde::{
    de::{
        Error,
        Visitor,
    },
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    borrow::Borrow,
    marker::PhantomData,
};

impl BooleanFieldType<DefaultBooleanMapping> for bool {}

/**
An Elasticsearch `boolean` with a mapping.

Where the mapping isn't custom, you can use the standard library `bool` instead.

# Examples

Defining a `bool` with a mapping:

```
# use elastic::types::prelude::*;
let boolean = Boolean::<DefaultBooleanMapping>::new(true);
```
*/
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Boolean<TMapping>
where
    TMapping: BooleanMapping,
{
    value: bool,
    _m: PhantomData<TMapping>,
}

impl<TMapping> Boolean<TMapping>
where
    TMapping: BooleanMapping,
{
    /**
    Creates a new `Boolean` with the given mapping.

    # Examples

    Create a new `Boolean` from a `bool`:

    ```
    # use elastic::types::prelude::*;
    let boolean = Boolean::<DefaultBooleanMapping>::new(false);
    ```
    */
    pub fn new<I>(boolean: I) -> Boolean<TMapping>
    where
        I: Into<bool>,
    {
        Boolean {
            value: boolean.into(),
            _m: PhantomData,
        }
    }

    /**
    Change the mapping of this boolean.

    # Examples

    Change the mapping for a given `Boolean`:

    ```
    # #[macro_use] fn main() {
    # use elastic::types::prelude::*;
    # #[derive(Default)]
    # struct MyBooleanMapping;
    # impl BooleanMapping for MyBooleanMapping { }
    let boolean = Boolean::<DefaultBooleanMapping>::new(true);

    let boolean: Boolean<MyBooleanMapping> = Boolean::remap(boolean);
    # }
    ```
    */
    pub fn remap<TNewMapping>(boolean: Boolean<TMapping>) -> Boolean<TNewMapping>
    where
        TNewMapping: BooleanMapping,
    {
        Boolean::<TNewMapping>::new(boolean.value)
    }
}

impl<TMapping> BooleanFieldType<TMapping> for Boolean<TMapping> where TMapping: BooleanMapping {}

impl_mapping_type!(bool, Boolean, BooleanMapping);

impl<TMapping> Serialize for Boolean<TMapping>
where
    TMapping: BooleanMapping,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

impl<'de, TMapping> Deserialize<'de> for Boolean<TMapping>
where
    TMapping: BooleanMapping,
{
    fn deserialize<D>(deserializer: D) -> Result<Boolean<TMapping>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BooleanVisitor<TMapping> {
            _m: PhantomData<TMapping>,
        }

        impl<'de, TMapping> Visitor<'de> for BooleanVisitor<TMapping>
        where
            TMapping: BooleanMapping,
        {
            type Value = Boolean<TMapping>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json boolean")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Boolean<TMapping>, E>
            where
                E: Error,
            {
                Ok(Boolean::<TMapping>::new(v))
            }
        }

        deserializer.deserialize_any(BooleanVisitor::<TMapping> { _m: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::prelude::*;

    #[derive(Default)]
    struct MyBooleanMapping;
    impl BooleanMapping for MyBooleanMapping {}

    #[test]
    fn can_change_boolean_mapping() {
        fn takes_custom_mapping(_: Boolean<MyBooleanMapping>) -> bool {
            true
        }

        let boolean: Boolean<DefaultBooleanMapping> = Boolean::new(true);

        assert!(takes_custom_mapping(Boolean::remap(boolean)));
    }

    #[test]
    fn serialise_elastic_boolean() {
        let boolean: Boolean<DefaultBooleanMapping> = Boolean::new(true);

        let ser = serde_json::to_string(&boolean).unwrap();

        assert_eq!("true", ser);
    }

    #[test]
    fn deserialise_elastic_boolean() {
        let boolean: Boolean<DefaultBooleanMapping> = serde_json::from_str("true").unwrap();

        assert_eq!(true, boolean);
    }

}
