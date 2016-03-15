//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//! 
//! # Json Macros
//! 
//! The `json!` macro will take an inline token tree and serialise it as json:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let json = json!({
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
//! let json = json!({
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
//! Json values can be spliced in to the result if they implement `serde::Serialize`:
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(elastic_macros)]
//! # fn main() {
//! let query = "match_all";
//! let dist = "20km";
//! let lat = 37.776;
//! let lon = -122.41;
//! 
//! let json = json!(query, dist, lat, lon {
//! 	query: {
//! 		filtered: {
//! 			query: {
//! 				$query: {}
//! 			},
//! 			filter: {
//! 				geo_distance: {
//! 					distance: $dist,
//! 					location: {
//! 						lat: $lat,
//! 						lon: $lon
//! 					}
//! 				}
//! 			}
//! 		}
//! 	}
//! });
//! # }
//! ```
//! 
//! # Types Macros
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
//! ## Date Formatting
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
pub mod types;

use rustc_plugin::Registry;

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("json", json::expand_json);

	#[cfg(feature = "types")]
	reg.register_macro("date_fmt", types::expand_date_fmt);
}