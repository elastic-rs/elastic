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

	//Get the serializable fields
    let fields: Vec<(Ident, ast::StructField)> = fields
    	.iter()
    	.map(|f| get_ser_field(cx, f))
    	.filter(|f| f.is_some())
    	.map(|f| f.unwrap())
    	.collect();

    //Get or build the mapping type
    let mapping = get_mapping(cx, item)
		.unwrap_or(build_mapping(cx, span, item, push));

	let field_visitor = build_field_visitor(cx, span, item, &mapping, push);
	impl_field_mapping(cx, span, item, &mapping, &field_visitor, push);

	let object_visitor = build_object_visitor(cx, span, item, push);
}

//Try get mapping name from attribute
fn get_mapping(cx: &ExtCtxt, item: &ast::Item) -> Option<Ident> {
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

//Build a mapping type and return the name
fn build_mapping(cx: &mut ExtCtxt, span: Span, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}Mapping", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			pub struct $name;
		).unwrap()
	));

	name
}

fn impl_field_mapping(cx: &mut ExtCtxt, span: Span, item: &ast::Item, mapping: &Ident, field_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl elastic_types::mapping::ElasticTypeMapping<()> for $mapping {
				type Visitor = $field_visitor;

				fn data_type() -> &'static str {
					<Self as ElasticObjectMapping>::data_type()
				}
			}
		).unwrap()
	));

	impl_field_mapping_ser(cx, span, mapping, field_visitor, push);
}

fn impl_field_mapping_ser(cx: &mut ExtCtxt, span: Span, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl serde::Serialize for $mapping {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: serde::Serializer {
					serializer.serialize_struct("", $visitor)
				}
			}
		).unwrap()
	));
}

//Build a field visitor and return the name
fn build_field_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}FieldVisitor", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default)]
			pub struct $name;
		).unwrap()
	));

	impl_field_visitor_ser(cx, span, item, mapping, &name, push);

	name
}

fn impl_field_visitor_ser(cx: &mut ExtCtxt, span: Span, item: &ast::Item, mapping: &Ident, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;
	push(Annotatable::Item(
		quote_item!(cx,
			impl serde::ser::MapVisitor for $visitor {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					let mut object_mapper = elastic_types::object::mapping::ElasticObjectMappingVisitor::<$mapping>::default();
					try!(object_mapper.visit(serializer));

					let data = $ty::default();
					try!(serializer.serialize_struct_elt("properties", elastic_types::object::mapping::ElasticObjectProperties::<$ty, elastic_types::object::mapping::ObjectPropertiesVisitor>::new(&data)));

					Ok(None)
				}
			}
		).unwrap()
	));
}

//Build an object visitor and return the name
fn build_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}ObjectPropertiesVisitor", item.ident));
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			pub struct $name<'a> {
				data: &'a $ty
			}
		).unwrap()
	));

	impl_object_visitor(cx, span, item, &name, push);

	name
}

fn impl_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> elastic_types::object::mapping::ElasticObjectTypeVisitor<'a, $ty> for $visitor<'a> {
				fn new(data: &'a $ty) -> Self {
					$visitor {
						data: data
					}
				}
			}
		).unwrap()
	));
}

fn impl_object_visitor_ser(cx: &mut ExtCtxt, span: Span, item: &ast::Item, visitor: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	let mut stmts: Vec<ast::Stmt> = fields.iter().cloned().map(|(name, field)| {
		let lit = cx.expr_str(span, name.name.as_str());
		let ident = field.ident.unwrap();
		
		quote_stmt!(cx,
			try!(elastic_types::mappers::FieldMapper::map($lit, &self.data.$ident, serializer));
		).unwrap()
	}).collect();

	let block = cx.expr_block(cx.block(span, stmts, None));

	push(Annotatable::Item(
		quote_item!(cx,
			impl <'a> serde::ser::MapVisitor for $visitor<'a> {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: serde::Serializer {
					$block

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

//TODO: Use serde_codegen for this
//TODO: Return Ident
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
