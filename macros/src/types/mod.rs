extern crate chrono;

pub mod date_format;
mod object;

use syntax::codemap::Span;
use syntax::parse::token::{self};
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
    let (item, fields, _) = item.unwrap();

	//Get the serializable fields
    let fields: Vec<(Ident, ast::StructField)> = fields
    	.iter()
    	.map(|f| get_ser_field(cx, f))
    	.filter(|f| f.is_some())
    	.map(|f| f.unwrap())
    	.collect();

	let es_ty = object::get_type_name(cx, item);

    //Get or build the mapping type
	let field_mapping;
    if let Some(mapping) = object::get_field_mapping(cx, item) {
		field_mapping = mapping;
	}
	else {
		field_mapping = object::build_field_mapping(cx, item, push);
	}

	let field_visitor = object::build_field_visitor(cx, item, push);
	object::impl_field_mapping(cx, span, &es_ty, &field_mapping, &field_visitor, push);

	let object_visitor = object::build_object_visitor(cx, span, item, &fields, push);

	object::impl_field_visitor_ser(cx, item, &field_mapping, &field_visitor, &object_visitor, push);

	let type_mapping = object::build_type_mapping(cx, item, push);
	let type_visitor = object::build_type_visitor(cx, item, &type_mapping, push);
	object::impl_type_mapping(cx, span, &es_ty, item, &type_mapping, &type_visitor, &object_visitor, &field_visitor, push);

	object::impl_type(cx, item, &type_mapping, push);
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

pub fn expand_derive_string_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
    let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::string::mapping::ElasticStringMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"string"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_boolean_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::boolean::mapping::ElasticBooleanMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"boolean"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_integer_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticIntegerMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"integer"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_long_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticLongMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"long"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_short_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticShortMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"short"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_byte_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticByteMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"byte"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_double_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticDoubleMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"double"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

pub fn expand_derive_float_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticFloatMappingVisitor<$ty>;

				fn data_type() -> &'static str {
					"float"
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

//TODO: Make it possible to implement for a single date format
pub fn expand_derive_date_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = match *annotatable {
		Annotatable::Item(ref item) => {
			match item.node {
				ast::ItemKind::Struct(ref data, ref generics) => {
					match *data {
						ast::VariantData::Struct(_, _) => Some((item, generics)),
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
			"`#[derive(ElasticDateMapping)]` may only be applied to structs with a generic parameter");
		return;
	}
	let (item, generics) = item.unwrap();
	let ty = item.ident;

	if generics.ty_params.len() == 1 {
		push(Annotatable::Item(
			quote_item!(cx,
				impl <T: ::elastic_types::date::DateFormat> ::elastic_types::mapping::ElasticTypeMapping<T> for $ty<T> {
					type Visitor = ::elastic_types::date::mapping::ElasticDateMappingVisitor<T, $ty<T>>;

					fn data_type() -> &'static str {
						"date"
					}
				}
			).unwrap()
		));

		push(Annotatable::Item(
			quote_item!(cx,
				impl <T: ::elastic_types::date::DateFormat> serde::Serialize for $ty<T> {
					fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
					where S: serde::Serializer {
						serializer.serialize_struct("mapping", Self::get_visitor())
					}
				}
			).unwrap()
		));
	}
	else {
		cx.span_err(
			meta_item.span,
			"`#[derive(ElasticDateMapping)]` may only be applied to structs with a generic parameter");
		return;
	}
}

fn impl_mapping_ser(cx: &mut ExtCtxt, ty: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::serde::Serialize for $ty {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: ::serde::Serializer {
					serializer.serialize_struct("mapping", Self::get_visitor())
				}
			}
		).unwrap()
	));
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

//TODO: Use serde_codegen for this
fn get_field_name(cx: &ExtCtxt, field: &ast::StructField) -> Ident {
	for meta_items in field.attrs.iter().filter_map(get_serde_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[serde(rename="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"rename" => {
                    let s = get_ident_from_lit(cx, name, lit).unwrap_or(field.ident.unwrap());

                    return s;
                }
                _ => ()
            }
        }
    }

    field.ident.unwrap()
}

//TODO: Use serde_codegen for this
fn get_ser_field(cx: &mut ExtCtxt, field: &ast::StructField) -> Option<(Ident, ast::StructField)> {
	//Get all fields on struct where there isn't `skip_serializing`
	if !serialized_by_serde(field) {
		return None;
	}

	let name = get_field_name(cx, field);
	Some((name, field.to_owned()))
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
