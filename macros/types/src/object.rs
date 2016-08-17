use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast;
use syntax::ast::Ident;
use syntax::attr::HasAttrs;
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
pub fn build_mapping(cx: &mut ExtCtxt, span: Span, item: &ast::Item, fields: &Vec<(Ident, ast::StructField)>,  push: &mut FnMut(Annotatable)) {
	let name = token::str_to_ident(&format!("{}Mapping", item.ident));

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone)]
			pub struct $name;
		).unwrap()
	));

	impl_type_mapping(cx, &name, push);
	impl_properties_ser(cx, span, &name, fields, push);
}

pub fn impl_type_mapping(cx: &mut ExtCtxt, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::object::ElasticUserTypeMapping for $mapping {
				//TODO: props_len and serialize_props
			}
		).unwrap()
	));
}

//TODO: Make this return vec of serialize statements
fn impl_properties_ser(cx: &mut ExtCtxt, span: Span, mapping: &Ident, fields: &Vec<(Ident, ast::StructField)>, push: &mut FnMut(Annotatable)) {
	let stmts: Vec<ast::Stmt> = fields.iter().cloned().map(|(name, field)| {
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

	let block = cx.expr_block(cx.block(span, stmts));

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
