use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::error::Error;
use serde;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticIpMapping, DefaultIpMapping };
use ::mapping::{ ElasticFieldMapping, ElasticType };

impl ElasticType<DefaultIpMapping, ()> for Ipv4Addr { }

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
/// use elastic_types::ip::ElasticIp;
///
/// let ip = ElasticIp::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	value: Ipv4Addr,
	phantom: PhantomData<M>
}
impl <M> ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	/// Creates a new `ElasticIp` with the given mapping.
	///
	/// # Examples
	///
	/// Create a new `ElasticIp` from a `Ip4vAddr`:
	///
	/// ```
    /// use std::net::Ipv4Addr;
	/// use elastic_types::ip::mapping::DefaultIpMapping;
	/// use elastic_types::ip::ElasticIp;
	///
	/// let ip = ElasticIp::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
	/// ```
	pub fn new<I>(ip: I) -> ElasticIp<M> where I: Into<Ipv4Addr> {
		ElasticIp {
			value: ip.into(),
			phantom: PhantomData
		}
	}

	/// Get the value of the ip.
	pub fn get(&self) -> &Ipv4Addr {
		&self.value
	}

	/// Set the value of the ip.
	pub fn set<I>(&mut self, ip: I) where I: Into<Ipv4Addr> {
		self.value = ip.into()
	}

	/// Change the mapping of this ip.
	///
	/// # Examples
	///
	/// Change the mapping for a given `ElasticIp`:
	///
	/// ```
	/// # extern crate serde;
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # fn main() {
    /// # use std::net::Ipv4Addr;
	/// # use elastic_types::prelude::*;
	/// # ip_mapping!(MyIpMapping {});
	/// let es_ip = ElasticIp::<DefaultIpMapping>::new(Ipv4Addr::new(127, 0, 0, 1));
	///
	/// let ip: ElasticIp<MyIpMapping> = es_ip.remap();
	/// # }
	/// ```
	pub fn remap<MInto>(self) -> ElasticIp<MInto> where
	MInto: ElasticFieldMapping<()> + ElasticIpMapping {
		ElasticIp::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, ()> for ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping { }

impl From<Ipv4Addr> for ElasticIp<DefaultIpMapping> {
	fn from(ip: Ipv4Addr) -> Self {
		ElasticIp::new(ip)
	}
}

impl<'a, M> PartialEq<Ipv4Addr> for ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	fn eq(&self, other: &Ipv4Addr) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &Ipv4Addr) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<ElasticIp<M>> for Ipv4Addr where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	fn eq(&self, other: &ElasticIp<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &ElasticIp<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

//Serialize elastic ip
impl <M> Serialize for ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value.to_string())
	}
}

//Deserialize elastic ip
impl <M> Deserialize for ElasticIp<M> where
M: ElasticFieldMapping<()> + ElasticIpMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticIp<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct ElasticIpVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticIpMapping {
			phantom: PhantomData<M>
		}

		impl <M> serde::de::Visitor for ElasticIpVisitor<M> where
		M: ElasticFieldMapping<()> + ElasticIpMapping {
			type Value = ElasticIp<M>;

			fn visit_string<E>(&mut self, v: String) -> Result<ElasticIp<M>, E> where
			E: serde::de::Error {
				let de = try!(Ipv4Addr::from_str(&v).map_err(|e| E::custom(e.description().to_string())));

				Ok(ElasticIp::<M>::new(de))
			}

            fn visit_str<E>(&mut self, v: &str) -> Result<ElasticIp<M>, E> where
			E: serde::de::Error {
				let de = try!(Ipv4Addr::from_str(v).map_err(|e| E::custom(e.description().to_string())));

				Ok(ElasticIp::<M>::new(de))
			}
		}

		deserializer.deserialize(ElasticIpVisitor::<M>::default())
	}
}
