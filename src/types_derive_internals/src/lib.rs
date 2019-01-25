/*! 
Elasticsearch Core Types Codegen

This crate contains the internals for `elastic_types`-related codegen.
*/

#![recursion_limit = "128"]

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

pub mod elastic_type;
pub mod date_format;

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
        syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref key, ref lit)) if key == name => Some(lit),
        _ => None,
    }
}

fn expect_list<'a>(name: &str, meta_item: &'a syn::NestedMetaItem) -> Option<&'a [syn::NestedMetaItem]> {
    match *meta_item {
        syn::NestedMetaItem::MetaItem(syn::MetaItem::List(ref key, ref list)) if key == name => Some(list),
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
