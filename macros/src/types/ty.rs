//TODO: Implement derive for elastic types

//Use stmts
/*
use std::marker::PhantomData;
use std::borrow::Cow;
use serde;
use serde::Serialize;
use elastic_types::mapping::DataMapper;
use elastic_types::mapping::prelude::*;
use super::$ty;
*/

//Implement ElasticDataType
/*
impl <'a> ElasticDataType<MyTypeMapping<'a>, ()> for $ty { }
*/

//Define ElasticMapping/TypeMapping
/*
#[derive(Default, Clone)]
struct $tymapping<'a> {
	phantom: PhantomData<&'a ()>
}

impl <'a> ElasticMapping<()> for $tymapping<'a> {
	type Visitor = MyTypeMappingVisitor<'a>;

	fn data_type() -> &'static str {
		$tyname
	}
}

impl <'a> TypeMapping<'a, $ty> for $tymapping<'a> {
	type Visitor = $tymappingvisitor<'a>;
}

impl <'a> Serialize for $tymapping<'a> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct($tyname, $tymappingvisitor::default())
	}
}
*/

//Define ElasticMapping visitor
/*
struct $tymappingvisitor<'a> { 
	data: Cow<'a, $ty>
}

impl <'a> TypeMappingVisitor<'a, MyType> for $tymappingvisitor<'a> {
	fn new(data: &'a $ty) -> Self {
		$tymappingvisitor {
			data: Cow::Borrowed(data)
		}
	}
}

impl <'a> Default for $tymappingvisitor<'a> {
	fn default() -> $tymappingvisitor<'a> {
		$tymappingvisitor {
			data: Cow::Owned($ty::default())
		}
	}
}

impl <'a> serde::ser::MapVisitor for $tymappingvisitor<'a> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("properties", $typrops::new(&self.data)));

		Ok(None)
	}
}
*/

//Define property mapping
/*
struct $typrops<'a> { 
	data: Cow<'a, $ty>
}
impl <'a> $typrops<'a> {
	fn new(data: &'a $ty) -> Self {
		MyTypeProperties {
			data: Cow::Borrowed(data)
		}
	}
}

impl <'a> serde::Serialize for $typrops<'a> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("properties", $typropsvisitor::new(&self.data))
	}
}
*/

//Property mapping visitor
/*
struct $typropsvisitor<'a> { 
	data: Cow<'a, $ty>
}
impl <'a> $typropsvisitor<'a> {
	fn new(data: &'a $ty) -> Self {
		$typropsvisitor {
			data: Cow::Borrowed(data)
		}
	}
}

impl <'a> serde::ser::MapVisitor for $typropsvisitor<'a> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		//for $prop in $ty::props
		try!(DataMapper::map($propname, &self.data.$prop, serializer));
		//end for

		Ok(None)
	}
}
*/