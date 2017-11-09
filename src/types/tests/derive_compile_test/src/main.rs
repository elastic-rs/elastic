/*!
Test crate to ensure derive macros can be used in a fresh crate without any extra dependencies.
*/

extern crate elastic_types;
#[macro_use]
extern crate elastic_types_derive;

#[derive(ElasticDateFormat, PartialEq, Debug, Default, Clone, Copy)]
#[elastic(date_format = "yyyy-MM-dd'T'HH:mm:ssZ")]
pub struct DerivedDateFormat;

#[derive(ElasticType)]
pub struct DerivedDocument {
    pub field1: String,
    pub field2: i32,
}

fn main() {}
