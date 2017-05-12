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
//! - [keyword mapping details](keyword/mapping/trait.KeywordMapping.html#derive-mapping)
//! - [text mapping details](text/mapping/trait.TextMapping.html#derive-mapping).
//!
//! Map with a default `string` (follows the semantics for legacy `string` mapping):
//!
//! ```
//! struct MyType {
//!     pub field: String
//! }
//! ```
//!
//! Map a `keyword`:
//!
//! ```
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # use elastic_types::string::prelude::*;
//! struct MyType {
//!     pub field: Keyword<DefaultKeywordMapping>
//! }
//! # }
//! ```
//!
//! Map `text`:
//!
//! ```
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # use elastic_types::string::prelude::*;
//! struct MyType {
//!     pub field: Text<DefaultTextMapping>
//! }
//! # }
//! ```
//! 
//! Map a custom type as a `keyword` field.
//! This is especially useful for simple `enum`s:
//! 
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! #[derive(Serialize)]
//! #[serde(rename_all = "lowercase")]
//! enum MyKeywordField {
//!     VariantA,
//!     VariantB,
//!     VariantC,
//! }
//! 
//! impl KeywordFieldType<DefaultKeywordMapping> for MyKeywordField {}
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

#[macro_use]
mod macros;

pub mod keyword;
pub mod text;

pub mod mapping;

pub use self::keyword::Keyword;
pub use self::text::Text;

pub mod prelude {
    //! Includes all types for the `string` types.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::keyword::prelude::*;
    pub use super::text::prelude::*;
    pub use super::mapping::*;
}
