//! Requirements for mapping user-defined types.
//! 
//! # Examples
//! 
//! Define your Elasticsearch types using _Plain Old Rust Structures_. 
//! Your types should at least derive `Default`, `Clone` and `serde::Serialize`.
//! 
//! ## Derive Mapping
//! 
//! _TODO: Fill this in_
//! 
//! ## Mapping with Macros
//! 
//! The `impl_type_mapping` macro can be used to hide a lot of the boilerplate in serialising the mapping:
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::ser::Serialize;
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::{ DateTime, EpochMillis };
//! use elastic_types::string::ElasticString;
//! 
//! #[derive(Default, Clone, Serialize)]
//! pub struct MyType {
//! 	//Some mappable fields
//! 	pub my_date1: DateTime,
//! 	pub my_date2: DateTime<EpochMillis>,
//! 	pub my_string: ElasticString<DefaultStringMapping>,
//! 	pub my_num: i32,
//! 	//Some un-mapped field
//! 	pub my_unmapped: Vec<u8>
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! impl_type_mapping!(MyType, "my_type", [my_date1, my_date2, my_string, my_num]);
//! # fn main() {
//! # }
//! ```
//! 
//! The fields passed to `impl_type_mapping` must all implement `ElasticType`. 
//! Note that not all fields need to be included in the macro call.
//! Also remember that Elasticsearch will automatically update mappings based on the objects it sees though,
//! so if your 'un-mapped' field is serialised on `index`, then some mapping will be added for it.
//! 
//! Because `MyType` now implements `ElasticType`, it too can be used in another type, and will be mapped as a [nested object](https://www.elastic.co/guide/en/elasticsearch/guide/current/nested-objects.html):
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::ser::Serialize;
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::{ DateTime, EpochMillis };
//! # use elastic_types::string::ElasticString;
//! # #[derive(Default, Clone, Serialize)]
//! # pub struct MyType {
//! # 	//Some mappable fields
//! # 	pub my_date1: DateTime,
//! # 	pub my_date2: DateTime<EpochMillis>,
//! # 	pub my_string: ElasticString<DefaultStringMapping>,
//! # 	pub my_num: i32,
//! # 	//Some un-mapped field
//! # 	pub my_unmapped: Vec<u8>
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl_type_mapping!(MyType, inner1, "my_type", [my_date1, my_date2, my_string, my_num]);
//! #[derive(Default, Clone, Serialize)]
//! pub struct MyOtherType {
//! 	pub my_type: MyType
//! }
//! # impl serde::Serialize for MyOtherType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! impl_type_mapping!(MyOtherType, "my_other_type", [my_type]);
//! # fn main() {
//! # }
//! ```
//! 
//! ## Mapping manually
//! 
//! You can also implement Elasticsearch type mapping manually to avoid using macros, 
//! or for more control over how your types will be mapped. 
//! The macro method is probably preferrable though because of the amount of boilerplate involved.
//! 
//! Take the following complete example:
//! 
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(elastic_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::ser::Serialize;
//! use elastic_types::mapping::prelude::*;
//! use elastic_types::date::{ DateTime, EpochMillis };
//! use elastic_types::string::ElasticString;
//! #[derive(Default, Clone, Serialize)]
//! struct MyType {
//! 	pub my_date1: DateTime,
//! 	pub my_date2: DateTime<EpochMillis>,
//! 	pub my_string: ElasticString<DefaultStringMapping>,
//! 	pub my_num: i32
//! }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! # 	where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! 
//! mod mytype_mapping {
//! 	use std::marker::PhantomData;
//! 	use std::borrow::Cow;
//! 	use serde;
//! 	use serde::Serialize;
//! 	use elastic_types::mapping::prelude::*;
//! 	use super::MyType;
//! 
//! 	impl <'a> ElasticType<MyTypeMapping<'a>, ()> for MyType { }
//! 
//! 	#[derive(Default, Clone)]
//! 	struct MyTypeMapping<'a> {
//! 		phantom: PhantomData<&'a ()>
//! 	}
//! 
//! 	impl <'a> ElasticTypeMapping<()> for MyTypeMapping<'a> {
//! 		type Visitor = MyTypeNestedMappingVisitor<'a>;
//! 
//! 		fn data_type() -> &'static str {
//! 			"nested"
//! 		}
//! 	}
//! 	impl <'a> ElasticUserTypeMapping<'a, MyType> for MyTypeMapping<'a> {
//! 		type Visitor = MyTypeMappingVisitor<'a>;
//! 		type PropertiesVisitor = MyTypePropertiesVisitor<'a>;
//! 
//! 		fn name() -> &'static str {
//! 			"mytype"
//! 		}
//! 	}
//! 
//! 	impl <'a> serde::Serialize for MyTypeMapping<'a> {
//! 		fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! 		where S: serde::Serializer {
//! 			serializer.serialize_struct(Self::name(), MyTypeNestedMappingVisitor::default())
//! 		}
//! 	}
//! 
//! 	struct MyTypeNestedMappingVisitor<'a> { 
//! 		data: Cow<'a, MyType>
//! 	}
//! 
//! 	impl <'a> MyTypeNestedMappingVisitor<'a> {
//! 		fn new(data: &'a MyType) -> Self {
//! 			MyTypeNestedMappingVisitor {
//! 				data: Cow::Borrowed(data)
//! 			}
//! 		}
//! 	}
//! 
//! 	impl <'a> Default for MyTypeNestedMappingVisitor<'a> {
//! 		fn default() -> MyTypeNestedMappingVisitor<'a> {
//! 			MyTypeNestedMappingVisitor {
//! 				data: Cow::Owned(MyType::default())
//! 			}
//! 		}
//! 	}
//! 
//! 	impl <'a> serde::ser::MapVisitor for MyTypeNestedMappingVisitor<'a> {
//! 		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 		where S: serde::Serializer {
//! 			try!(serializer.serialize_struct_elt("type", MyTypeMapping::data_type()));
//! 			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));
//! 
//! 			Ok(None)
//! 		}
//! 	}
//! 
//! 	struct MyTypeMappingVisitor<'a> { 
//! 		data: &'a MyType
//! 	}
//! 
//! 	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypeMappingVisitor<'a> {
//! 		fn new(data: &'a MyType) -> Self {
//! 			MyTypeMappingVisitor {
//! 				data: data
//! 			}
//! 		}
//! 	}
//! 
//! 	impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
//! 		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 		where S: serde::Serializer {
//! 			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));
//! 
//! 			Ok(None)
//! 		}
//! 	}
//! 
//! 	struct MyTypePropertiesVisitor<'a> {
//! 		data: &'a MyType
//! 	}
//! 
//! 	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypePropertiesVisitor<'a> {
//! 		fn new(data: &'a MyType) -> Self {
//! 			MyTypePropertiesVisitor {
//! 				data: data
//! 			}
//! 		}
//! 	}
//! 
//! 	impl <'a> serde::ser::MapVisitor for MyTypePropertiesVisitor<'a> {
//! 		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 		where S: serde::Serializer {
//! 			try!(FieldMapper::map("my_date1", &self.data.my_date1, serializer));
//! 			try!(FieldMapper::map("my_date2", &self.data.my_date2, serializer));
//! 			try!(FieldMapper::map("my_string", &self.data.my_string, serializer));
//! 			try!(FieldMapper::map("my_num", &self.data.my_num, serializer));
//! 
//! 			Ok(None)
//! 		}
//! 	}
//! }
//! # fn main() {
//! # }
//! ```
//! 
//! First, we need to implement `ElasticType`, which takes the mapping as a generic parameter:
//! 
//! ```ignore
//! impl <'a> ElasticType<MyTypeMapping<'a>, ()> for MyType { }
//! ```
//! 
//! The mapping struct is pretty simple. It takes an explicit lifetime parameter though so we can pass around an instance
//! of our `MyType` and avoid allocations:
//!
//! ```ignore
//! #[derive(Default, Clone)]
//! struct MyTypeMapping<'a> {
//! 	phantom: PhantomData<&'a ()>
//! }
//! ```
//! 
//! When mapping our type as a field, we implement `ElasticTypeMapping`:
//! 
//! ```ignore
//! impl <'a> ElasticTypeMapping<()> for MyTypeMapping<'a> {
//! 	type Visitor = MyTypeNestedMappingVisitor<'a>;
//! 
//! 	fn data_type() -> &'static str {
//! 		"nested"
//! 	}
//! }
//! 
//! impl <'a> serde::Serialize for MyTypeMapping<'a> {
//! 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//! 	where S: serde::Serializer {
//! 		serializer.serialize_struct(Self::name(), MyTypeNestedMappingVisitor::default())
//! 	}
//! }
//! ```
//! 
//! The serialisation follows the standard visitor pattern in serde. 
//! Again we use an explicit lifetime so it's possible to build our visitor with either a default owned `MyType`
//! or a borrowed instance:
//! 
//! ```ignore
//! struct MyTypeNestedMappingVisitor<'a> { 
//! 	data: Cow<'a, MyType>
//! }
//! 
//! impl <'a> MyTypeNestedMappingVisitor<'a> {
//! 	fn new(data: &'a MyType) -> Self {
//! 		MyTypeNestedMappingVisitor {
//! 			data: Cow::Borrowed(data)
//! 		}
//! 	}
//! }
//! 
//! impl <'a> Default for MyTypeNestedMappingVisitor<'a> {
//! 	fn default() -> MyTypeNestedMappingVisitor<'a> {
//! 		MyTypeNestedMappingVisitor {
//! 			data: Cow::Owned(MyType::default())
//! 		}
//! 	}
//! }
//! 
//! impl <'a> serde::ser::MapVisitor for MyTypeNestedMappingVisitor<'a> {
//! 	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 	where S: serde::Serializer {
//! 		try!(serializer.serialize_struct_elt("type", MyTypeMapping::data_type()));
//! 		try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));
//! 
//! 		Ok(None)
//! 	}
//! }
//! ```
//! 
//! The actual mapping of the properties is offloaded to yet another visitor:
//! 
//! ```ignore
//! struct MyTypePropertiesVisitor<'a> {
//! 	data: &'a MyType
//! }
//! 
//! impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypePropertiesVisitor<'a> {
//! 	fn new(data: &'a MyType) -> Self {
//! 		MyTypePropertiesVisitor {
//! 			data: data
//! 		}
//! 	}
//! }
//! 
//! impl <'a> serde::ser::MapVisitor for MyTypePropertiesVisitor<'a> {
//! 	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 	where S: serde::Serializer {
//! 		try!(FieldMapper::map("my_date1", &self.data.my_date1, serializer));
//! 		try!(FieldMapper::map("my_date2", &self.data.my_date2, serializer));
//! 		try!(FieldMapper::map("my_string", &self.data.my_string, serializer));
//! 		try!(FieldMapper::map("my_num", &self.data.my_num, serializer));
//! 
//! 		Ok(None)
//! 	}
//! }		
//! ```
//! 
//! The reason we do this is so the serialisation can be reused by the mapping when we map as a standalone type,
//! rather than as a field:
//! 
//! ```ignore
//! impl <'a> ElasticUserTypeMapping<'a, MyType> for MyTypeMapping<'a> {
//! 	type Visitor = MyTypeMappingVisitor<'a>;
//! 	type PropertiesVisitor = MyTypePropertiesVisitor<'a>;
//! 
//! 	fn name() -> &'static str {
//! 		"mytype"
//! 	}
//! }
//! ```
//! 
//! Note we take two visitors for our standalone mapping; the `MyTypePropertiesVisitor` defined already,
//! and an alternative to the `MyTypeNestedMappingVisitor`. 
//! The difference is that we don't need to specify the `type` field as `nested`:
//! 
//! ```ignore
//! struct MyTypeMappingVisitor<'a> { 
//! 	data: &'a MyType
//! }
//! 
//! impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypeMappingVisitor<'a> {
//! 	fn new(data: &'a MyType) -> Self {
//! 		MyTypeMappingVisitor {
//! 			data: data
//! 		}
//! 	}
//! }
//! 
//! impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
//! 	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//! 	where S: serde::Serializer {
//! 		try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));
//! 
//! 		Ok(None)
//! 	}
//! }
//! ```
//! 
//! Although there is a _lot_ of code involved, it's all very similar across implementations.
//! All that really changed is the implementation for `MyTypeNestedMappingVisitor`, where fields are mapped.

use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use ::mapping::{ ElasticTypeMapping };

/// The base requirements for mapping a user-defined type.
pub trait ElasticUserTypeMapping<'a, T: 'a + Clone + Default>
where Self: ElasticTypeMapping<()> + Default + Clone + serde::Serialize {
	#[doc(hidden)]
	type Visitor: ElasticUserTypeVisitor<'a, T>;
	#[doc(hidden)]
	type PropertiesVisitor: ElasticUserTypeVisitor<'a, T>;

	/// The name of the user-defined type used in Elasticsearch.
	fn name() -> &'static str;
}

/// Visitor for the `ElasticUserTypeProperties` struct and given user-defined type.
/// 
/// The purpose of this trait is to serialise the mapping for each datatype on the user-defined type `T`.
/// To make this easier, the `FieldMapper` can be used to infer the mapping for a field that implements `ElasticType`.
/// 
/// # Examples
/// 
/// Implement `ElasticUserTypeVisitor` for a user-defined type:
/// 
/// ```
/// //TODO: Implement
/// ```
pub trait ElasticUserTypeVisitor<'a, T: 'a + Clone + Default>
where Self: serde::ser::MapVisitor { 
	/// Create a new visitor from a borrowed user-defined type.
	fn new(data: &'a T) -> Self;
}

pub trait ElasticObjectMapping
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
    fn data_type() -> ObjectType {
        ObjectType::Nested
    }

    fn dynamic() -> Option<bool> {
    	None
    }
}

pub enum ObjectType {
    Object,
    Nested
}

impl ObjectType {
	pub fn as_str(&self) -> &str {
		match *self {
			ObjectType::Object => "object",
			ObjectType::Nested => "nested"
		}
	}
}

//TODO: Figure out how to work this in to the serialisation
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

		Ok(None)
	}
}

//TODO: Rename ElasticObjectProperties?
/// Represents the properties object that encapsulates type mappings.
pub struct ElasticUserTypeProperties<'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> { 
	data: &'a T,
	phantom: PhantomData<M>
}
impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> ElasticUserTypeProperties<'a, T, M> {
	/// Create a new properties struct from a borrowed user-defined type.
	pub fn new(data: &'a T) -> Self {
		ElasticUserTypeProperties {
			data: data,
			phantom: PhantomData
		}
	}
}

impl <'a, T: 'a + Clone + Default, M: ElasticUserTypeMapping<'a, T>> serde::Serialize for ElasticUserTypeProperties<'a, T, M> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", <M as ElasticUserTypeMapping<T>>::PropertiesVisitor::new(&self.data))
	}
}