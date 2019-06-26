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

#[macro_use]
extern crate nom;

extern crate serde;
extern crate serde_derive_internals;
extern crate serde_json;

extern crate chrono;

mod date_format;
mod elastic_type;

#[proc_macro_derive(ElasticType, attributes(elastic))]
pub fn derive_elastic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
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
    let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
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
fn get_crate_root<'a>(item: &'a syn::MacroInput) -> Result<quote::Tokens, String> {
    let val = get_elastic_meta_items(&item.attrs);

    let val = val
        .iter()
        .filter_map(|meta| expect_name_value("crate_root", &meta))
        .next();

    match val {
        Some(crate_root) => {
            let crate_root = get_str_from_lit(crate_root)?;

            let mut tokens = quote::Tokens::new();
            tokens.append(crate_root);

            Ok(tokens)
        }
        None => Ok(quote!(elastic::types)),
    }
}

fn get_elastic_meta_items<'a, I>(attrs: I) -> Vec<syn::NestedMetaItem>
where
    I: IntoIterator<Item = &'a syn::Attribute> + 'a,
{
    attrs
        .into_iter()
        .filter_map(|attr| match attr.value {
            syn::MetaItem::List(ref key, ref list) if key == "elastic" => Some(list),
            _ => None,
        })
        .flat_map(|list| list)
        .cloned()
        .collect()
}

fn expect_name_value<'a>(name: &str, meta_item: &'a syn::NestedMetaItem) -> Option<&'a syn::Lit> {
    match *meta_item {
        syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref key, ref lit))
            if key == name =>
        {
            Some(lit)
        }
        _ => None,
    }
}

fn expect_list<'a>(
    name: &str,
    meta_item: &'a syn::NestedMetaItem,
) -> Option<&'a [syn::NestedMetaItem]> {
    match *meta_item {
        syn::NestedMetaItem::MetaItem(syn::MetaItem::List(ref key, ref list)) if key == name => {
            Some(list)
        }
        _ => None,
    }
}

fn expect_ident<'a>(name: &str, meta_item: &'a syn::NestedMetaItem) -> bool {
    match *meta_item {
        syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) if ident == name => true,
        _ => false,
    }
}

fn get_ident_from_lit(lit: &syn::Lit) -> Result<syn::Ident, &'static str> {
    get_str_from_lit(lit).map(Into::into)
}

fn get_tokens_from_lit<'a>(lit: &'a syn::Lit) -> Result<quote::Tokens, &'static str> {
    get_str_from_lit(lit).map(|s| {
        let mut tokens = quote::Tokens::new();
        tokens.append(s);
        tokens
    })
}

fn get_str_from_lit<'a>(lit: &'a syn::Lit) -> Result<&'a str, &'static str> {
    match *lit {
        syn::Lit::Str(ref s, _) => Ok(s.as_str()),
        _ => {
            return Err("Unable to get str from lit");
        }
    }
}
