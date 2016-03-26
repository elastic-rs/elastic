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

extern crate chrono;
extern crate serde;

#[macro_use]
pub mod macros;

pub mod mapping;
pub mod mappers;

pub mod user_type;
pub mod date;
pub mod string;

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