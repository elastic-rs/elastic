//TODO: Implement derive for elastic types


/*
mod mytype_mapping {
	use std::marker::PhantomData;
	use std::borrow::Cow;
	use serde;
	use serde::Serialize;
	use elastic_types::mapping::prelude::*;
	use super::MyType;

	impl <'a> ElasticType<MyTypeMapping<'a>, ()> for MyType { }

	#[derive(Default, Clone)]
	struct MyTypeMapping<'a> {
		phantom: PhantomData<&'a ()>
	}

	impl <'a> ElasticTypeMapping<()> for MyTypeMapping<'a> {
		type Visitor = MyTypeNestedMappingVisitor<'a>;

		fn data_type() -> &'static str {
			"nested"
		}
	}
	impl <'a> ElasticUserTypeMapping<'a, MyType> for MyTypeMapping<'a> {
		type Visitor = MyTypeMappingVisitor<'a>;
		type PropertiesVisitor = MyTypePropertiesVisitor<'a>;

		fn name() -> &'static str {
			"mytype"
		}
	}

	impl <'a> serde::Serialize for MyTypeMapping<'a> {
		fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer {
			serializer.serialize_struct(Self::name(), MyTypeNestedMappingVisitor::default())
		}
	}

	struct MyTypeNestedMappingVisitor<'a> { 
		data: Cow<'a, MyType>
	}

	impl <'a> MyTypeNestedMappingVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypeNestedMappingVisitor {
				data: Cow::Borrowed(data)
			}
		}
	}

	impl <'a> Default for MyTypeNestedMappingVisitor<'a> {
		fn default() -> MyTypeNestedMappingVisitor<'a> {
			MyTypeNestedMappingVisitor {
				data: Cow::Owned(MyType::default())
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypeNestedMappingVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt("type", MyTypeMapping::data_type()));
			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));

			Ok(None)
		}
	}

	struct MyTypeMappingVisitor<'a> { 
		data: &'a MyType
	}

	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypeMappingVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypeMappingVisitor {
				data: data
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, MyType, MyTypeMapping<'a>>::new(&self.data)));

			Ok(None)
		}
	}

	struct MyTypePropertiesVisitor<'a> {
		data: &'a MyType
	}

	impl <'a> ElasticUserTypeVisitor<'a, MyType> for MyTypePropertiesVisitor<'a> {
		fn new(data: &'a MyType) -> Self {
			MyTypePropertiesVisitor {
				data: data
			}
		}
	}

	impl <'a> serde::ser::MapVisitor for MyTypePropertiesVisitor<'a> {
		fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
		where S: serde::Serializer {
			try!(FieldMapper::map("my_date1", &self.data.my_date1, serializer));
			try!(FieldMapper::map("my_date2", &self.data.my_date2, serializer));
			try!(FieldMapper::map("my_string", &self.data.my_string, serializer));
			try!(FieldMapper::map("my_num", &self.data.my_num, serializer));

			Ok(None)
		}
	}
}
*/