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
			impl <'a> ::elastic_types::mapping::ElasticType<$mapping<'a>, ()> for $ty { }
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

pub fn impl_field_mapping(cx: &mut ExtCtxt, span: Span, es_ty: &Ident, mapping: &Ident, field_visitor: &Ident, push: &mut FnMut(Annotatable)) {
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
pub fn build_type_mapping(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
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

pub fn impl_type_mapping(cx: &mut ExtCtxt, span: Span, es_ty: &Ident, item: &ast::Item, mapping: &Ident, type_visitor: &Ident, object_visitor: &Ident, field_visitor: &Ident, push: &mut FnMut(Annotatable)) {
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
pub fn build_field_visitor(cx: &mut ExtCtxt, item: &ast::Item, push: &mut FnMut(Annotatable)) -> Ident {
	let name = token::str_to_ident(&format!("{}FieldVisitor", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default)]
			pub struct $name;
		).unwrap()
	));

	name
}

pub fn impl_field_visitor_ser(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, field_visitor: &Ident, object_visitor: &Ident, push: &mut FnMut(Annotatable)) {
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
pub fn build_object_visitor(cx: &mut ExtCtxt, span: Span, item: &ast::Item, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) -> Ident {
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
pub fn build_type_visitor(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) -> Ident {
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
