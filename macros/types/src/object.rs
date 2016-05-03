use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast;
use syntax::ast::Ident;
use syntax::ext::base::{ ExtCtxt, Annotatable };
use syntax::ext::build::AstBuilder;

pub fn impl_type(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticType<$mapping, ()> for $ty { }
		).unwrap()
	));
}

//Build a field mapping type and return the name
pub fn build_field_mapping(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
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

pub fn impl_field_mapping(cx: &mut ExtCtxt, span: Span, es_ty: &Ident, mapping: &Ident, object_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	let lit = cx.expr_str(span, es_ty.name.as_str());

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticFieldMapping<()> for $mapping {
				type Visitor = ::elastic_types::object::ElasticObjectMappingVisitor<$mapping, $object_visitor>;

				fn data_type() -> &'static str {
					<Self as ElasticObjectMapping>::data_type()
				}

				fn name() -> &'static str {
					$lit
				}
			}
		).unwrap()
	));

	impl_field_mapping_ser(cx, mapping, push);
}

fn impl_field_mapping_ser(cx: &mut ExtCtxt, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::serde::Serialize for $mapping {
				fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
				where S: ::serde::Serializer {
					serializer.serialize_struct("", Self::get_visitor())
				}
			}
		).unwrap()
	));
}

pub fn impl_type_mapping(cx: &mut ExtCtxt, mapping: &Ident, object_visitor: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::object::ElasticUserTypeMapping for $mapping {
				type Visitor = ::elastic_types::object::ElasticUserTypeMappingVisitor<$object_visitor>;
			}
		).unwrap()
	));
}

//Build an object visitor and return the name
pub fn build_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}ObjectVisitor", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone)]
			pub struct $name;
		).unwrap()
	));

	impl_object_visitor(cx, span, &name, fields, push);

	name
}

fn impl_object_visitor(cx: &mut ExtCtxt, span: Span, visitor: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticTypeVisitor for $visitor {
				fn new() -> Self {
					$visitor
				}
			}
		).unwrap()
	));

	impl_object_visitor_ser(cx, span, visitor, fields, push);
}

fn impl_object_visitor_ser(cx: &mut ExtCtxt, span: Span, visitor: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {
	let stmts: Vec<ast::Stmt> = fields.iter().cloned().map(|(name, field)| {
		let lit = cx.expr_str(span, name.name.as_str());
		let ty = match field.ty.node {
			ast::TyKind::Path(_, ref p) => Some(p),
			_ => None
		};

		if let Some(ty) = ty {
			let mut ty = ty.clone();

			ty.segments.push(ast::PathSegment {
                identifier: token::str_to_ident("mapping"),
                parameters: ast::PathParameters::none()
            });

			let expr = cx.expr_call(span, cx.expr_path(ty), Vec::new());

			Some(quote_stmt!(cx,
				try!(serializer.serialize_struct_elt($lit, $expr));
			).unwrap())
		}
		else {
			None
		}
	})
	.filter_map(|stmt| stmt)
	.collect();

	let block = cx.expr_block(cx.block(span, stmts, None));

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::serde::ser::MapVisitor for $visitor {
				fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
				where S: ::serde::Serializer {
					$block

					Ok(None)
				}
			}
		).unwrap()
	));
}

pub fn get_type_name(cx: &ExtCtxt, item: &ast::Item) -> Ident {
	for meta_items in item.attrs().iter().filter_map(super::get_elastic_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[elastic(ty="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"ty" => {
                    let s = super::get_ident_from_lit(cx, name, lit).unwrap_or(get_default_type_name(&item.ident));

                    return s;
                }
                _ => ()
            }
        }
    }

    get_default_type_name(&item.ident)
}

pub fn get_default_type_name(name: &Ident) -> Ident {
	token::str_to_ident(&format!("{}", name.name.as_str()).to_lowercase())
}

//Try get mapping name from attribute
pub fn get_field_mapping(cx: &ExtCtxt, item: &ast::Item) -> Option<Ident> {
	for meta_items in item.attrs.iter().filter_map(super::get_elastic_meta_items) {
        for meta_item in meta_items {
            match meta_item.node {
                // Parse `#[elastic(mapping="foo")]`
                ast::MetaItemKind::NameValue(ref name, ref lit) if name == &"mapping" => {
                    let s = super::get_ident_from_lit(cx, name, lit).unwrap();

                    return Some(s);
                }
                _ => ()
            }
        }
    }

    None
}
