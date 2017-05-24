use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::error::Error as StdError;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::mapping::{IpFieldType, IpMapping, DefaultIpMapping};

impl IpFieldType<DefaultIpMapping> for Ipv4Addr {}

/// An Elasticsearch `ip` with a mapping.
///
/// Where the mapping isn't custom, you can use the standard library `Ipv4Addr` instead.
///
/// # Examples
///
/// Defining an `ip` with a mapping:
///
/// ```
/// use std::net::Ipv4Addr;
/// use elastic_types::ip::mapping::DefaultIpMapping;
/// use elastic_types::ip::Ip;
///
/// let ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Ip<M>
    where M: IpMapping
{
    value: Ipv4Addr,
    _m: PhantomData<M>,
}
impl<M> Ip<M>
    where M: IpMapping
{
    /// Creates a new `Ip` with the given mapping.
    ///
    /// # Examples
    ///
    /// Create a new `Ip` from a `Ip4vAddr`:
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use elastic_types::ip::mapping::DefaultIpMapping;
    /// use elastic_types::ip::Ip;
    ///
    /// let ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
    /// ```
    pub fn new<I>(ip: I) -> Ip<M>
        where I: Into<Ipv4Addr>
    {
        Ip {
            value: ip.into(),
            _m: PhantomData,
        }
    }

    /// Change the mapping of this ip.
    ///
    /// # Examples
    ///
    /// Change the mapping for a given `Ip`:
    ///
    /// ```
    /// # extern crate serde;
    /// # #[macro_use]
    /// # extern crate elastic_types;
    /// # fn main() {
    /// # use std::net::Ipv4Addr;
    /// # use elastic_types::prelude::*;
    /// # #[derive(Default)]
    /// # struct MyIpMapping;
    /// # impl IpMapping for MyIpMapping { }
    /// let es_ip = Ip::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
    ///
    /// let ip: Ip<MyIpMapping> = es_ip.remap();
    /// # }
    /// ```
    pub fn remap<MInto>(self) -> Ip<MInto>
        where MInto: IpMapping
    {
        Ip::<MInto>::new(self.value)
    }
}

impl<M> IpFieldType<M> for Ip<M> where M: IpMapping {}

impl_mapping_type!(Ipv4Addr, Ip, IpMapping);

// Serialize elastic ip
impl<M> Serialize for Ip<M>
    where M: IpMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.value.to_string())
    }
}

// Deserialize elastic ip
impl<'de, M> Deserialize<'de> for Ip<M>
    where M: IpMapping
{
    fn deserialize<D>(deserializer: D) -> Result<Ip<M>, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Default)]
        struct IpVisitor<M>
            where M: IpMapping
        {
            _m: PhantomData<M>,
        }

        impl<'de, M> Visitor<'de> for IpVisitor<M>
            where M: IpMapping
        {
            type Value = Ip<M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string containing an IpV4 address")
            }

            fn visit_string<E>(self, v: String) -> Result<Ip<M>, E>
                where E: Error
            {
                let de = try!(Ipv4Addr::from_str(&v).map_err(|e| E::custom(e.description().to_string())));

                Ok(Ip::<M>::new(de))
            }

            fn visit_str<E>(self, v: &str) -> Result<Ip<M>, E>
                where E: Error
            {
                let de = try!(Ipv4Addr::from_str(v).map_err(|e| E::custom(e.description().to_string())));

                Ok(Ip::<M>::new(de))
            }
        }

        deserializer.deserialize_any(IpVisitor::<M>::default())
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::net::Ipv4Addr;

    use prelude::*;

    #[derive(Default)]
    struct MyIpMapping;
    impl IpMapping for MyIpMapping {}

    #[test]
    fn can_change_ip_mapping() {
        fn takes_custom_mapping(_: Ip<MyIpMapping>) -> bool {
            true
        }

        let ip: Ip<DefaultIpMapping> = Ip::new(Ipv4Addr::new(127, 0, 0, 1));

        assert!(takes_custom_mapping(ip.remap()));
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
