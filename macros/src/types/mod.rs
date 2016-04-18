extern crate chrono;

pub mod date_format;

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

	let es_ty = get_type_name(cx, item);

    //Get or build the mapping type
	let field_mapping;
    if let Some(mapping) = get_field_mapping(cx, item) {
		field_mapping = mapping;
	}
	else {
		field_mapping = build_field_mapping(cx, item, push);
	}

	let field_visitor = build_field_visitor(cx, item, push);
	impl_field_mapping(cx, span, &es_ty, &field_mapping, &field_visitor, push);

	let object_visitor = build_object_visitor(cx, span, item, &fields, push);

	impl_field_visitor_ser(cx, item, &field_mapping, &field_visitor, &object_visitor, push);

	let type_mapping = build_type_mapping(cx, item, push);
	let type_visitor = build_type_visitor(cx, item, &type_mapping, push);
	impl_type_mapping(cx, span, &es_ty, item, &type_mapping, &type_visitor, &object_visitor, &field_visitor, push);

	impl_type(cx, item, &type_mapping, push);
}

fn impl_type(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::elastic_types::mapping::ElasticType<$mapping<'a>, ()> for $ty { }
		).unwrap()
	));
}

//Build a field mapping type and return the name
fn build_field_mapping(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}Mapping", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone)]
			pub struct $name;
		).unwrap()
	));

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::object::ElasticObjectMapping for $name { }
		).unwrap()
	));

	name
}

fn impl_field_mapping(cx: &mut ExtCtxt, span: Span, es_ty: &Ident, mapping: &Ident, field_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let lit = cx.expr_str(span, es_ty.name.as_str());

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeMapping<()> for $mapping {
				type Visitor = $field_visitor;

				fn data_type() -> &'static str {
					<Self as ElasticObjectMapping>::data_type()
				}

				fn name() -> &'static str {
					$lit
				}
			}
		).unwrap()
	));

	impl_field_mapping_ser(cx, mapping, field_visitor, push);
}

fn impl_field_mapping_ser(cx: &mut ExtCtxt, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::serde::Serialize for $mapping {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: ::serde::Serializer {
					serializer.serialize_struct("", $visitor)
				}
			}
		).unwrap()
	));
}

//Build a type mapping and return the name
fn build_type_mapping(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}TypeMapping", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone)]
			pub struct $name<'a> {
				phantom: ::std::marker::PhantomData<&'a ()>
			}
		).unwrap()
	));

	name
}

fn impl_type_mapping(cx: &mut ExtCtxt, span: Span, es_ty: &Ident, item: &ast::Item, mapping: &Ident, type_visitor: &Ident, object_visitor: &Ident, field_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;
	let lit = cx.expr_str(span, es_ty.name.as_str());

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::elastic_types::object::ElasticUserTypeMapping<'a, $ty> for $mapping<'a> {
				type Visitor = $type_visitor<'a>;
				type PropertiesVisitor = $object_visitor<'a>;
			}
		).unwrap()
	));

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::elastic_types::mapping::ElasticTypeMapping<()> for $mapping<'a> {
				type Visitor = $field_visitor;

				fn name() -> &'static str {
					$lit
				}
			}
		).unwrap()
	));

	impl_type_mapping_ser(cx, mapping, field_visitor, push);
}

fn impl_type_mapping_ser(cx: &mut ExtCtxt, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::serde::Serialize for $mapping<'a> {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: ::serde::Serializer {
					serializer.serialize_struct(Self::name(), $visitor::default())
				}
			}
		).unwrap()
	));
}

//Build a field visitor and return the name
fn build_field_visitor(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}FieldVisitor", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default)]
			pub struct $name;
		).unwrap()
	));

	name
}

