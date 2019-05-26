use super::{
    helpers::*,
    types,
};
use parse::Endpoint;
use syn;

/// Structure for a request type constructor associated function.
struct Constructor {
    ident: syn::Ident,
    doc_comment: Option<String>,
    params_fields: Vec<(syn::Ident, syn::Ty)>,
    body_field: Option<(syn::Ident, syn::Ty)>,
}

impl Constructor {
    pub fn fields(&self) -> Vec<&(syn::Ident, syn::Ty)> {
        self.params_fields.iter().collect()
    }
}

/// Builder for request type constructor associated functions.
pub struct RequestParamsCtorBuilder {
    req_ty: syn::Ty,
    params_ty: syn::Ty,
    has_body: bool,
    ctors: Vec<Constructor>,
}

impl RequestParamsCtorBuilder {
    pub fn new(has_body: bool, request_ty: syn::Ty, params_ty: syn::Ty) -> Self {
        RequestParamsCtorBuilder {
            req_ty: request_ty,
            params_ty: params_ty,
            has_body: has_body,
            ctors: vec![],
        }
    }

    pub fn with_constructor(mut self, params: Vec<String>, doc_comment: Option<String>) -> Self {
        let ctor = match params.len() {
            0 => Self::ctor_none(self.has_body, doc_comment),
            _ => {
                let name: Vec<String> = params.iter().map(|i| i.into_rust_var()).collect();
                let name = format!("for_{}", name.join("_"));

                let cased: Vec<String> = params.iter().map(|i| i.into_rust_type()).collect();

                Self::ctor(&name, cased, self.has_body, doc_comment)
            }
        };

        self.ctors.push(ctor);

        self
    }

    /// A constructor function with no url parameters.
    ///
    /// This function has the form `new(body?)`.
    fn ctor_none(has_body: bool, doc_comment: Option<String>) -> Constructor {
        let body_field = Self::body_field(has_body);

        Constructor {
            ident: ident("new"),
            doc_comment: doc_comment,
            params_fields: vec![],
            body_field: body_field,
        }
    }

    /// A constructor function with url parameters.
    ///
    /// This function has the form `param1_param2(param1, param2, body?)`.
    fn ctor(
        name: &str,
        fields: Vec<String>,
        has_body: bool,
        doc_comment: Option<String>,
    ) -> Constructor {
        let fields: Vec<(syn::Ident, syn::Ty)> = fields
            .iter()
            .map(|f| (ident(f.into_rust_var()), ty_a(f)))
            .collect();

        let body_field = Self::body_field(has_body);

        Constructor {
            ident: ident(name),
            doc_comment: doc_comment,
            params_fields: fields,
            body_field: body_field,
        }
    }

    /// A helper for getting an optional `Ident` and `Type` for representing
    /// a body field.
    fn body_field(has_body: bool) -> Option<(syn::Ident, syn::Ty)> {
        if has_body {
            Some((ident("body"), types::body::ty()))
        } else {
            None
        }
    }

    /// Get a generic bound for a given type.
    ///
    /// The bound has the form `I{type_name}`.
    fn field_generic_ty(field: &syn::Ty) -> String {
        let generic_ident = field.get_ident().into_rust_type();

        format!("I{}", generic_ident)
    }

    /// Build the `IT` generic type.
    fn ctor_field_generic(field: &syn::Ty) -> syn::TyParam {
        let generic_ident = Self::field_generic_ty(field);

        ty_param(&generic_ident, vec![])
    }

    /// Build the `IT: Into<T>` generic where bound.
    fn ctor_field_generic_where_bound(field: &syn::Ty) -> syn::WherePredicate {
        let generic_ident = Self::field_generic_ty(field);

        let into_bound = path("Into", vec![], vec![field.clone()]);

        syn::WherePredicate::BoundPredicate(syn::WhereBoundPredicate {
            bound_lifetimes: vec![],
            bounded_ty: ty(&generic_ident),
            bounds: vec![ty_bound(into_bound)],
        })
    }

