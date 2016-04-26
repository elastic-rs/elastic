//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/elastic_macros).
//! To get started, add `elastic_macros` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_macros = "*"
//! ```
//!
//! And reference it in your crate root:
//!
//! ```ignore
//! #![feature(plugin)]
//! #![plugin(elastic_macros)]
//! ```
//!
//! ## Json Macros
//!
//! The `json_str!` macro will take an inline token tree and return an `str` literal:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let json = json_str!({
//! 	"query": {
//! 		"filtered": {
//! 			"query": {
//! 				"match_all": {}
//! 			},
//! 			"filter": {
//! 				"geo_distance": {
//! 					"distance": "20km",
//! 					"location": {
//! 						"lat": 37.776,
//! 						"lon": -122.41
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//!
//! This will also work for unquoted keys for something a bit more `rusty`:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let json = json_str!({
//! 	query: {
//! 		filtered: {
//! 			query: {
//! 				match_all: {}
//! 			},
//! 			filter: {
//! 				geo_distance: {
//! 					distance: "20km",
//! 					location: {
//! 						lat: 37.776,
//! 						lon: -122.41
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//!
//! For values that can't be determined at compile-time, use [json_macros](https://github.com/tomjakubowski/json_macros) instead.
//!
//! ## Types Macros
//!
//! There are also a couple of macros designed to work with `elastic_types`.
//! These are feature-gated, so you'll need to use the `types` feature in your `Cargo.toml`:
//!
//! ```norun
//! [dependencies.elastic_macros]
//! version = "*"
//! features = [ "types" ]
//! ```
//!
//! ### Date Formatting
//!
//! The `date_fmt!` macro will take a literal date format and parse it to a more efficient `Vec<Item>`.
//! This is used by date formatters.
//!
//! ```
//! # #![feature(plugin, types]
//! # #![plugin(elastic_macros)]
//! # extern crate chrono;
//! # fn main() {
//! let my_fmt = date_fmt!("yyyyMMddTHHmmss.SSSZ");
//! # }
//! ```
//!
//! This also works for `chrono` date formats:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # extern crate chrono;
//! # fn main() {
//! let my_fmt = date_fmt!("%Y%m%dT%H%M%S%.3fZ");
//! # }
//! ```
//!
//! # Links
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_macros/")]

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote, plugin, stmt_expr_attributes)]
#![plugin(serde_macros)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate serde;
extern crate serde_json;

#[doc(hidden)]
pub mod parse;
#[doc(hidden)]
pub mod json;

#[doc(hidden)]
#[cfg(feature = "types")]
#[macro_use]
pub mod types;

use rustc_plugin::Registry;

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("json_str", json::expand_json);

	#[cfg(feature = "types")]
	{
		reg.register_macro("date_fmt", types::expand_date_fmt);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticType"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_type_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticStringMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_string_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticBooleanMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_boolean_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticIntegerMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_integer_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticLongMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_long_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticShortMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_short_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticByteMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_byte_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticDoubleMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_double_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticFloatMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_float_mapping))
		);

		reg.register_syntax_extension(
	        syntax::parse::token::intern("derive_ElasticDateMapping"),
	        syntax::ext::base::MultiDecorator(
	            Box::new(types::expand_derive_date_mapping))
		);
	}
}
