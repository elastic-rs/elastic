use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use super::ElasticObjectTypeVisitor;
use ::mapping::{ ElasticTypeMapping };

pub trait ElasticObjectMapping
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
	fn data_type() -> &'static str {
		"nested"
	}

	fn dynamic() -> Option<bool> {
		None
	}

	fn enabled() -> Option<bool> {
		None
	}

	fn include_in_all() -> Option<bool> {
		None
	}
}

/// Represents the properties object that encapsulates type mappings.
pub struct ElasticObjectProperties<'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<V>
}
impl <'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> ElasticObjectProperties<'a, T, V> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticObjectProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, V: ElasticObjectTypeVisitor<'a, T>> serde::Serialize for ElasticObjectProperties<'a, T, V> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", V::new(&self.data))
	}
}

#[derive(Debug, PartialEq, Default)]
pub struct ElasticObjectMappingVisitor<T: ElasticObjectMapping> {
	phantom: PhantomData<T>
}

impl <T: ElasticObjectMapping> serde::ser::MapVisitor for ElasticObjectMappingVisitor<T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		if let Some(dynamic) = T::dynamic() {
			try!(serializer.serialize_struct_elt("dynamic", dynamic));
		};

		if let Some(enabled) = T::enabled() {
			try!(serializer.serialize_struct_elt("enabled", enabled));
		};

		if let Some(include_in_all) = T::include_in_all() {
			try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
		};

		Ok(None)
	}
}