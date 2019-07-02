use super::mapping::{
    DefaultIpMapping,
    IpFieldType,
    IpMapping,
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
    error::Error as StdError,
    marker::PhantomData,
    net::Ipv4Addr,
    str::FromStr,
};

impl IpFieldType<DefaultIpMapping> for Ipv4Addr {}

/**
An Elasticsearch `ip` with a mapping.

Where the mapping isn't custom, you can use the standard library `Ipv4Addr` instead.

# Examples

Defining an `ip` with a mapping:

```
use std::net::Ipv4Addr;
use elastic::types::ip::mapping::DefaultIpMapping;
use elastic::types::ip::Ip;

let ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
```
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Ip<TMapping>
where
    TMapping: IpMapping,
{
    value: Ipv4Addr,
    _m: PhantomData<TMapping>,
}

impl<TMapping> Ip<TMapping>
where
    TMapping: IpMapping,
{
    /**
    Creates a new `Ip` with the given mapping.

    # Examples

    Create a new `Ip` from a `Ip4vAddr`:

    ```
    use std::net::Ipv4Addr;
    use elastic::types::ip::mapping::DefaultIpMapping;
    use elastic::types::ip::Ip;

    let ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
    ```
    */
    pub fn new<I>(ip: I) -> Ip<TMapping>
    where
        I: Into<Ipv4Addr>,
    {
        Ip {
            value: ip.into(),
            _m: PhantomData,
        }
    }

    /**
    Change the mapping of this ip.

    # Examples

    Change the mapping for a given `Ip`:

    ```
    # #[macro_use] fn main() {
    # use std::net::Ipv4Addr;
    # use elastic::types::prelude::*;
    # #[derive(Default)]
    # struct MyIpMapping;
    # impl IpMapping for MyIpMapping { }
    let es_ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));

    let ip: Ip<MyIpMapping> = Ip::remap(es_ip);
    # }
    ```
    */
    pub fn remap<TNewMapping>(ip: Ip<TMapping>) -> Ip<TNewMapping>
    where
        TNewMapping: IpMapping,
    {
        Ip::new(ip.value)
    }
}

impl<TMapping> IpFieldType<TMapping> for Ip<TMapping> where TMapping: IpMapping {}

impl_mapping_type!(Ipv4Addr, Ip, IpMapping);

// Serialize elastic ip
impl<TMapping> Serialize for Ip<TMapping>
where
    TMapping: IpMapping,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value.to_string())
    }
}

// Deserialize elastic ip
impl<'de, TMapping> Deserialize<'de> for Ip<TMapping>
where
    TMapping: IpMapping,
{
    fn deserialize<D>(deserializer: D) -> Result<Ip<TMapping>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IpVisitor<TMapping> {
            _m: PhantomData<TMapping>,
        }

        impl<'de, TMapping> Visitor<'de> for IpVisitor<TMapping>
        where
            TMapping: IpMapping,
        {
            type Value = Ip<TMapping>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string containing an IpV4 address")
            }

            fn visit_string<E>(self, v: String) -> Result<Ip<TMapping>, E>
            where
                E: Error,
            {
                let de =
                    Ipv4Addr::from_str(&v).map_err(|e| E::custom(e.description().to_string()))?;

                Ok(Ip::new(de))
            }

            fn visit_str<E>(self, v: &str) -> Result<Ip<TMapping>, E>
            where
                E: Error,
            {
                let de =
                    Ipv4Addr::from_str(v).map_err(|e| E::custom(e.description().to_string()))?;

                Ok(Ip::new(de))
            }
        }

        deserializer.deserialize_any(IpVisitor::<TMapping> { _m: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::net::Ipv4Addr;

    use crate::types::prelude::*;

    #[derive(Default)]
    struct MyIpMapping;
    impl IpMapping for MyIpMapping {}

    #[test]
    fn can_change_ip_mapping() {
        fn takes_custom_mapping(_: Ip<MyIpMapping>) -> bool {
            true
        }

        let ip: Ip<DefaultIpMapping> = Ip::new(Ipv4Addr::new(127, 0, 0, 1));

        assert!(takes_custom_mapping(Ip::remap(ip)));
    }

    #[test]
    fn serialise_elastic_ip() {
        let ip: Ip<DefaultIpMapping> = Ip::new(Ipv4Addr::new(127, 0, 0, 1));

        let ser = serde_json::to_string(&ip).unwrap();

        assert_eq!(r#""127.0.0.1""#, ser);
    }

    #[test]
    fn deserialise_elastic_ip() {
        let ip: Ip<DefaultIpMapping> = serde_json::from_str(r#""127.0.0.1""#).unwrap();

        assert_eq!(Ipv4Addr::new(127, 0, 0, 1), ip);
    }

}
