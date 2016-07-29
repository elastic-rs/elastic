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
//! - [keyword mapping details](mapping/trait.ElasticKeywordMapping.html#derive-mapping)
//! - [text mapping details](mapping/trait.ElasticTextMapping.html#derive-mapping).
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
//! 	pub field: ElasticKeyword<DefaultKeywordMapping>
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
//! 	pub field: ElasticText<DefaultTextMapping>
//! }
//! # }
//! ```
//! 
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

macro_rules! impl_string_type {
    ($t:ident, $m:ident, $d:ident) => (
    	impl <M> ElasticType<M, ()> for $t<M> where
		M: ElasticFieldMapping<()> + $m { }

		impl From<String> for $t<$d> {
			fn from(string: String) -> Self {
				$t::new(string)
			}
		}

		impl <M> AsRef<str> for $t<M> where
		M: ElasticFieldMapping<()> + $m {
			fn as_ref(&self) -> &str {
				&self.value
			}
		}

		impl<'a, M> PartialEq<String> for $t<M> where
		M: ElasticFieldMapping<()> + $m {
			fn eq(&self, other: &String) -> bool {
				PartialEq::eq(&self.value, other)
			}

			fn ne(&self, other: &String) -> bool {
				PartialEq::ne(&self.value, other)
			}
		}

		impl<'a, M> PartialEq<$t<M>> for String where
		M: ElasticFieldMapping<()> + $m {
			fn eq(&self, other: &$t<M>) -> bool {
				PartialEq::eq(self, &other.value)
			}

			fn ne(&self, other: &$t<M>) -> bool {
				PartialEq::ne(self, &other.value)
			}
		}

		impl<'a, M> PartialEq<&'a str> for $t<M> where
		M: ElasticFieldMapping<()> + $m {
			fn eq(&self, other: & &'a str) -> bool {
				PartialEq::eq(&self.value, *other)
			}

			fn ne(&self, other: & &'a str) -> bool {
				PartialEq::ne(&self.value, *other)
			}
		}

		impl<'a, M> PartialEq<$t<M>> for &'a str where
		M: ElasticFieldMapping<()> + $m {
			fn eq(&self, other: &$t<M>) -> bool {
				PartialEq::eq(*self, &other.value)
			}

			fn ne(&self, other: &$t<M>) -> bool {
				PartialEq::ne(*self, &other.value)
			}
		}

		impl <M> Serialize for $t<M> where
		M: ElasticFieldMapping<()> + $m {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
			S: Serializer {
				serializer.serialize_str(&self.value)
			}
		}

		impl <M> Deserialize for $t<M> where
		M: ElasticFieldMapping<()> + $m {
			fn deserialize<D>(deserializer: &mut D) -> Result<$t<M>, D::Error> where
			D: Deserializer {
				#[derive(Default)]
				struct StringVisitor<M> where
				M: ElasticFieldMapping<()> + $m {
					phantom: PhantomData<M>
				}

				impl <M> serde::de::Visitor for StringVisitor<M> where
				M: ElasticFieldMapping<()> + $m {
					type Value = $t<M>;

					fn visit_str<E>(&mut self, v: &str) -> Result<$t<M>, E> where
					E: serde::de::Error {
						Ok($t::<M>::new(v))
					}
				}

				deserializer.deserialize(StringVisitor::<M>::default())
			}
		}
    );
}

macro_rules! ser_sub_field {
	($s:ident, $f:expr, $n:expr) => (
		if let Some(f) = $f {
			try!($s.serialize_struct_elt($n, f));
		}
	)
}

pub mod mapping;

impl ::mapping::ElasticType<mapping::DefaultStringMapping, ()> for String { }

mod keyword;
mod text;

pub use self::keyword::ElasticKeyword;
pub use self::text::ElasticText;


pub mod prelude {
	//! Includes non-mapping types for the `string` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::keyword::ElasticKeyword;
	pub use super::text::ElasticText;
}