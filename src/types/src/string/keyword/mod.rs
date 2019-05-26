/*!
Implementation of the Elasticsearch `keyword` type.

Keyword fields are stored as a raw string of tokens, and aren't analysed when querying.
They're useful for data that only has meaning when considered as a whole, like an id
or single word.
*/

#[macro_use]
pub mod mapping;

mod impls;
pub use self::impls::*;

pub mod prelude {
    /*!
    Includes all types for the `keyword` type.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        impls::*,
        mapping::*,
    };
}
