//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//! This crate provides custom `derive` attributes for data types in the [elastic_types](https://github.com/elastic-rs/elastic-types) crate.
//!
//! # Links
//! - [Github](https://github.com/elastic-rs/elastic-types)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types_macros/")]

#![feature(proc_macro, proc_macro_lib)]

extern crate proc_macro;

extern crate syn;
#[macro_use]
extern crate quote;

extern crate serde;
extern crate serde_json;
extern crate serde_codegen_internals;

use proc_macro::TokenStream;
use serde_codegen_internals::attr as serde_attr;

#[proc_macro_derive(ElasticType)]
pub fn derive_type_mapping(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();

    // Match the AST with an iteam we can annotate
    // Expand the derives and impls
    // Write back to a TokenStream

    expanded.to_string().parse().unwrap()
}

#[doc(hidden)]
fn expand_derive_type_mapping(input: &syn::MacroInput) {
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
		panic!("proper error for non struct derives");
	}
	let fields = fields.unwrap();

	//Get the serializable fields
	let fields: Vec<(syn::Ident, syn::Field)> = fields
		.iter()
		.map(|f| get_ser_field(f))
		.filter(|f| f.is_some())
		.map(|f| f.unwrap())
		.collect();

	build_mapping(input, &fields);
}

//Build a field mapping type and return the name
fn build_mapping(input: &syn::MacroInput, fields: &[(syn::Ident, syn::Field)]) {
	let name = {
		//If a user supplies a mapping with `#[elastic(mapping="")]`, then use it.
		//Otherwise, define the mapping struct and implement defaults for it.
		if let Some(name) = get_mapping(input) {
			name
		}
		else {
			let name = get_default_mapping(item);
			let es_ty = get_elastic_type_name(span, item);

			define_mapping(cx, &name, push);
			impl_object_mapping(cx, &name, &es_ty, push);

			name
		}
	};
	
	impl_type(cx, item, &name, push);

	let stmts = get_props_ser_stmts(cx, span, fields);
	impl_props_mapping(cx, span, &name, stmts, push);
}

//Define a struct for the mapping with a few defaults
fn define_mapping(name: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote!(
			#[derive(Default, Clone, Copy, Debug)]
			pub struct #name;
		).unwrap()
	));
}

//Implement ElasticType for the type being derived with the mapping
fn impl_type(item: &syn::MacroInput, mapping: &syn::Ident) {
	let ty = item.ident;

	quote!(
		impl ::elastic_types::mapping::ElasticType<#mapping> for #ty { }
	)
}

//Implement ObjectMapping for the mapping
fn impl_object_mapping(mapping: &syn::Ident, es_ty: &syn::Lit) {
	quote!(
		impl ObjectMapping for #mapping {
			fn name() -> &'static str { #es_ty }
		}
	)
}

//Implement PropertiesMapping for the mapping
fn impl_props_mapping(mapping: &syn::Ident, prop_ser_stmts: Vec<syn::Stmt>) {
	let stmts_len = prop_ser_stmts.len();
	let stmts_block = syn::Block {
		stmts: prop_ser_stmts,
		rules: syn::BlockCheckMode::Default
	};

	quote!(
		impl ::elastic_types::object::PropertiesMapping for #mapping {
			fn props_len() -> usize { #stmts_len }
			
			fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
			where S: ::serde::Serializer {
				#stmts_block
			}
		}
	)
}

//Get the serde serialisation statements for each of the fields on the type being derived
fn get_props_ser_stmts(fields: &[(syn::Ident, syn::Field)]) -> Vec<syn::Stmt> {
	let mut fields: Vec<syn::Stmt> = fields.iter().cloned().map(|(name, field)| {
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
			let mut ty = ty.clone();

			ty.segments.push(syn::PathSegment {
				ident: "mapping_ser".into(),
				parameters: syn::PathParameters::none()
			});

			let expr = syn::Expr::Call(
				Box::new(syn::Expr::Path(None, ty)), 
				Vec::new()
			);

			let expr = syn::parse_expr(
				quote!(try!(serializer.serialize_struct_elt(state, #lit, #expr));)
					.to_string()
					.as_ref()
			).unwrap();

			Some(syn::Stmt::Semi(Box::new(expr)))
		}
		else {
			None
		}
	})
	.filter_map(|stmt| stmt)
	.collect();

	fields.push(quote!(Ok(())));

	fields
}

//Get the mapping ident supplied by an #[elastic()] attribute or create a default one
fn get_mapping(item: &syn::MacroInput) -> Option<syn::Ident> {
	for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
		for meta_item in meta_items {
			match *meta_item {
				// Parse `#[elastic(mapping="foo")]`
				syn::MetaItem::NameValue(ref name, ref lit) if name == &"mapping" => {
					return Some(
						get_ident_from_lit("mapping", lit)
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
fn get_default_mapping(item: &Item) -> Ident {
	token::str_to_ident(&format!("{}Mapping", item.ident))
}

//Get the default name for the indexed elasticsearch type name
fn get_elastic_type_name(item: &Item) -> Lit {
	let name = token::str_to_ident(&format!("{}", item.ident.name.as_str()).to_lowercase());

	Lit {
		node: LitKind::Str(InternedString::new_from_name(name.name), syn::StrStyle::Cooked),
		span: span
	}
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

fn get_ser_field(field: &syn::Field) -> Option<(syn::Ident, syn::Field)> {
	let serde_field = serde_attr::Field::from_ast(cx, 0, field);

	//Get all fields on struct where there isn't `skip_serializing`
	if serde_field.skip_serializing() {
		return None;
	}

	Some((token::str_to_ident(serde_field.name().serialize_name().as_ref()), field.to_owned()))
}

fn get_ident_from_lit(name: &str, lit: &syn::Lit) -> Result<syn::Ident, &'static str> {
	match *lit {
		syn::Lit::Str(ref s, _) => Ok(syn::Ident::from(*s)),
		_ => {
			return Err("Unable to get str from lit");
		}
	}
}