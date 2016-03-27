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

#[macro_export]
macro_rules! impl_type_mapping {
    ($t:ident, $es_ty:expr, [$($arg:ident),*]) => (impl_type_mapping!($t, inner, $es_ty, [$($arg),*]););
    ($t:ident, $mod_name:ident, $es_ty:expr, [$($arg:ident),*]) => (
    	mod $mod_name {
			use std::marker::PhantomData;
			use std::borrow::Cow;
			use serde;
			use serde::Serialize;
			use $crate::mapping::prelude::*;
			use super::$t;

			impl <'a> ElasticType<MyMapping<'a>, ()> for $t { }

			#[derive(Default, Clone)]
			struct MyMapping<'a> {
				phantom: PhantomData<&'a ()>
			}

			impl <'a> ElasticTypeMapping<()> for MyMapping<'a> {
				type Visitor = MyNestedMappingVisitor<'a>;

				fn data_type() -> &'static str {
					"nested"
				}
			}
			impl <'a> ElasticUserTypeMapping<'a, $t> for MyMapping<'a> {
				type Visitor = MyMappingVisitor<'a>;
				type PropertiesVisitor = MyPropertiesVisitor<'a>;

				fn name() -> &'static str {
					"mytype"
				}
			}

			impl <'a> serde::Serialize for MyMapping<'a> {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: serde::Serializer {
					serializer.serialize_struct(Self::name(), MyNestedMappingVisitor::default())
				}
			}

			struct MyNestedMappingVisitor<'a> { 
				data: Cow<'a, $t>
			}

			impl <'a> MyNestedMappingVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					MyNestedMappingVisitor {
						data: Cow::Borrowed(data)
					}
				}
			}

			impl <'a> Default for MyNestedMappingVisitor<'a> {
				fn default() -> MyNestedMappingVisitor<'a> {
					MyNestedMappingVisitor {
						data: Cow::Owned($t::default())
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for MyNestedMappingVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					try!(serializer.serialize_struct_elt("type", MyMapping::data_type()));
					try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, $t, MyMapping<'a>>::new(&self.data)));

					Ok(None)
				}
			}

			struct MyMappingVisitor<'a> { 
				data: &'a $t
			}

			impl <'a> ElasticUserTypeVisitor<'a, $t> for MyMappingVisitor<'a> {
				fn new(data: &'a $t) -> Self {
					MyMappingVisitor {
						data: data
					}
				}
			}

			impl <'a> serde::ser::MapVisitor for MyMappingVisitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					try!(serializer.serialize_struct_elt("properties", ElasticUserTypeProperties::<'a, $t, MyMapping<'a>>::new(&self.data)));

					Ok(None)
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
		}
    )
}

//impl_type_mapping(MyType, [my_date1, my_date2, my_string, my_num])