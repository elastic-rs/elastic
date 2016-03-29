//! Implementation helper macros.

#[macro_export]
macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticType<$crate::mapping::NullMapping, ()> for $t { }
		)*
	)
}

#[macro_export]
macro_rules! impl_string_mapping {
    ($t:ty) => (
    	impl $crate::mapping::ElasticTypeMapping<()> for $t {
			type Visitor = $crate::string::mapping::ElasticStringMappingVisitor<$t>;

			fn data_type() -> &'static str {
				"string"
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
    )
}

#[macro_export]
macro_rules! impl_date_mapping {
	($t:ty, $f:ty) => (
    	impl $crate::mapping::ElasticTypeMapping<$f> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<$f, $t>;

			fn data_type() -> &'static str {
				"date"
			}
		}
		
		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
    );
    ($t:ty) => (
    	impl <T: $crate::date::DateFormat> $crate::mapping::ElasticTypeMapping<T> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<T, $t>;

			fn data_type() -> &'static str {
				"date"
			}
		}

		impl <T: $crate::date::DateFormat> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
    )
}

#[macro_export]
macro_rules! impl_date_fmt {
    ($t:ty, $f:tt, $n:expr) => (
		impl $crate::date::DateFormat for $t {
			fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
				date_fmt!($f)
					.iter()
					.cloned()
					.collect()
			}

			fn name() -> &'static str { $n }
		}
    )
}

//TODO: See if we can get away without explicit lifetime
//TODO: See if we can specify the type name here
/*
#[derive(Default, Clone)]
struct MyMapping;
impl ElasticObjectMapping for MyMapping {
	fn data_type() -> ObjectType {
        ObjectType::Nested;
    }
}
*/

#[macro_export]
macro_rules! impl_object_mapping {
    ($t:ident, $m:ident, $es_ty:expr, [$($arg:ident),*]) => (impl_object_mapping!($t, $m, inner, $es_ty, [$($arg),*]););
    ($t:ident, $m:ident, $mod_name:ident, $es_ty:expr, [$($arg:ident),*]) => (
    	mod $mod_name {
			use std::marker::PhantomData;
			use std::borrow::Cow;
			use serde;
			use serde::Serialize;
			use $crate::mapping::prelude::*;
			use super::{ $t, $m };

			impl ElasticType<$m, ()> for $t { }

			impl <'a> ElasticTypeMapping<'a, ()> for $m {
				type Visitor = MyFieldMappingVisitor<'a>;

				fn data_type() -> &'static str {
					<Self as ElasticObjectMapping>::data_type().as_str()
				}
			}
			impl <'a> ElasticUserTypeMapping<'a, $t> for $m {
				type Visitor = MyTypeMappingVisitor<'a>;
				type PropertiesVisitor = MyPropertiesVisitor<'a>;

				fn name() -> &'static str {
					$es_ty
				}
			}

			impl <'a> serde::Serialize for $m {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: serde::Serializer {
					serializer.serialize_struct(Self::name(), MyFieldMappingVisitor::default())
				}
			}

			struct MyPropertiesVisitor<'a> {
				data: &'a $t
			}

			impl <'a> ElasticUserTypeVisitor<'a, $t> for MyPropertiesVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					MyPropertiesVisitor {
						data: data
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for MyPropertiesVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					$(
						try!(FieldMapper::map(stringify!($arg), &self.data.$arg, serializer));
					)*

					Ok(None)
				}
			}

			struct MyFieldMappingVisitor<'a> { 
				data: Cow<'a, $t>
			}

			impl <'a> Default for MyFieldMappingVisitor<'a> {
				fn default() -> MyFieldMappingVisitor<'a> {
					MyFieldMappingVisitor {
						data: Cow::Owned($t::default())
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for MyFieldMappingVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					try!(serializer.serialize_struct_elt("type", <$m as ElasticTypeMapping<()>>::data_type()));

					let mut object_mapper = ElasticObjectMappingVisitor::<$m>::default();
					try!(object_mapper.visit(&mut serializer));

					try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, $t, $m>::new(&self.data)));

					Ok(None)
				}
			}

			struct MyTypeMappingVisitor<'a> { 
				data: &'a $t
			}

			impl <'a> ElasticUserTypeVisitor<'a, $t> for MyTypeMappingVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					MyTypeMappingVisitor {
						data: data
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					let mut object_mapper = ElasticObjectMappingVisitor::<$m>::default();
					try!(object_mapper.visit(&mut serializer));

					try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, $t, $m>::new(&self.data)));

					Ok(None)
				}
			}
		}
    )
}