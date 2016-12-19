use syn;
use ::parse::Endpoint;
use super::helpers::*;
use super::types;

/// Structure for a request type constructor associated function.
struct Constructor {
    ident: syn::Ident,
    params_fields: Vec<(syn::Ident, syn::Ty)>,
    body_field: Option<(syn::Ident, syn::Ty)>,
}

impl Constructor {
    pub fn fields(&self) -> Vec<&(syn::Ident, syn::Ty)> {
        let mut fields: Vec<&(syn::Ident, syn::Ty)> = self.params_fields
            .iter()
            .collect();

        if let Some(ref body_field) = self.body_field {
            fields.push(body_field);
        }

        fields
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

    pub fn with_constructor(mut self, params: Vec<String>) -> Self {
        let ctor = match params.len() {
            0 => Self::ctor_none(self.has_body),
            _ => {
                let name: Vec<String> = params.iter().map(|i| i.into_rust_var()).collect();
                let name = format!("for_{}", name.join("_"));

                let cased: Vec<String> = params.iter()
                    .map(|i| i.into_rust_type())
                    .collect();

                Self::ctor(&name, cased, self.has_body)
            }
        };

        self.ctors.push(ctor);

        self
    }

    /// A constructor function with no url parameters.
    ///
    /// This function has the form `new(body?)`.
    fn ctor_none(has_body: bool) -> Constructor {
        let body_field = Self::body_field(has_body);

        Constructor {
            ident: ident("new"),
            params_fields: vec![],
            body_field: body_field,
        }
    }

    /// A constructor function with url parameters.
    ///
    /// This function has the form `param1_param2(param1, param2, body?)`.
    fn ctor(name: &str, fields: Vec<String>, has_body: bool) -> Constructor {
        let fields: Vec<(syn::Ident, syn::Ty)> = fields.iter()
            .map(|f| (ident(f.into_rust_var()), ty_a(f)))
            .collect();

        let body_field = Self::body_field(has_body);

        Constructor {
            ident: ident(name),
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

    /// Build the `IT: Into<T>` generic type.
    fn ctor_field_generic(field: &syn::Ty) -> syn::TyParam {
        let generic_ident = Self::field_generic_ty(field);

        syn::TyParam {
            attrs: vec![],
            ident: ident(generic_ident),
            bounds: vec![syn::TyParamBound::Trait(syn::PolyTraitRef {
                                                      bound_lifetimes: vec![],
                                                      trait_ref: syn::Path {
                                                          global: false,
                                                          segments: vec![syn::PathSegment {
                                                                             ident: ident("Into"),
                                                                             parameters: syn::PathParameters::AngleBracketed(syn::AngleBracketedParameterData {
                                                                                 lifetimes: vec![],
                                                                                 types: vec![field.clone()],
                                                                                 bindings: vec![],
                                                                             }),
                                                                         }],
                                                      },
                                                  },
                                                  syn::TraitBoundModifier::None)],
            default: None,
        }
    }

    /// Build the `T { url_params: UrlParams::ABC(a.into(), b.into(), c.into()), body: body.into() }` body.
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
                    let params_ty_variant: Vec<String> = ctor.params_fields
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
            _ => syn::ExprKind::Call(Box::new(syn::ExprKind::Path(None, params_ty).into()),
                                    ctor.params_fields.iter().map(|&(ref f, _)| Self::expr_into(f)).collect())
                 .into()
        };

        let mut fields = vec![syn::FieldValue {
                                  ident: ident("url"),
                                  expr: syn::ExprKind::MethodCall(ident("url"), vec![], vec![params_expr]).into(),
                                  is_shorthand: false,
                              }];

        if let &Some((ref body_ident, _)) = &ctor.body_field {
            fields.push(syn::FieldValue {
                ident: ident("body"),
                expr: Self::expr_into(body_ident),
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
    fn ctor_decl(req_ty: syn::Ty, ctor: &Constructor) -> syn::FnDecl {
        let args: Vec<syn::FnArg> = ctor.fields()
            .iter()
            .map(|&f| {
                let (ref name, ref field) = *f;

                let ty = ty(&Self::field_generic_ty(field));

                syn::FnArg::Captured(syn::Pat::Path(None, path(&name.to_string())), ty)
            })
            .collect();

        syn::FnDecl {
            inputs: args,
            output: syn::FunctionRetTy::Ty(req_ty),
            variadic: false,
        }
    }

    /// Build a self implementation for the request parameter constructors.
    fn ctor_item(req_ty: syn::Ty, params_ty: syn::Ty, ctor: &Constructor) -> syn::ImplItem {
        let field_generics: Vec<syn::TyParam> = ctor.fields()
            .iter()
            .map(|&f| {
                let (_, ref ty) = *f;
                Self::ctor_field_generic(ty)
            })
            .collect();

        let generics = syn::Generics {
            lifetimes: vec![],
            ty_params: field_generics,
            where_clause: syn::WhereClause { predicates: vec![] },
        };

        let fndecl = Self::ctor_decl(req_ty.clone(), &ctor);

        let body = Self::ctor_body(req_ty.clone(), params_ty, &ctor);

        syn::ImplItem {
            ident: ctor.ident.clone(),
            vis: syn::Visibility::Public,
            defaultness: syn::Defaultness::Final,
            attrs: vec![],
            node: syn::ImplItemKind::Method(syn::MethodSig {
                                                unsafety: syn::Unsafety::Normal,
                                                constness: syn::Constness::NotConst,
                                                abi: None,
                                                decl: fndecl,
                                                generics: generics,
                                            },
                                            body),
        }
    }

    pub fn build(self) -> syn::Item {
        let ctors: Vec<syn::ImplItem> = self.ctors
            .iter()
            .map(|c| Self::ctor_item(self.req_ty.clone(), self.params_ty.clone(), c))
            .collect();

        syn::Item {
            ident: ident(""),
            vis: syn::Visibility::Public,
            attrs: vec![],
            node: syn::ItemKind::Impl(syn::Unsafety::Normal,
                                      syn::ImplPolarity::Positive,
                                      generics_a(),
                                      None,
                                      Box::new(self.req_ty),
                                      ctors),
        }
    }
}

impl<'a> From<(&'a (String, Endpoint), &'a syn::Ty, &'a (syn::Item, syn::Ty))> for RequestParamsCtorBuilder {
    fn from(value: (&'a (String, Endpoint), &'a syn::Ty, &'a (syn::Item, syn::Ty))) -> Self {
        let (&(_, ref endpoint), ref req_ty, &(ref params, ref params_ty)) = value;

        let mut builder = RequestParamsCtorBuilder::new(endpoint.has_body(), (*req_ty).to_owned(), (*params_ty).to_owned());

        let ctors: Vec<Vec<String>> = match params.node {
            syn::ItemKind::Enum(ref variants, _) => {
                variants.iter()
                    .map(|v| {
                        match v.data {
                            syn::VariantData::Unit => vec![],
                            syn::VariantData::Tuple(ref fields) => fields.iter().map(|f| f.ty.get_ident().to_string()).collect(),
                            _ => panic!("Only tuple and unit variants are supported."),
                        }
                    })
                    .collect()
            }
            _ => panic!("Only enum types are supported."),
        };

        for ctor in ctors {
            builder = builder.with_constructor(ctor);
        }

        builder
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn gen_request_ctor_none() {
        let result = RequestParamsCtorBuilder::new(false, ty_a("Request"), ty_a("UrlParams"))
            .with_constructor(vec![])
            .build();

        let expected = quote!(
            impl <'a> Request<'a> {
                pub fn new() -> Request<'a> {
                    Request {
                        url: UrlParams::None.url()
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_params() {
        let result = RequestParamsCtorBuilder::new(false, ty_a("Request"), ty_a("UrlParams"))
            .with_constructor(vec![
                "Index".into(),
                "Type".into(),
                "Id".into()
            ])
            .build();

        let body = quote!(
            Request {
                url: UrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url()
            }
        );

        let expected = quote!(
            impl <'a> Request<'a> {
                pub fn for_index_ty_id<IIndex: Into<Index<'a> >, IType: Into<Type<'a> >, IId: Into<Id<'a> > >(index: IIndex, ty: IType, id: IId) -> Request<'a> {
                    #body
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_body() {
        let result = RequestParamsCtorBuilder::new(true, ty_a("Request"), ty_a("UrlParams"))
            .with_constructor(vec![])
            .build();

        let expected = quote!(
            impl <'a> Request<'a> {
                pub fn new<IBody: Into<Body<'a> > >(body: IBody) -> Request<'a> {
                    Request {
                        url: UrlParams::None.url(),
                        body: body.into()
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_params_body() {
        let result = RequestParamsCtorBuilder::new(true, ty_a("Request"), ty_a("UrlParams"))
            .with_constructor(vec![
                "Index".into(),
                "Type".into(),
                "Id".into()
            ])
            .build();

        let body = quote!(
            Request {
                url: UrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body.into()
            }
        );

        let generics = quote!(
            <IIndex: Into<Index<'a> >, IType: Into<Type<'a> >, IId: Into<Id<'a> >, IBody: Into<Body<'a> > >
        );

        let expected = quote!(
            impl <'a> Request<'a> {
                pub fn for_index_ty_id #generics (index: IIndex, ty: IType, id: IId, body: IBody) -> Request<'a> {
                    #body
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_ctor_from_endpoint() {
        use ::parse::*;
        use ::gen::url_params::UrlParamBuilder;

        let endpoint = ("indices.exists_alias".to_string(),
                        Endpoint {
            documentation: String::new(),
            methods: vec![HttpMethod::Get],
            url: get_url(),
            body: Some(Body { description: String::new() }),
        });

        let req_ty = ty_a("IndicesExistsAliasRequest");
        let url_params = UrlParamBuilder::from(&endpoint).build();

        let result = RequestParamsCtorBuilder::from((&endpoint, &req_ty, &url_params)).build();

        let ctor_new = quote!(
            pub fn new < IBody : Into < Body < 'a > > > ( body : IBody ) -> IndicesExistsAliasRequest < 'a > { 
                IndicesExistsAliasRequest { 
                    url : IndicesExistsAliasUrlParams::None.url(),
                    body: body.into()
                }
            }
        );

        let ctor_index = quote!(
            pub fn for_index < IIndex : Into < Index < 'a > >, IBody : Into < Body < 'a > > > ( index : IIndex, body : IBody ) -> IndicesExistsAliasRequest < 'a > { 
                IndicesExistsAliasRequest { 
                    url : IndicesExistsAliasUrlParams::Index(index.into()).url(),
                    body: body.into()
                }
            }
        );

        let ctor_index_ty = quote!(
            pub fn for_index_ty < IIndex : Into < Index < 'a > > , IType : Into < Type < 'a > >, IBody : Into < Body < 'a > > > ( index : IIndex , ty : IType , body: IBody ) -> IndicesExistsAliasRequest < 'a > { 
                IndicesExistsAliasRequest { 
                    url : IndicesExistsAliasUrlParams::IndexType(index.into(), ty.into()).url(),
                    body: body.into()
                }
            }
        );

        let expected = quote!(
            impl < 'a > IndicesExistsAliasRequest < 'a > { 
                #ctor_new

                #ctor_index

                #ctor_index_ty
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
