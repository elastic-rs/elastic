//! Json String Literal Generator
//!
//! Write json with Rust syntax instead of hard to read inline strings.
//! Results are converted to a `&'static str` at compile-time.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/json_str).
//! To get started, add `json_str` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! json_str = "*"
//! ```
//!
//! And reference it in your crate root:
//!
//! ```ignore
//! #![feature(plugin)]
//! #![plugin(json_str)]
//! ```
//!
//! The `json_str!` macro will take an inline token tree and return an `str` literal:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(json_str, elastic_types_macros)]
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
//! # #![plugin(json_str, elastic_types_macros)]
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
//! For json values that can't be fully determined at compile-time,
//! use [json_macros](https://github.com/tomjakubowski/json_macros) instead.

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/json_str/")]

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

use rustc_plugin::Registry;

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("json_str", json::expand_json);
}
