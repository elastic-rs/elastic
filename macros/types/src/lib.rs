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
extern crate serde_json;

use rustc_plugin::Registry;

mod object;

use syntax::codemap::Span;
use syntax::parse::token::{self};
use syntax::attr;
use syntax::ast;
use syntax::ast::{ MetaItem, Ident };
use syntax::ptr::P;
use syntax::ext::base::{ ExtCtxt, Annotatable };
use syntax::print::pprust::lit_to_string;

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

#[doc(hidden)]
pub fn expand_derive_string_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
    let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::string::mapping::ElasticStringMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::string::mapping::STRING_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_boolean_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::boolean::mapping::ElasticBooleanMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::boolean::mapping::BOOLEAN_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_integer_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticIntegerMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::INTEGER_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_long_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticLongMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::LONG_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_short_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticShortMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::SHORT_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_byte_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticByteMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::BYTE_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_double_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticDoubleMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::DOUBLE_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

#[doc(hidden)]
pub fn expand_derive_float_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
	let item = expect_item!(cx, meta_item, annotatable);
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $ty {
				type Visitor = ::elastic_types::number::mapping::ElasticFloatMappingVisitor<$ty>;
                type MultiFieldMapping = Self;

				fn data_type() -> &'static str {
					::elastic_types::number::mapping::FLOAT_DATATYPE
				}
			}
		).unwrap()
	));

	impl_mapping_ser(cx, &ty, push);
}

//TODO: Make it possible to implement for a single date format
#[doc(hidden)]
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
				impl <T: ::elastic_types::date::DateFormat> ::elastic_types::mapping::ElasticFieldMapping<T> for $ty<T> {
					type Visitor = ::elastic_types::date::mapping::ElasticDateMappingVisitor<T, $ty<T>>;
                    type MultiFieldMapping = Self;

					fn data_type() -> &'static str {
						::elastic_types::date::mapping::DATE_DATATYPE
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

//TODO: Make it possible to implement for a single geo_point format
#[doc(hidden)]
pub fn expand_derive_geo_point_mapping(cx: &mut ExtCtxt, _: Span, meta_item: &MetaItem, annotatable: &Annotatable, push: &mut FnMut(Annotatable)) {
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
			"`#[derive(ElasticGeoPointMapping)]` may only be applied to structs with a generic parameter");
		return;
	}
	let (item, generics) = item.unwrap();
	let ty = item.ident;

	if generics.ty_params.len() == 1 {
		push(Annotatable::Item(
			quote_item!(cx,
				impl <T: ::elastic_types::geo::point::GeoPointFormat> ::elastic_types::mapping::ElasticFieldMapping<T> for $ty<T> {
					type Visitor = ::elastic_types::geo::point::mapping::ElasticGeoPointMappingVisitor<T, $ty<T>>;
                    type MultiFieldMapping = Self;

					fn data_type() -> &'static str {
						::elastic_types::geo::point::mapping::GEOPOINT_TYPE
					}
				}
			).unwrap()
		));

		push(Annotatable::Item(
			quote_item!(cx,
				impl <T: ::elastic_types::geo::point::GeoPointFormat> serde::Serialize for $ty<T> {
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
			"`#[derive(ElasticGeoPointMapping)]` may only be applied to structs with a generic parameter");
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

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticType"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_type_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticStringMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_string_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticBooleanMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_boolean_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticIntegerMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_integer_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticLongMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_long_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticShortMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_short_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticByteMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_byte_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticDoubleMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_double_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticFloatMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_float_mapping))
	);

	reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticDateMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_date_mapping))
	);

    reg.register_syntax_extension(
		syntax::parse::token::intern("derive_ElasticGeoPointMapping"),
		syntax::ext::base::MultiDecorator(
			Box::new(expand_derive_geo_point_mapping))
	);
}
