/*! Elasticsearch Codegen

Compile-time code generation for Elasticsearch type implementations.
This crate provides custom `derive` attributes for data types in the [`elastic`][github] crate.

[github]: https://github.com/elastic-rs/elastic
*/

#![recursion_limit = "128"]

extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

#[macro_use]
extern crate quick_error;

extern crate nom;

extern crate serde;
extern crate serde_derive_internals;
extern crate serde_json;

extern crate chrono;

use quote::TokenStreamExt;
use syn::{
    parse_macro_input,
    spanned::Spanned,
    Attribute,
    DeriveInput,
    Error as SynError,
    Expr,
    Ident,
    Lit,
    Meta,
    MetaList,
    MetaNameValue,
    NestedMeta,
};

mod date_format;
mod elastic_type;

#[proc_macro_derive(ElasticType, attributes(elastic))]
pub fn derive_elastic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = proc_macro2::TokenStream::new();
    let ast: DeriveInput = parse_macro_input!(input);
    let crate_root = get_crate_root(&ast).unwrap();

    match elastic_type::expand_derive(crate_root, &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        }
        Err(e) => panic!("{}", e),
    }
}

#[proc_macro_derive(ElasticDateFormat, attributes(elastic))]
pub fn derive_date_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = proc_macro2::TokenStream::new();
    let ast: DeriveInput = parse_macro_input!(input);
    let crate_root = get_crate_root(&ast).unwrap();

    match date_format::expand_derive(crate_root, &ast) {
        Ok(genned) => {
            expanded.append_all(genned);

            expanded.to_string().parse().unwrap()
        }
        Err(e) => panic!("{}", e),
    }
}

// Get the format string supplied by an #[elastic()] attribute
fn get_crate_root(item: &DeriveInput) -> Result<proc_macro2::TokenStream, SynError> {
    let val = get_elastic_meta_items(&item.attrs);

    let val = val
        .iter()
        .filter_map(|meta| expect_name_value("crate_root", &meta))
        .next();

    match val {
        Some(crate_root) => {
            let root = get_tokens_from_lit(&crate_root)?;
            Ok(quote!(#root))
        }
        None => Ok(quote!(elastic::types)),
    }
}

fn get_elastic_meta_items<'a, I>(attrs: I) -> Vec<NestedMeta>
where
    I: IntoIterator<Item = &'a Attribute> + 'a,
{
    attrs
        .into_iter()
        .filter_map(|attr| {
            attr.parse_meta().ok().and_then(|meta| match meta {
                Meta::List(MetaList {
                    ref path,
                    ref nested,
                    ..
                }) if path.get_ident() == Some(&quote::format_ident!("{}", "elastic")) => {
                    Some(nested.clone().into_iter())
                }
                _ => None,
            })
        })
        .flat_map(|list| list)
        .collect()
}

fn expect_name_value<'a>(name: &str, meta_item: &'a NestedMeta) -> Option<&'a Lit> {
    match *meta_item {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path, ref lit, ..
        })) if path.get_ident() == Some(&quote::format_ident!("{}", name)) => Some(lit),
        _ => None,
    }
}

fn expect_list<'a>(
    name: &str,
    meta_item: &'a NestedMeta,
) -> Option<impl Iterator<Item = &'a NestedMeta>> {
    match *meta_item {
        NestedMeta::Meta(Meta::List(MetaList {
            ref path,
            ref nested,
            ..
        })) if path.get_ident() == Some(&quote::format_ident!("{}", name)) => Some(nested.iter()),
        _ => None,
    }
}

fn expect_ident<'a>(name: &str, meta_item: &'a NestedMeta) -> bool {
    let name = quote::format_ident!("{}", name);
    match *meta_item {
        NestedMeta::Meta(Meta::Path(ref path)) if path.get_ident() == Some(&name) => true,
        _ => false,
    }
}

fn get_ident_from_lit(lit: &Lit) -> Result<Ident, &'static str> {
    get_string_from_lit(lit).map(|s| proc_macro2::Ident::new(&s, proc_macro2::Span::call_site()))
}

fn get_tokens_from_lit(lit: &Lit) -> Result<proc_macro2::TokenStream, SynError> {
    if let Lit::Str(ref s) = *lit {
        let toks = s.parse::<Expr>()?;
        Ok(quote!(#toks))
    } else {
        let err = SynError::new(lit.span(), "Unable to get TokenStream from Lit");
        Err(err)
    }
}

fn get_string_from_lit<'a>(lit: &'a Lit) -> Result<String, &'static str> {
    match *lit {
        Lit::Str(ref s) => Ok(s.value()),
        _ => Err("Unable to get String from Lit"),
    }
}