fn impl_field_visitor_ser(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, field_visitor: &Ident, object_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::serde::ser::MapVisitor for $field_visitor {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: ::serde::Serializer {
					let mut object_mapper = ::elastic_types::object::ElasticObjectMappingVisitor::<$mapping>::default();
					try!(object_mapper.visit(serializer));

					let data = $ty::default();
					try!(serializer.serialize_struct_elt("properties", ::elastic_types::object::ElasticObjectProperties::<$ty, $object_visitor>::new(&data)));

					Ok(None)
				}
			}
		).unwrap()
	));
}

//Build an object visitor and return the name
fn build_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}ObjectPropertiesVisitor", item.ident));
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			pub struct $name<'a> {
				data: &'a $ty
			}
		).unwrap()
	));

	impl_object_visitor(cx, span, item, &name, fields, push);

	name
}

fn impl_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, visitor: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::elastic_types::object::ElasticObjectTypeVisitor<'a, $ty> for $visitor<'a> {
				fn new(data: &'a $ty) -> Self {
					$visitor {
						data: data
					}
				}
			}
		).unwrap()
	));

	impl_object_visitor_ser(cx, span, visitor, fields, push);
}

fn impl_object_visitor_ser(cx: &mut ExtCtxt, span: Span, visitor: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {
	let stmts: Vec<ast::Stmt> = fields.iter().cloned().map(|(name, field)| {
		let lit = cx.expr_str(span, name.name.as_str());
		let ident = field.ident.unwrap();

		quote_stmt!(cx,
			try!(::elastic_types::mappers::FieldMapper::map($lit, &self.data.$ident, serializer));
		).unwrap()
	}).collect();

	let block = cx.expr_block(cx.block(span, stmts, None));

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::serde::ser::MapVisitor for $visitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: ::serde::Serializer {
					$block

					Ok(None)
				}
			}
		).unwrap()
	));
}

//Build a type visitor and return the name
fn build_type_visitor(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}TypeMappingVisitor", item.ident));
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			pub struct $name<'a> {
				data: &'a $ty
			}
		).unwrap()
	));

	impl_type_visitor(cx, item, mapping, &name, push);

	name
}

fn impl_type_visitor(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::elastic_types::object::ElasticObjectTypeVisitor<'a, $ty> for $visitor<'a> {
				fn new(data: &'a $ty) -> Self {
					$visitor {
						data: data
					}
				}
			}
		).unwrap()
	));

	impl_type_visitor_ser(cx, item, mapping, visitor, push);
}

fn impl_type_visitor_ser(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> ::serde::ser::MapVisitor for $visitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: ::serde::Serializer {
					try!(serializer.serialize_struct_elt("properties", ::elastic_types::object::ElasticTypeProperties::<'a, $ty, $mapping<'a>>::new(&self.data)));

					Ok(None)
				}
			}
		).unwrap()
	));
}

//Helpers
fn get_ser_field(cx: &mut ExtCtxt, field: &ast::StructField) -> Option<(Ident, ast::StructField)> {
	//Get all fields on struct where there isn't `skip_serializing`
	if !serialized_by_serde(field) {
		return None;
	}

	let name = get_field_name(cx, field);
	Some((name, field.to_owned()))
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

fn get_type_name(cx: &ExtCtxt, item: &ast::Item) -> Ident {
	for meta_items in item.attrs().iter().filter_map(get_elastic_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[elastic(ty="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"ty" => {
                    let s = get_ident_from_lit(cx, name, lit).unwrap_or(get_default_type_name(&item.ident));

                    return s;
                }
                _ => ()
            }
        }
    }

    get_default_type_name(&item.ident)
}

fn get_default_type_name(name: &Ident) -> Ident {
	token::str_to_ident(&format!("{}", name.name.as_str()).to_lowercase())
}

//Try get mapping name from attribute
fn get_field_mapping(cx: &ExtCtxt, item: &ast::Item) -> Option<Ident> {
	for meta_items in item.attrs.iter().filter_map(get_elastic_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[elastic(mapping="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"mapping" => {
                    let s = get_ident_from_lit(cx, name, lit).unwrap();

                    return Some(s);
                }
                _ => ()
            }
        }
    }

    None
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
