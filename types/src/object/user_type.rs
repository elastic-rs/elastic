use std::marker::PhantomData;
use serde;
use serde::Serializer;
use super::ElasticObjectProperties;
use ::mapping::{ ElasticTypeVisitor };

/// The base requirements for mapping a user-defined type.
///
/// User-defined type mappings are tied to `object` mappings.
pub trait ElasticUserTypeMapping where
Self: super::ElasticObjectMapping {
	#[doc(hidden)]
	type Visitor : ElasticTypeVisitor;
}

/// Visitor for an `object` type mapping when mapping as a user-defined type in an Elasticsearch index.
#[derive(Debug, PartialEq)]
pub struct ElasticUserTypeMappingVisitor<V> where
V: ElasticTypeVisitor {
	phantom: PhantomData<V>
}

impl <V> ElasticTypeVisitor for ElasticUserTypeMappingVisitor<V> where
V: ElasticTypeVisitor {
	fn new() -> Self {
		ElasticUserTypeMappingVisitor {
			phantom: PhantomData
		}
	}
}

impl <V> serde::ser::MapVisitor for ElasticUserTypeMappingVisitor<V> where
V: ElasticTypeVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("properties", ElasticObjectProperties::<V>::new()));

		Ok(None)
	}
}
