#[macro_use]
extern crate quote;
extern crate syn;

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate nom;

extern crate serde;
extern crate serde_json;
extern crate serde_codegen_internals;

extern crate chrono;

pub mod elastic_type;
pub mod date_format;

fn get_elastic_meta_items(attr: &syn::Attribute) -> Option<&[syn::NestedMetaItem]> {
    match attr.value {
        // Get elastic meta items
        syn::MetaItem::List(ref name, ref items) if name == &"elastic" => Some(items),
        _ => None,
    }
}

// Get the mapping ident supplied by an #[elastic()] attribute or create a default one
fn get_elastic_attr_name_value<'a>(name: &str, item: &'a syn::MacroInput) -> Option<&'a syn::Lit> {
    for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
        for meta_item in meta_items {
            match *meta_item {
                // Parse `#[elastic({name}="foo")]`
                syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref key, ref lit))
                    if key == name => {
                    return Some(lit);
                }
                _ => (),
            }
        }
    }

    None
}

fn get_ident_from_lit(lit: &syn::Lit) -> Result<syn::Ident, &'static str> {
    get_str_from_lit(lit).map(|s| syn::Ident::from(s))
}

fn get_str_from_lit<'a>(lit: &'a syn::Lit) -> Result<&'a str, &'static str> {
    match *lit {
        syn::Lit::Str(ref s, _) => Ok(s.as_str()),
        _ => {
            return Err("Unable to get str from lit");
        }
    }
}
