use std::marker::PhantomData;
use serde;
use serde::Serializer;
use ::mapping::{ ElasticTypeMapping, ElasticTypeVisitor };

/// The base requirements for mapping a user-defined type.
///
/// User-defined type mappings are tied to `object` mappings.
pub trait ElasticUserTypeMapping where
Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticTypeVisitor;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticTypeVisitor;
}

/// Represents the properties object that encapsulates type mappings.
#[derive(Clone)]
pub struct ElasticTypeProperties<T, M> where
M: ElasticUserTypeMapping {
	phantom_t: PhantomData<T>,
	phantom_m: PhantomData<M>
}

impl <T, M> ElasticTypeProperties<T, M> where
M: ElasticUserTypeMapping {
	/// Create a new type properties container.
	pub fn new() -> Self {
		ElasticTypeProperties {
			phantom_t: PhantomData,
			phantom_m: PhantomData
		}
	}
}

impl <T, M> serde::Serialize for ElasticTypeProperties<T, M> where
M: ElasticUserTypeMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping>::PropertiesVisitor::new())
	}
}
