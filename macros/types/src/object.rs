use syntax::codemap::Span;
use syntax::parse::token::{ self, InternedString };
use syntax::ast::{ self, Ident, Lit, LitKind, MetaItemKind };
use syntax::attr::HasAttrs;
use syntax::ext::base::{ ExtCtxt, Annotatable };
use syntax::ext::build::AstBuilder;

//Build a field mapping type and return the name
pub fn build_mapping(cx: &mut ExtCtxt, span: Span, item: &ast::Item, fields: &[(Ident, ast::StructField)], push: &mut FnMut(Annotatable)) {
	let name = token::str_to_ident(&format!("{}Mapping", item.ident));

	impl_type(cx, item, &name, push);

	push(Annotatable::Item(
		quote_item!(cx,
			#[derive(Default, Clone)]
			pub struct $name;
		).unwrap()
	));

	let es_ty = get_type_name(span, item);
	let stmts = get_props_ser_stmts(cx, span, fields);

	impl_type_mapping(cx, &name, &es_ty, push);
	impl_props_mapping(cx, span, &name, stmts, push);
}

fn impl_type(cx: &mut ExtCtxt, item: &ast::Item, mapping: &Ident, push: &mut FnMut(Annotatable)) {
	let ty = item.ident;

	push(Annotatable::Item(
		quote_item!(cx,
			impl ::elastic_types::mapping::ElasticType<$mapping> for $ty { }
		).unwrap()
	));
}

fn impl_type_mapping(cx: &mut ExtCtxt, mapping: &Ident, es_ty: &Lit, push: &mut FnMut(Annotatable)) {
	push(Annotatable::Item(
		quote_item!(cx,
			impl ObjectMapping for $mapping {
				fn name() -> &'static str { $es_ty }
			}
		).unwrap()
	));
}

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

pub fn get_type_name(span: Span, item: &ast::Item) -> Lit {
	for meta_items in item.attrs().iter().filter_map(super::get_elastic_meta_items) {
		for meta_item in meta_items {
			match meta_item.node {
				// Parse `#[elastic(ty="foo")]`
				MetaItemKind::NameValue(ref name, ref lit) if name == &"ty" => {
					return lit.to_owned();
				}
				_ => ()
			}
		}
	}

	get_default_type_name(&item.ident, span)
}

pub fn get_default_type_name(name: &Ident, span: Span) -> Lit {
	let name = token::str_to_ident(&format!("{}", name.name.as_str()).to_lowercase());

	Lit {
		node: LitKind::Str(InternedString::new_from_name(name.name), ast::StrStyle::Cooked),
		span: span
	}
}
