use std::marker::PhantomData;
use serde;
use serde::Serializer;
use super::ElasticObjectTypeVisitor;
use ::mapping::ElasticTypeMapping;

/// The base requirements for mapping a user-defined type.
///
/// User-defined type mappings are tied to `object` mappings.
pub trait ElasticUserTypeMapping<'a, T> where
T: 'a + Clone + Default,
Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticObjectTypeVisitor<'a, T>;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticObjectTypeVisitor<'a, T>;

	/// The name of the user-defined type used in Elasticsearch.
	fn name() -> &'static str;
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticTypeProperties<'a, T, M> where
T: 'a + Clone + Default,
M: ElasticUserTypeMapping<'a, T> {
	data: &'a T,
	phantom: PhantomData<M>
}
impl <'a, T, M> ElasticTypeProperties<'a, T, M> where
T: 'a + Clone + Default,
M: ElasticUserTypeMapping<'a, T> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticTypeProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T, M> serde::Serialize for ElasticTypeProperties<'a, T, M> where
T: 'a + Clone + Default,
M: ElasticUserTypeMapping<'a, T> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping<T>>::PropertiesVisitor::new(&self.data))
	}
}
