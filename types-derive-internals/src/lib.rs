#[macro_use]
extern crate quote;
extern crate syn;

extern crate serde;
extern crate serde_json;
extern crate serde_codegen_internals;

pub mod elastic_type;
pub mod date_format;

fn get_elastic_meta_items(attr: &syn::Attribute) -> Option<&[syn::NestedMetaItem]> {
    match attr.value {
        //Get elastic meta items
        syn::MetaItem::List(ref name, ref items) if name == &"elastic" => {
            Some(items)
        },
        _ => None
    }
}