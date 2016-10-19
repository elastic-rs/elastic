//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//! This crate provides custom `derive` attributes for data types in the [elastic_types](https://github.com/elastic-rs/elastic-types) crate.
//!
//! # Links
//! - [Github](https://github.com/elastic-rs/elastic-types)

#![feature(proc_macro, proc_macro_lib)]
#![crate_type = "proc-macro"]
#![cfg(not(test))]

extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate post_expansion;

extern crate serde;
extern crate serde_json;
extern crate serde_codegen_internals;

use proc_macro::TokenStream;
use serde_codegen_internals::attr as serde_attr;

register_post_expansion!(PostExpansion_elastic_types);

#[proc_macro_derive(ElasticType)]
pub fn derive(input: TokenStream) -> TokenStream {
	let mut expanded = quote::Tokens::new();
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let genned = expand_derive_type_mapping(&ast);

    let ast = post_expansion::strip_attrs_later(ast, &["elastic"], "elastic_types");
    expanded.append(&quote!(#ast).to_string());

    expanded.append_all(genned);

    expanded.to_string().parse().unwrap()
}

fn expand_derive_type_mapping(input: &syn::MacroInput) -> Vec<quote::Tokens> {
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
			genned.push(impl_object_mapping(&mapping_ty, &es_ty));

			mapping_ty
		}
	};
	
	genned.push(impl_elastic_type(input, &mapping_ty));
	genned.push(impl_props_mapping(&name, get_props_ser_stmts(fields)));

	genned
}

//Define a struct for the mapping with a few defaults
fn define_mapping(name: &syn::Ident) -> quote::Tokens {
	quote!(
		#[derive(Default, Clone, Copy, Debug)]
		pub struct #name;
	)
}

//Implement ElasticType for the type being derived with the mapping
fn impl_elastic_type(item: &syn::MacroInput, mapping: &syn::Ident) -> quote::Tokens {
	let ty = &item.ident;

	quote!(
		impl ::elastic_types::mapping::ElasticType<#mapping> for #ty { }
	)
}

//Implement ObjectMapping for the mapping
fn impl_object_mapping(mapping: &syn::Ident, es_ty: &syn::Lit) -> quote::Tokens {
	quote!(
		impl ::elastic_types::object::ObjectMapping for #mapping {
			fn name() -> &'static str { #es_ty }
		}
	)
}

//Implement PropertiesMapping for the mapping
fn impl_props_mapping(mapping: &syn::Ident, prop_ser_stmts: Vec<quote::Tokens>) -> quote::Tokens {
	let stmts_len = prop_ser_stmts.len();
	let stmts = prop_ser_stmts;

	quote!(
		impl ::elastic_types::object::PropertiesMapping for #mapping {
			fn props_len() -> usize { #stmts_len }
			
			fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				#(#stmts)*
				Ok(())
			}
		}
	)
}

//Get the serde serialisation statements for each of the fields on the type being derived
fn get_props_ser_stmts(fields: &[(syn::Ident, &syn::Field)]) -> Vec<quote::Tokens> {
	let fields: Vec<quote::Tokens> = fields.iter().cloned().map(|(name, field)| {
		let lit = syn::Lit::Str(name.as_ref().to_string(), syn::StrStyle::Cooked);
		let ty = match field.ty {
			//Standard type path
			syn::Ty::Path(_, ref p) => Some(p),
			//Borrowed type
			syn::Ty::Rptr(_, ref t) => {
				match t.ty {
					syn::Ty::Path(_, ref p) => Some(p),
					_ => None
				}
			},
			_ => None
		};

		if let Some(ty) = ty {
			let mut segments = Vec::new();

			for seg in &ty.segments {
				let id = &seg.ident;
				segments.push(quote!(#id));

				if seg.parameters != syn::PathParameters::none() {
					match seg.parameters {
						syn::PathParameters::AngleBracketed(ref p) => segments.push(quote!(#p)),
						_ => ()
					}
				}
			}

			let expr = quote!(#(#segments)::*::mapping_ser());

			Some(quote!(try!(serializer.serialize_struct_elt(state, #lit, #expr));))
		}
		else {
			None
		}
	})
	.filter_map(|stmt| stmt)
	.collect();

	fields
}

//Get the mapping ident supplied by an #[elastic()] attribute or create a default one
fn get_mapping_from_attr(item: &syn::MacroInput) -> Option<syn::Ident> {
	for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
		for meta_item in meta_items {
			match *meta_item {
				// Parse `#[elastic(mapping="foo")]`
				syn::MetaItem::NameValue(ref name, ref lit) if name == &"mapping" => {
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

//Helpers
fn get_elastic_meta_items(attr: &syn::Attribute) -> Option<&[syn::MetaItem]> {
	match attr.value {
		//Get elastic meta items
		syn::MetaItem::List(ref name, ref items) if name == &"elastic" => {
			Some(items)
		},
		_ => None
	}
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