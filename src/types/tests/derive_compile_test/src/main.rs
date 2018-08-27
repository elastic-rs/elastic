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
pub struct DerivedDocument1 {
    pub field1: String,
    pub field2: i32,
}

#[derive(ElasticType)]
#[elastic(index(expr = "DerivedDocument2::index"), ty = "doc")]
pub struct DerivedDocument2 {
    #[elastic(id)]
    pub field1: String,
    pub field2: i32,
}

impl DerivedDocument2 {
    fn index(&self) -> String {
        format!("docs-{}", self.field2)
    }
}

#[derive(ElasticType)]
#[elastic(index = "derived_documents", id(expr = "DerivedDocument3::id"))]
pub struct DerivedDocument3 {
    pub field1: String,
    pub field2: i32,
}

impl DerivedDocument3 {
    fn id(&self) -> &str {
        &self.field1
    }
}

fn main() {}