    /// Build the `T { url_params: UrlParams::ABC(a.into(), b.into(), c.into()), body: Body::new(body) }` body.
    fn ctor_body(req_ty: syn::Ty, params_ty: syn::Ty, ctor: &Constructor) -> syn::Block {
        let req_ty = {
            let mut path = req_ty.get_path().to_owned();
            path.segments[0].parameters = syn::PathParameters::none();
            path
        };

        let mut params_ty = {
            let mut path = params_ty.get_path().to_owned();
            path.segments[0].parameters = syn::PathParameters::none();
            path
        };

        let params_ty_variant = {
            match ctor.params_fields.len() {
                0 => ident("None"),
                _ => {
                    let params_ty_variant: Vec<String> = ctor
                        .params_fields
                        .iter()
                        .map(|&(_, ref t)| t.get_ident().into_rust_type())
                        .collect();

                    let params_ty_variant = params_ty_variant.join("");

                    ident(params_ty_variant)
                }
            }
        };

        params_ty.segments.push(syn::PathSegment {
            ident: ident(params_ty_variant),
            parameters: syn::PathParameters::none(),
        });

        let params_expr = match ctor.params_fields.len() {
            0 => syn::ExprKind::Path(None, params_ty).into(),
            _ => syn::ExprKind::Call(
                Box::new(syn::ExprKind::Path(None, params_ty).into()),
                ctor.params_fields
                    .iter()
                    .map(|&(ref f, _)| Self::expr_into(f))
                    .collect(),
            )
            .into(),
        };

        // AST to set the url field: `url: UrlParams::SomeVariant(a, b).url()`
        let mut fields = vec![syn::FieldValue {
            attrs: vec![],
            ident: ident("url"),
            expr: syn::ExprKind::MethodCall(ident("url"), vec![], vec![params_expr]).into(),
            is_shorthand: false,
        }];

        // AST to set the body field, if present: `body: Body::new(body)`
        if let &Some((ref body_ident, _)) = &ctor.body_field {
            fields.push(syn::FieldValue {
                attrs: vec![],
                ident: ident("body"),
                expr: syn::ExprKind::Path(None, path_none(body_ident.as_ref())).into(),
                is_shorthand: false,
            });
        }

        let stmt = syn::Stmt::Expr(Box::new(syn::ExprKind::Struct(req_ty, fields, None).into()));

        syn::Block { stmts: vec![stmt] }
    }

    fn expr_into(field: &syn::Ident) -> syn::Expr {
        method("into", vec![field.to_string().as_ref()])
    }

    /// Build the function header for the constructor.
    fn ctor_decl(ctor: &Constructor) -> syn::FnDecl {
        // Generic field args: `index: IIndex`.
        let mut args: Vec<syn::FnArg> = ctor
            .fields()
            .iter()
            .map(|&f| {
                let (ref name, ref field) = *f;

                let ty = ty(&Self::field_generic_ty(field));

                syn::FnArg::Captured(syn::Pat::Path(None, path_none(&name.to_string())), ty)
            })
            .collect();

        // Body arg: `body: B`.
        if let Some(ref body) = ctor.body_field {
            args.push({
                let (ref name, _) = *body;

                syn::FnArg::Captured(
                    syn::Pat::Path(None, path_none(&name.to_string())),
                    types::body::ty(),
                )
            });
        }

        syn::FnDecl {
            inputs: args,
            output: syn::FunctionRetTy::Ty(ty("Self")),
            variadic: false,
        }
    }

    /// Build a self implementation for the request parameter constructors.
    fn ctor_item(req_ty: syn::Ty, params_ty: syn::Ty, ctor: &Constructor) -> syn::ImplItem {
        let generic_fields: Vec<&syn::Ty> = ctor
            .fields()
            .iter()
            .map(|&f| {
                let (_, ref ty) = *f;
                ty
            })
            .collect();

        let generic_field_tys: Vec<syn::TyParam> = generic_fields
            .iter()
            .map(|ty| Self::ctor_field_generic(ty))
            .collect();

        let generic_field_where_tys: Vec<syn::WherePredicate> = generic_fields
            .iter()
            .map(|ty| Self::ctor_field_generic_where_bound(ty))
            .collect();

        let generics = syn::Generics {
            lifetimes: vec![],
            ty_params: generic_field_tys,
            where_clause: syn::WhereClause {
                predicates: generic_field_where_tys,
            },
        };

        let fndecl = Self::ctor_decl(&ctor);

        let body = Self::ctor_body(req_ty.clone(), params_ty, &ctor);

        let mut attrs = vec![];
        if let Some(ref doc_comment) = ctor.doc_comment {
            attrs.push(doc(doc_comment.to_owned()));
        }

        syn::ImplItem {
            ident: ctor.ident.clone(),
            vis: syn::Visibility::Public,
            defaultness: syn::Defaultness::Final,
            attrs: attrs,
            node: syn::ImplItemKind::Method(
                syn::MethodSig {
                    unsafety: syn::Unsafety::Normal,
                    constness: syn::Constness::NotConst,
                    abi: None,
                    decl: fndecl,
                    generics: generics,
                },
                body,
            ),
        }
    }

    pub fn build(self) -> syn::Item {
        let ctors: Vec<syn::ImplItem> = self
            .ctors
            .iter()
            .map(|c| Self::ctor_item(self.req_ty.clone(), self.params_ty.clone(), c))
            .collect();

        let generics = {
            let segment = self
                .req_ty
                .get_path()
                .to_owned()
                .segments
                .into_iter()
                .next()
                .unwrap();

            match segment.parameters {
                syn::PathParameters::AngleBracketed(data) => {
                    let types = data
                        .types
                        .iter()
                        .map(|t| ty_param(t.get_ident().as_ref(), vec![]))
                        .collect();

                    generics(data.lifetimes, types)
                }
                _ => panic!("Only angle bracketed generics are supported."),
            }
        };

        syn::Item {
            ident: ident(""),
            vis: syn::Visibility::Public,
            attrs: vec![],
            node: syn::ItemKind::Impl(
                syn::Unsafety::Normal,
                syn::ImplPolarity::Positive,
                generics,
                None,
                Box::new(self.req_ty),
                ctors,
            ),
        }
    }
}

