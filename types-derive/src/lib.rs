/*! 
Elasticsearch Core Types Codegen

 Compile-time code generation for Elasticsearch type implementations.
 This crate provides custom `derive` attributes for data types in the [elastic_types](https://github.com/elastic-rs/elastic-types) crate.

 # Links
 - [Github](https://github.com/elastic-rs/elastic-types)
 !*/

extern crate proc_macro;

extern crate syn;
#[macro_use]
extern crate quote;
extern crate elastic_types_derive_internals as internals;

use internals::{elastic_type, date_format};

#[proc_macro_derive(ElasticType, attributes(elastic))]
pub fn derive_elastic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    match elastic_type::expand_derive(quote!(::elastic_types), &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        },
        Err(e) => panic!("{}", e)
    }
}

#[proc_macro_derive(ElasticDateFormat, attributes(elastic))]
pub fn derive_date_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    match date_format::expand_derive(quote!(::elastic_types), &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        },
        Err(e) => panic!("{}", e)
    }
}
