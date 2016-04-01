use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use super::ElasticObjectTypeVisitor;
use ::mapping::{ ElasticTypeMapping };

/// The base requirements for mapping an `object` type.
/// 
/// Object mappings are tied to user-defined `type` mappings.
pub trait ElasticObjectMapping
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
	/// Get the type name for this mapping, like `object` or `nested`.
	fn data_type() -> &'static str {
		"object"
	}

	/// Whether or not new properties should be added dynamically to an existing object. Accepts `true` (default), `false` and `strict`.
	fn dynamic() -> Option<Dynamic> {
		None
	}

	/// Whether the JSON value given for the object field should be parsed and indexed (`true`, default) or completely ignored (`false`).
	fn enabled() -> Option<bool> {
		None
	}

	/// Sets the default `include_in_all` value for all the properties within the object. The object itself is not added to the `_all` field.
	fn include_in_all() -> Option<bool> {
		None
	}
}

/// The dynamic setting may be set at the mapping type level, and on each inner object. Inner objects inherit the setting from their parent object or from the mapping type.
#[derive(Debug, Clone, Copy)]
pub enum Dynamic {
	/// Newly detected fields are added to the mapping. (default).
	True,
	/// Newly detected fields are ignored. New fields must be added explicitly.
	False,
	/// If new fields are detected, an exception is thrown and the document is rejected.
	Strict
}

impl serde::Serialize for Dynamic {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		match *self {
			Dynamic::True => serializer.serialize_bool(true),
			Dynamic::False => serializer.serialize_bool(false),
			Dynamic::Strict => serializer.serialize_str("strict")
		}
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

/// Visitor for an `object` field mapping.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticObjectMappingVisitor<T: ElasticObjectMapping> {
	phantom: PhantomData<T>
}

impl <T: ElasticObjectMapping> serde::ser::MapVisitor for ElasticObjectMappingVisitor<T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", <T as ElasticTypeMapping<()>>::data_type()));

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