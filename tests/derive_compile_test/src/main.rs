/*!
Test crate to ensure derive macros can be used in a fresh crate without any extra dependencies.
*/

#[macro_use]
extern crate elastic_derive;

#[derive(ElasticDateFormat, PartialEq, Debug, Default, Clone, Copy)]
#[elastic(date_format = "yyyy-MM-dd'T'HH:mm:ssZ")]
pub struct DerivedDateFormat;

#[derive(ElasticType)]
pub struct DerivedDocument1 {
    pub field1: String,
    pub field2: i32,
}

#[derive(ElasticType)]
#[elastic(index(expr = "self.index()"), ty = "_doc")]
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
pub struct DerivedDocument2U32 {
    #[elastic(id(expr = "field1.to_string()"))]
    pub field1: i32,
    pub field2: i32,
}

#[derive(ElasticType)]
#[elastic(index = "derived_documents", id(expr = "self.id()"))]
struct DerivedDocument3 {
    pub field1: String,
    pub field2: i32,
}

impl DerivedDocument3 {
    fn id(&self) -> &str {
        &self.field1
    }
}

fn main() {}
