//! Elasticsearch Core Types Codegen
//!
//! Compile-time code generation for Elasticsearch type implementations.
//! This crate provides custom `derive` attributes for data types in the [elastic_types](http://kodraus.github.io/rustdoc/elastic_types/) crate.
//!
//! # Links
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types_macros/")]

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote, plugin, stmt_expr_attributes)]
#![plugin(serde_macros)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate serde;
extern crate serde_codegen_internals;
extern crate serde_json;

use rustc_plugin::Registry;

use syntax::codemap::{ Span, Spanned };
use syntax::attr::{ self, HasAttrs };
use syntax::ast::{ self, Ident, Item, Lit, LitKind, MetaItem, MetaItemKind, NestedMetaItemKind };
use syntax::ext::base::{ ExtCtxt, Annotatable };
use syntax::ext::build::AstBuilder;
use syntax::print::pprust::lit_to_string;
use syntax::parse::token::{ self, InternedString };

use serde_codegen_internals::attr as serde_attr;

#[doc(hidden)]
pub fn expand_derive_type_mapping(cx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	//Annotatable item for a struct with struct fields
	let item = match *annotatable {
		Annotatable::Item(ref item) => {
			match item.node {
				ast::ItemKind::Struct(ref data, ref generics) => {
					match *data {
						ast::VariantData::Struct(ref fields, _) => Some((item, fields, generics)),
						_ => None
					}
				},
				_ => None
			}
		},
		_ => None
	};

	if item.is_none() {
		cx.span_err(
			meta_item.span,
			"`#[derive(ElasticType)]` may only be applied to structs");
		return;
	}
	let (item, fields, _) = item.unwrap();

	//Get the serializable fields
	let fields: Vec<(Ident, ast::StructField)> = fields
		.iter()
		.map(|f| get_ser_field(cx, f))
		.filter(|f| f.is_some())
		.map(|f| f.unwrap())
		.collect();

	build_mapping(cx, span, item, &fields, push);
}

//Build a field mapping type and return the name
pub fn build_mapping(cx: &mut ExtCtxt, span: Span, item: &Item, fields: &[(Ident, ast::StructField)], push: &mut FnMut(Annotatable)) {
	let name = {
		//If a user supplies a mapping with `#[elastic(mapping="")]`, then use it.
		//Otherwise, define the mapping struct and implement defaults for it.
		if let Some(name) = get_mapping(cx, item) {
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
fn define_mapping(cx: &mut ExtCtxt, name: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone, Copy, Debug)]
			pub struct $name;
		).unwrap()
	));
}

//Implement ElasticType for the type being derived with the mapping
fn impl_type(cx: &mut ExtCtxt, item: &Item, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticType<$mapping> for $ty { }
		).unwrap()
	));
}

//Implement ObjectMapping for the mapping
fn impl_object_mapping(cx: &mut ExtCtxt, mapping: &Ident, es_ty: &Lit, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ObjectMapping for $mapping {
				fn name() -> &'static str { $es_ty }
			}
		).unwrap()
	));
}

//Implement PropertiesMapping for the mapping
fn impl_props_mapping(cx: &mut ExtCtxt, span: Span, mapping: &Ident, prop_ser_stmts: Vec<ast::Stmt>, push: &mut FnMut(Annotatable)) {
	let stmts_len = prop_ser_stmts.len();
	let stmts_block = cx.expr_block(cx.block(span, prop_ser_stmts));

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::object::PropertiesMapping for $mapping {
				fn props_len() -> usize { $stmts_len }
				
				fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
				where S: ::serde::Serializer {
					$stmts_block
				}
			}
		).unwrap()
	));
}

//Get the serde serialisation statements for each of the fields on the type being derived
fn get_props_ser_stmts(cx: &mut ExtCtxt, span: Span, fields: &[(Ident, ast::StructField)]) -> Vec<ast::Stmt> {
	let mut fields: Vec<ast::Stmt> = fields.iter().cloned().map(|(name, field)| {
		let lit = cx.expr_str(span, name.name.as_str());
		let ty = match field.ty.node {
			//Standard type path
			ast::TyKind::Path(_, ref p) => Some(p),
			//Borrowed type
			ast::TyKind::Rptr(_, ref t) => {
				match t.ty.node {
					ast::TyKind::Path(_, ref p) => Some(p),
					_ => None
				}
			},
			_ => None
		};

		if let Some(ty) = ty {
			let mut ty = ty.clone();

			ty.segments.push(ast::PathSegment {
				identifier: token::str_to_ident("mapping_ser"),
				parameters: ast::PathParameters::none()
			});

			let expr = cx.expr_call(span, cx.expr_path(ty), Vec::new());

			Some(quote_stmt!(cx,
				try!(serializer.serialize_struct_elt(state, $lit, $expr));
			).unwrap())
		}
		else {
			None
		}
	})
	.filter_map(|stmt| stmt)
	.collect();

	fields.push(quote_stmt!(cx, Ok(())).unwrap());

	fields
}

//Get the mapping ident supplied by an #[elastic()] attribute or create a default one
pub fn get_mapping(cx: &mut ExtCtxt, item: &Item) -> Option<Ident> {
	for meta_items in item.attrs().iter().filter_map(get_elastic_meta_items) {
		for meta_item in meta_items {
			match meta_item.node {
				NestedMetaItemKind::MetaItem(ref meta_item) => {
					match meta_item.node {
						// Parse `#[elastic(mapping="foo")]`
						MetaItemKind::NameValue(ref name, ref lit) if name == &"mapping" => {
							return Some(
								get_ident_from_lit(cx, "mapping", lit)
									.unwrap_or(get_default_mapping(item))
							)
						},
						_ => ()
					}
				},
				_ => ()
			}
		}
	}

	None
}

//Get the default mapping name
pub fn get_default_mapping(item: &Item) -> Ident {
	token::str_to_ident(&format!("{}Mapping", item.ident))
}

//Get the default name for the indexed elasticsearch type name
pub fn get_elastic_type_name(span: Span, item: &Item) -> Lit {
	let name = token::str_to_ident(&format!("{}", item.ident.name.as_str()).to_lowercase());

	Lit {
		node: LitKind::Str(InternedString::new_from_name(name.name), ast::StrStyle::Cooked),
		span: span
	}
}

//Helpers
fn get_elastic_meta_items(attr: &ast::Attribute) -> Option<&[Spanned<ast::NestedMetaItemKind>]> {
	match attr.node.value.node {
		//Get elastic meta items
		ast::MetaItemKind::List(ref name, ref items) if name == &"elastic" => {
			attr::mark_used(&attr);
			Some(items)
		},
		_ => None
	}
}

fn get_ser_field(cx: &mut ExtCtxt, field: &ast::StructField) -> Option<(Ident, ast::StructField)> {
	let serde_field = serde_attr::Field::from_ast(cx, 0, field);

	//Get all fields on struct where there isn't `skip_serializing`
	if serde_field.skip_serializing() {
		return None;
	}

	Some((token::str_to_ident(serde_field.name().serialize_name().as_ref()), field.to_owned()))
}

fn get_ident_from_lit(cx: &ExtCtxt, name: &str, lit: &ast::Lit) -> Result<Ident, &'static str> {
	match lit.node {
		ast::LitKind::Str(ref s, _) => Ok(token::str_to_ident(s)),
		_ => {
			cx.span_err(
				lit.span,
				&format!("annotation `{}` must be a string, not `{}`",
						 name,
						 lit_to_string(lit)));

			return Err("Unable to get str from lit");
		}
	}
}

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticType"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_type_mapping))
	);
}
