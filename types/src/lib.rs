//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//! 
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
//! 
//! # Examples
//! 
//! Derive `ElasticType` on your Elasticsearch-mappable types:
//! 
//! ```
//! //TODO: Implement this
//! ```
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![feature(custom_derive, custom_attribute, plugin, optin_builtin_traits, associated_type_defaults)]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![plugin(serde_macros, elastic_macros)]

//TODO: Move these to elastic_macros
#![macro_use]
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
macro_rules! impl_properties {
    ($t:ty) => (
    	impl <'a> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("properties", self.get_visitor())
			}
		}
    )
}

extern crate chrono;
extern crate serde;

pub mod mapping;
pub mod mappers;

pub mod user_type;
pub mod date;
pub mod string;

macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticType<$crate::mapping::NullMapping, ()> for $t { }
		)*
	)
}

impl_mapping!(
	bool,
	isize,
	i8,
	i16,
	i32,
	i64,
	usize,
	u8,
	u16,
	u32,
	u64,
	f32,
	f64,
	char
);

impl <T: serde::Serialize> mapping::ElasticType<mapping::NullMapping, ()> for Vec<T> { }
impl <'a, T: serde::Serialize> mapping::ElasticType<mapping::NullMapping, ()> for &'a [T] { }