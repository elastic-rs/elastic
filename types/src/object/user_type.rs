use std::marker::PhantomData;
use serde;
use serde::{ Serializer };
use super::ElasticObjectTypeVisitor;
use ::mapping::{ ElasticTypeMapping };

/// The base requirements for mapping a user-defined type.
pub trait ElasticUserTypeMapping<'a, T: 'a + Clone + Default>
where Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticObjectTypeVisitor<'a, T>;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticObjectTypeVisitor<'a, T>;

	/// The name of the user-defined type used in Elasticsearch.
	fn name() -> &'static str;
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticTypeProperties<'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<M>
}
impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> ElasticTypeProperties<'a, T, M> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticTypeProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> serde::Serialize for ElasticTypeProperties<'a, T, M> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping<T>>::PropertiesVisitor::new(&self.data))
	}
}