impl<'a>
    From<(
        &'a (String, Endpoint),
        &'a syn::Ty,
        &'a (syn::Item, syn::Ty),
    )> for RequestParamsCtorBuilder
{
    fn from(
        value: (
            &'a (String, Endpoint),
            &'a syn::Ty,
            &'a (syn::Item, syn::Ty),
        ),
    ) -> Self {
        let (&(_, ref endpoint), ref req_ty, &(ref params, ref params_ty)) = value;

        let mut builder = RequestParamsCtorBuilder::new(
            endpoint.has_body(),
            (*req_ty).to_owned(),
            (*params_ty).to_owned(),
        );

        let ctors: Vec<(Vec<String>, String)> = match params.node {
            syn::ItemKind::Enum(ref variants, _) => variants
                .iter()
                .zip(endpoint.url.paths.iter())
                .map(|(v, p)| {
                    let doc = format!("Request to: `{}`", p);

                    match v.data {
                        syn::VariantData::Unit => {
                            let args = vec![];

                            (args, doc)
                        }
                        syn::VariantData::Tuple(ref fields) => {
                            let args = fields
                                .iter()
                                .map(|f| f.ty.get_ident().to_string())
                                .collect();

                            (args, doc)
                        }
                        _ => panic!("Only tuple and unit variants are supported."),
                    }
                })
                .collect(),
            _ => panic!("Only enum types are supported."),
        };

        for (ctor, doc) in ctors {
            builder = builder.with_constructor(ctor, Some(doc));
        }

        builder
    }
}

#[cfg(test)]
pub mod tests {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
    use super::*;

    #[test]
    fn gen_request_ctor_none() {
        let req_ty = ty_a("Request");
        let result = RequestParamsCtorBuilder::new(false, req_ty, ty_a("UrlParams")).with_constructor(vec![], Some("A doc comment".to_owned())).build();

        let expected = quote!(
            impl<'a> Request<'a> {
                #[doc = "A doc comment"]
                pub fn new() -> Self {
                    Request { url: UrlParams::None.url() }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_params() {
        let req_ty = ty_a("Request");
        let result = RequestParamsCtorBuilder::new(false, req_ty, ty_a("UrlParams"))
            .with_constructor(vec!["Index".into(), "Type".into(), "Id".into()], Some("A doc comment".to_owned()))
            .build();

        let expected = quote!(
            impl<'a> Request<'a> {
                #[doc = "A doc comment"]
                pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
                where
                    IIndex: Into<Index<'a> >,
                    IType: Into<Type<'a> >,
                    IId: Into<Id<'a> >
                {
                    Request {
                        url: UrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url()
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_body() {
        let req_ty = ty_path("Request", vec![lifetime()], vec![types::body::ty()]);
        let result = RequestParamsCtorBuilder::new(true, req_ty, ty_a("UrlParams")).with_constructor(vec![], None).build();

        let expected = quote!(
            impl<'a, B> Request<'a, B> {
                pub fn new(body: B) -> Self {
                    Request { url: UrlParams::None.url(), body: body }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_params_body() {
        let req_ty = ty_path("Request", vec![lifetime()], vec![types::body::ty()]);
        let result = RequestParamsCtorBuilder::new(true, req_ty, ty_a("UrlParams")).with_constructor(vec!["Index".into(), "Type".into(), "Id".into()], None).build();

        let expected = quote!(
            impl<'a, B> Request<'a, B> {
                pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
                where
                    IIndex: Into<Index<'a> >,
                    IType: Into<Type<'a> >,
                    IId: Into<Id<'a> >
                {
                    Request {
                        url: UrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                        body: body
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_from_endpoint() {
        use gen::url_params::UrlParamBuilder;
        use parse::*;

        let endpoint = (
            "indices.exists_alias".to_string(),
            Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: Some(Body { description: String::new() }),
            },
        );

        let req_ty = ty_path("IndicesExistsAliasRequest", vec![lifetime()], vec![types::body::ty()]);
        let url_params = UrlParamBuilder::from(&endpoint).build();

        let result = RequestParamsCtorBuilder::from((&endpoint, &req_ty, &url_params)).build();

        let expected = quote!(
            impl<'a, B> IndicesExistsAliasRequest<'a, B> {
                #[doc = "Request to: `/_search`"]
                pub fn new(body: B) -> Self {
                    IndicesExistsAliasRequest {
                        url: IndicesExistsAliasUrlParams::None.url(),
                        body: body
                    }
                }

                #[doc = "Request to: `/{index}/_search`"]
                pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
                where
                    IIndex: Into<Index<'a> >
                {
                    IndicesExistsAliasRequest {
                        url: IndicesExistsAliasUrlParams::Index(index.into()).url(),
                        body: body
                    }
                }

                #[doc = "Request to: `/{index}/{type}/_search`"]
                pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
                where
                    IIndex: Into<Index<'a> >,
                    IType: Into<Type<'a> >
                {
                    IndicesExistsAliasRequest {
                        url: IndicesExistsAliasUrlParams::IndexType(index.into(), ty.into()).url(),
                        body: body
                    }
                }
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
