/*! Elasticsearch Codegen

Compile-time code generation for Elasticsearch type implementations.
This crate provides custom `derive` attributes for data types in the [`elastic`][github] crate.

[github]: https://github.com/elastic-rs/elastic
*/

extern crate proc_macro;

extern crate elastic_types_derive_internals as internals;
#[macro_use]
extern crate quote;
extern crate syn;

use internals::{
    date_format,
    elastic_type,
};

#[proc_macro_derive(ElasticType, attributes(elastic))]
pub fn derive_elastic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    match elastic_type::expand_derive(quote!(::elastic::types), &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        }
        Err(e) => panic!("{}", e),
    }
}

#[proc_macro_derive(ElasticDateFormat, attributes(elastic))]
pub fn derive_date_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    match date_format::expand_derive(quote!(::elastic::types), &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        }
        Err(e) => panic!("{}", e),
    }
}
