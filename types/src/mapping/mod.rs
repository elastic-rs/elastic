//! Base requirements for type mappings.
//!
//! There are two kinds of types we can map in Elasticsearch; `field`/`data` types and `user-defined` types.
//! Either kind of type must implement `ElasticFieldType`, which captures the mapping and possible formatting
//! requirements as generic parameters.
//! Most of the work lives in the `ElasticFieldMapping`, which holds the serialisation requirements
//! to convert a Rust type into an Elasticsearch mapping.
//! User-defined types must also implement `ObjectMapping`, which maps the fields of a struct as properties,
//! and treats the type as `nested` when used as a field itself.
//!
//! # Links
//! - [Field Types](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping-types.html)
//! - [User-defined Types](https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping.html)

mod field;
mod object;

pub use self::object::*;
pub use self::field::*;

pub mod prelude {
    //! Includes mapping types for all data types.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use ::mappers::*;
    pub use super::object::*;
    pub use super::field::*;
    pub use ::date::mapping::*;
    pub use ::ip::mapping::*;
    pub use ::geo::mapping::*;
    pub use ::string::mapping::*;
    pub use ::number::mapping::*;
    pub use ::boolean::mapping::*;
}