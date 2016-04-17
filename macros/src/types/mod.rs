extern crate chrono;

pub mod date_format;

use syntax::codemap::Span;
use syntax::parse::token::{self, InternedString};
use syntax::attr;
use syntax::ast;
use syntax::ast::{ MetaItem, TokenTree, Ident };
use syntax::ptr::P;
use syntax::ext::base::{ ExtCtxt, MacResult, DummyResult, MacEager, Annotatable };
use syntax::ext::build::AstBuilder;
use syntax::print::pprust::lit_to_string;

pub fn expand_date_fmt(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult+'static> {
	let mut fmt = String::new();

	for arg in args {
		let _fmt = match *arg {
			TokenTree::Token(_, token::Literal(token::Lit::Str_(s), _)) => s.to_string(),
			_ => {
				cx.span_err(sp, "argument should be a single string literal");
				return DummyResult::any(sp);
			}
		};

		fmt.push_str(&_fmt);
	}

	//Build up the token tree
	let tokens = date_format::to_tokens(&fmt);
	let token_expr = cx.expr_vec(sp, tokens.iter().map(|t| date_format::Formatter::to_stmt(t, cx)).collect());

	MacEager::expr(quote_expr!(cx, { $token_expr }))
}

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
    let (item, fields, generics) = item.unwrap();

    //Get or build the mapping type
    //TODO: Pass in fn to build_mapping rather than maybe returning None
    let (mapping, mapping_ident) = get_mapping(cx, item).unwrap_or(build_mapping(cx, span, item));

    //Get the serializable fields
    //TODO: Use Vec<(Ident, ast::StructField)>
    let fields: Vec<(InternedString, ast::StructField)> = fields
    	.iter()
    	.map(|f| get_ser_field_name(cx, f))
    	.filter(|f| f.is_some())
    	.map(|f| f.unwrap())
    	.collect();

    if let Some(mapping_ident) = mapping_ident {
    	push(Annotatable::Item(mapping_ident));
    }
}

//Try get mapping name from attribute
fn get_mapping(cx: &ExtCtxt, item: &ast::Item) -> Option<(Ident, Option<P<ast::Item>>)> {
	for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[elastic(mapping="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"mapping" => {
                    let s = get_ident_from_lit(cx, name, lit).unwrap();

                    return Some((s, None));
                }
                _ => ()
            }
        }
    }

    None
}

//Build a mapping type and return the name
fn build_mapping(cx: &mut ExtCtxt, span: Span, item: &ast::Item) -> (Ident, Option<P<ast::Item>>) {
	let name = token::str_to_ident(&format!("{}Mapping", item.ident));

	(name, quote_item!(cx, 
		pub struct $name;
	))
}

fn get_ser_field_name(cx: &mut ExtCtxt, field: &ast::StructField) -> Option<(InternedString, ast::StructField)> {
	//Get all fields on struct where there isn't `skip_serializing`
	if !serialized_by_serde(field) {
		return None;
	}

	let name = get_field_name(cx, field);
	Some((name, field.to_owned()))
}

//TODO: Use serde_codegen for this
fn serialized_by_serde(field: &ast::StructField) -> bool {
    for meta_items in field.attrs.iter().filter_map(get_serde_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                ast::MetaItemKind::Word(ref name) if name == &"skip_serializing" => {
                    return false
                }
                _ => {}
            }
        }
    }
    true
}

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

fn get_serde_meta_items(attr: &ast::Attribute) -> Option<&[P<ast::MetaItem>]> {
    match attr.node.value.node {
        //Also get serde meta items, but don't mark as used
        ast::MetaItemKind::List(ref name, ref items) if name == &"serde" => {
            Some(items)
        }
        _ => None
    }
}

//TODO: Use serde_codegen for this
//TODO: Return Ident
fn get_field_name(cx: &ExtCtxt, field: &ast::StructField) -> InternedString {
	for meta_items in field.attrs.iter().filter_map(get_serde_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[serde(rename="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"rename" => {
                    let s = get_str_from_lit(cx, name, lit).unwrap_or(field.ident.unwrap().name.as_str());

                    return s;
                }
                _ => ()
            }
        }
    }

    field.ident.unwrap().name.as_str()
}

fn get_str_from_lit(cx: &ExtCtxt, name: &str, lit: &ast::Lit) -> Result<InternedString, &'static str> {
    match lit.node {
        ast::LitKind::Str(ref s, _) => Ok(s.clone()),
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
