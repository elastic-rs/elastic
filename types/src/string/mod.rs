//! Implementation of the Elasticsearch `keyword` and `text` types.
//!
//! Keyword fields are stored as a raw string of tokens, and aren't analysed when querying.
//! They're useful for data that only has meaning when considered as a whole, like an id
//! or single word.
//! 
//! Text fields are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! They're useful for blobs of content that can be sliced in various ways, like prose.
//!
//! As far as serialisation is concerned, `keyword` and `text` are equivalent.
//! 
//! # Examples
//!
//! For defining your own string mapping, see: 
//! 
//! - [keyword mapping details](mapping/trait.KeywordMapping.html#derive-mapping)
//! - [text mapping details](mapping/trait.TextMapping.html#derive-mapping).
//!
//! Map with a default `string` (follows the [semantics](CHECK ME) for legacy `string` mapping:
//!
//! ```
//! struct MyType {
//! 	pub field: String
//! }
//! ```
//!
//! Map a `keyword`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::string::prelude::*;
//! struct MyType {
//! 	pub field: Keyword<DefaultKeywordMapping>
//! }
//! # }
//! ```
//! 
//! Map `text`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::string::prelude::*;
//! struct MyType {
//! 	pub field: Text<DefaultTextMapping>
//! }
//! # }
//! ```
//! 
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

macro_rules! impl_string_type {
    ($t:ident, $m:ident, $f:ident, $d:ident) => (
    	impl <M> ElasticType<M, $f> for $t<M> where
		M: $m { }

		impl From<String> for $t<$d> {
			fn from(string: String) -> Self {
				$t::new(string)
			}
		}

		impl <M> AsRef<str> for $t<M> where
		M: $m {
			fn as_ref(&self) -> &str {
				&self.value
			}
		}

		impl<'a, M> PartialEq<String> for $t<M> where
		M: $m {
			fn eq(&self, other: &String) -> bool {
				PartialEq::eq(&self.value, other)
			}

			fn ne(&self, other: &String) -> bool {
				PartialEq::ne(&self.value, other)
			}
		}

		impl<'a, M> PartialEq<$t<M>> for String where
		M: $m {
			fn eq(&self, other: &$t<M>) -> bool {
				PartialEq::eq(self, &other.value)
			}

			fn ne(&self, other: &$t<M>) -> bool {
				PartialEq::ne(self, &other.value)
			}
		}

		impl<'a, M> PartialEq<&'a str> for $t<M> where
		M: $m {
			fn eq(&self, other: & &'a str) -> bool {
				PartialEq::eq(&self.value, *other)
			}

			fn ne(&self, other: & &'a str) -> bool {
				PartialEq::ne(&self.value, *other)
			}
		}

		impl<'a, M> PartialEq<$t<M>> for &'a str where
		M: $m {
			fn eq(&self, other: &$t<M>) -> bool {
				PartialEq::eq(*self, &other.value)
			}

			fn ne(&self, other: &$t<M>) -> bool {
				PartialEq::ne(*self, &other.value)
			}
		}

		impl <M> Serialize for $t<M> where
		M: $m {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
			S: Serializer {
				serializer.serialize_str(&self.value)
			}
		}

		impl <M> Deserialize for $t<M> where
		M: $m {
			fn deserialize<D>(deserializer: &mut D) -> Result<$t<M>, D::Error> where
			D: Deserializer {
				#[derive(Default)]
				struct StringVisitor<M> where
				M: $m {
					_m: PhantomData<M>
				}

				impl <M> Visitor for StringVisitor<M> where
				M: $m {
					type Value = $t<M>;

					fn visit_str<E>(&mut self, v: &str) -> Result<$t<M>, E> where
					E: Error {
						Ok($t::<M>::new(v))
					}
				}

				deserializer.deserialize(StringVisitor::<M>::default())
			}
		}
    );
}

#[macro_use]
pub mod keyword;
#[macro_use]
pub mod text;

pub mod mapping;

pub use self::keyword::Keyword;
pub use self::text::Text;


pub mod prelude {
	//! Includes non-mapping types for the `string` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::keyword::Keyword;
	pub use super::text::Text;
}