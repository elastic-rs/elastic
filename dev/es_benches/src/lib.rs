#![feature(plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types_codegen;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
