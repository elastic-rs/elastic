use quote::Tokens;
use syn;
use serde_codegen_internals::{self, attr as serde_attr};
use super::get_elastic_meta_items;

pub fn expand_derive(crate_root: Tokens, input: &syn::MacroInput) -> Vec<Tokens> {
    //Annotatable item for a struct with struct fields
    let fields = match input.body {
        syn::Body::Struct(ref data) => {
            match *data {
                syn::VariantData::Struct(ref fields) => Some(fields),
                _ => None
            }
        },
        _ => None
    };

    if fields.is_none() {
        panic!("derive(ElasticType) is only valid for structs");
    }
    let fields = fields.unwrap();

    //Get the serializable fields
    let fields: Vec<(syn::Ident, &syn::Field)> = fields
        .iter()
        .map(|f| get_ser_field(f))
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect();

    let mut genned = Vec::new();
    
    let mapping_ty = {
        //If a user supplies a mapping with `#[elastic(mapping="")]`, then use it.
        //Otherwise, define the mapping struct and implement defaults for it.
        if let Some(mapping_ty) = get_mapping_from_attr(input) {
            mapping_ty
        }
        else {
            let mapping_ty = get_default_mapping(input);
            let es_ty = get_elastic_type_name(input);

            genned.push(define_mapping(&mapping_ty));
            genned.push(impl_object_mapping(crate_root.clone(), &mapping_ty, &es_ty));

            mapping_ty
        }
    };
    
    genned.push(impl_elastic_type(crate_root.clone(), input, &mapping_ty));
    genned.push(impl_props_mapping(crate_root.clone(), &mapping_ty, get_props_ser_stmts(crate_root.clone(), &fields)));

    genned
}

//Define a struct for the mapping with a few defaults
fn define_mapping(name: &syn::Ident) -> Tokens {
    quote!(
        #[derive(Default, Clone, Copy, Debug)]
        pub struct #name;
    )
}

//Implement DocumentType for the type being derived with the mapping
fn impl_elastic_type(crate_root: Tokens, item: &syn::MacroInput, mapping: &syn::Ident) -> Tokens {
    let ty = &item.ident;

    quote!(
        impl #crate_root::document::DocumentType<#mapping> for #ty { }
    )
}

//Implement DocumentMapping for the mapping
fn impl_object_mapping(crate_root: Tokens, mapping: &syn::Ident, es_ty: &syn::Lit) -> Tokens {
    quote!(
        impl #crate_root::document::DocumentMapping for #mapping {
            fn name() -> &'static str { #es_ty }
        }
    )
}

//Implement PropertiesMapping for the mapping
fn impl_props_mapping(crate_root: Tokens, mapping: &syn::Ident, prop_ser_stmts: Vec<Tokens>) -> Tokens {
    let stmts_len = prop_ser_stmts.len();
    let stmts = prop_ser_stmts;

    quote!(
        impl #crate_root::document::PropertiesMapping for #mapping {
            fn props_len() -> usize { #stmts_len }

            fn serialize_props<S>(state: &mut S) -> ::std::result::Result<(), S::Error> 
                where S: ::serde::ser::SerializeStruct {
                #(#stmts)*
                Ok(())
            }
        }
    )
}

//Get the serde serialisation statements for each of the fields on the type being derived
fn get_props_ser_stmts(crate_root: Tokens, fields: &[(syn::Ident, &syn::Field)]) -> Vec<Tokens> {
    let fields: Vec<Tokens> = fields.iter().cloned().map(|(name, field)| {
        let lit = syn::Lit::Str(name.as_ref().to_string(), syn::StrStyle::Cooked);
        let ty = &field.ty;

        let expr = quote!(#crate_root::field::mapping::<#ty, _, _>());

        quote!(try!(#crate_root::document::field_ser(state, #lit, #expr));)
    })
    .collect();

    fields
}

//Get the mapping ident supplied by an #[elastic()] attribute or create a default one
fn get_mapping_from_attr(item: &syn::MacroInput) -> Option<syn::Ident> {
    for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
        for meta_item in meta_items {
            match *meta_item {
                // Parse `#[elastic(mapping="foo")]`
                syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref name, ref lit)) if name == &"mapping" => {
                    return Some(
                        get_ident_from_lit(lit)
                            .unwrap_or(get_default_mapping(item))
                    )
                },
                _ => ()
            }
        }
    }

    None
}

//Get the default mapping name
fn get_default_mapping(item: &syn::MacroInput) -> syn::Ident {
    syn::Ident::from(format!("{}Mapping", item.ident))
}

fn get_ident_from_lit(lit: &syn::Lit) -> Result<syn::Ident, &'static str> {
    match *lit {
        syn::Lit::Str(ref s, _) => Ok(syn::Ident::from(s.as_str())),
        _ => {
            return Err("Unable to get str from lit");
        }
    }
}

//Get the default name for the indexed elasticsearch type name
fn get_elastic_type_name(item: &syn::MacroInput) -> syn::Lit {
    syn::Lit::Str(format!("{}", item.ident).to_lowercase(), syn::StrStyle::Cooked)
}

fn get_ser_field(field: &syn::Field) -> Option<(syn::Ident, &syn::Field)> {
    let ctxt = serde_codegen_internals::Ctxt::new();
    let serde_field = serde_attr::Field::from_ast(&ctxt, 0, field);

    match ctxt.check() {
        Err(e) => panic!(e),
        _ => ()
    };

    //Get all fields on struct where there isn't `skip_serializing`
    if serde_field.skip_serializing() {
        return None;
    }

    Some((syn::Ident::from(serde_field.name().serialize_name().as_ref()), field))
}