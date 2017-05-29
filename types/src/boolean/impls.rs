use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::mapping::{BooleanFieldType, BooleanMapping, DefaultBooleanMapping};

impl BooleanFieldType<DefaultBooleanMapping> for bool {}

/**
An Elasticsearch `boolean` with a mapping.

Where the mapping isn't custom, you can use the standard library `bool` instead.

# Examples

Defining a `bool` with a mapping:

```
# use elastic_types::prelude::*;
let boolean = Boolean::<DefaultBooleanMapping>::new(true);
```
*/
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Boolean<M>
    where M: BooleanMapping
{
    value: bool,
    _m: PhantomData<M>,
}
impl<M> Boolean<M>
    where M: BooleanMapping
{
    /**
    Creates a new `Boolean` with the given mapping.
    
    # Examples
    
    Create a new `Boolean` from a `bool`:
    
    ```
    # use elastic_types::prelude::*;
    let boolean = Boolean::<DefaultBooleanMapping>::new(false);
    ```
    */
    pub fn new<I>(boolean: I) -> Boolean<M>
        where I: Into<bool>
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
    # extern crate serde;
    # #[macro_use]
    # extern crate elastic_types;
    # fn main() {
    # use elastic_types::prelude::*;
    # #[derive(Default)]
    # struct MyBooleanMapping;
    # impl BooleanMapping for MyBooleanMapping { }
    let es_boolean = Boolean::<DefaultBooleanMapping>::new(true);
    
    let boolean: Boolean<MyBooleanMapping> = es_boolean.remap();
    # }
    ```
    */
    pub fn remap<MInto>(self) -> Boolean<MInto>
        where MInto: BooleanMapping
    {
        Boolean::<MInto>::new(self.value)
    }
}

impl<M> BooleanFieldType<M> for Boolean<M> where M: BooleanMapping {}

impl_mapping_type!(bool, Boolean, BooleanMapping);

impl<M> Serialize for Boolean<M>
    where M: BooleanMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bool(self.value)
    }
}

impl<'de, M> Deserialize<'de> for Boolean<M>
    where M: BooleanMapping
{
    fn deserialize<D>(deserializer: D) -> Result<Boolean<M>, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Default)]
        struct BooleanVisitor<M>
            where M: BooleanMapping
        {
            _m: PhantomData<M>,
        }

        impl<'de, M> Visitor<'de> for BooleanVisitor<M>
            where M: BooleanMapping
        {
            type Value = Boolean<M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json boolean")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Boolean<M>, E>
                where E: Error
            {
                Ok(Boolean::<M>::new(v))
            }
        }

        deserializer.deserialize_any(BooleanVisitor::<M>::default())
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use prelude::*;

    #[derive(Default)]
    struct MyBooleanMapping;
    impl BooleanMapping for MyBooleanMapping {}

    #[test]
    fn can_change_boolean_mapping() {
        fn takes_custom_mapping(_: Boolean<MyBooleanMapping>) -> bool {
            true
        }

        let boolean: Boolean<DefaultBooleanMapping> = Boolean::new(true);

        assert!(takes_custom_mapping(boolean.remap()));
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
