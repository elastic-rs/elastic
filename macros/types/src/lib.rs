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

//TODO: Rework this: 
// - Declare mapping struct
// - Implement property mapping
// - Call macro

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate serde;
extern crate serde_codegen_internals;
extern crate serde_json;

use rustc_plugin::Registry;

mod object;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::attr;
use syntax::ast;
use syntax::ast::{ MetaItem, Ident };
use syntax::ptr::P;
use syntax::ext::base::{ ExtCtxt, Annotatable };
use syntax::print::pprust::lit_to_string;

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

	let es_ty = object::get_type_name(cx, item);
    let object_visitor = object::build_properties_visitor(cx, span, item, &fields, push);

    //Get or build the mapping type
	let field_mapping;
    if let Some(mapping) = object::get_field_mapping(cx, item) {
		field_mapping = mapping;
	}
	else {
		field_mapping = object::build_field_mapping(cx, item, push);
	}

	object::impl_field_mapping(cx, span, &es_ty, &field_mapping, &object_visitor, push);
	object::impl_type_mapping(cx, &field_mapping, &object_visitor, push);

	object::impl_type(cx, item, &field_mapping, push);
}

macro_rules! expect_item {
	($cx:ident, $meta_item:ident, $annotatable:ident) => ({
		let item = match *$annotatable {
	        Annotatable::Item(ref item) => {
	        	match item.node {
	        		ast::ItemKind::Struct(ref data, _) => {
	        			match *data {
	        				ast::VariantData::Struct(_, _) => Some(item),
							ast::VariantData::Unit(_) => Some(item),
							_ => None
	        			}
	        		},
	        		_ => None
	        	}
	        },
	        _ => None
	    };

		if item.is_none() {
	    	$cx.span_err(
	            $meta_item.span,
	            "`#[derive(Elastic	Mapping)]` may only be applied to structs");
	        return;
	    }
	    item.unwrap()
	})
}

//Helpers
fn get_elastic_meta_items(attr: &ast::Attribute) -> Option<&[P<ast::MetaItem>]> {
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
