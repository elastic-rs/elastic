use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::error::Error as StdError;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use serde::de::{ Visitor, Error };
use super::mapping::{ IpMapping, DefaultIpMapping, IpFormat };
use ::mapping::ElasticType;

impl ElasticType<DefaultIpMapping, IpFormat> for Ipv4Addr { }

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
pub struct Ip<M> where
M: IpMapping {
	value: Ipv4Addr,
	_m: PhantomData<M>
}
impl <M> Ip<M> where
M: IpMapping {
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
	pub fn new<I>(ip: I) -> Ip<M> where I: Into<Ipv4Addr> {
		Ip {
			value: ip.into(),
			_m: PhantomData
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
	pub fn remap<MInto>(self) -> Ip<MInto> where
	MInto: IpMapping {
		Ip::<MInto>::new(self.value)
	}
}

impl <M> ElasticType<M, IpFormat> for Ip<M> where
M: IpMapping { }

impl From<Ipv4Addr> for Ip<DefaultIpMapping> {
	fn from(ip: Ipv4Addr) -> Self {
		Ip::new(ip)
	}
}

impl<'a, M> PartialEq<Ipv4Addr> for Ip<M> where
M: IpMapping {
	fn eq(&self, other: &Ipv4Addr) -> bool {
		PartialEq::eq(&self.value, other)
	}

	fn ne(&self, other: &Ipv4Addr) -> bool {
		PartialEq::ne(&self.value, other)
	}
}

impl<'a, M> PartialEq<Ip<M>> for Ipv4Addr where
M: IpMapping {
	fn eq(&self, other: &Ip<M>) -> bool {
		PartialEq::eq(self, &other.value)
	}

	fn ne(&self, other: &Ip<M>) -> bool {
		PartialEq::ne(self, &other.value)
	}
}

//Serialize elastic ip
impl <M> Serialize for Ip<M> where
M: IpMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_str(&self.value.to_string())
	}
}

//Deserialize elastic ip
impl <M> Deserialize for Ip<M> where
M: IpMapping {
	fn deserialize<D>(deserializer: &mut D) -> Result<Ip<M>, D::Error> where
	D: Deserializer {
		#[derive(Default)]
		struct IpVisitor<M> where
		M: IpMapping {
			_m: PhantomData<M>
		}

		impl <M> Visitor for IpVisitor<M> where
		M: IpMapping {
			type Value = Ip<M>;

			fn visit_string<E>(&mut self, v: String) -> Result<Ip<M>, E> where
			E: Error {
				let de = try!(Ipv4Addr::from_str(&v).map_err(|e| E::custom(e.description().to_string())));

				Ok(Ip::<M>::new(de))
			}

            fn visit_str<E>(&mut self, v: &str) -> Result<Ip<M>, E> where
			E: Error {
				let de = try!(Ipv4Addr::from_str(v).map_err(|e| E::custom(e.description().to_string())));

				Ok(Ip::<M>::new(de))
			}
		}

		deserializer.deserialize(IpVisitor::<M>::default())
	}
}
