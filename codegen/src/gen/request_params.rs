use syn;
use ::parse;
use super::types;
use super::helpers::*;

/// Builder for request parameters enum.
pub struct RequestParamBuilder {
    name: syn::Ident,
    url_params: syn::Ty,
    has_body: bool,
}

impl RequestParamBuilder {
    pub fn new(name: &str, url_params: syn::Ty) -> Self {
        RequestParamBuilder {
            name: ident(name),
            url_params: url_params,
            has_body: false,
        }
    }

    pub fn has_body(mut self, has_body: bool) -> Self {
        self.has_body = has_body;

        self
    }

    pub fn build(self) -> (syn::Item, syn::Ty) {
        let mut fields = vec![syn::Field {
            ident: Some(ident("url_params")),
            vis: syn::Visibility::Public,
            attrs: vec![],
            ty: self.url_params
        }];

        if self.has_body {
            fields.push(syn::Field {
                ident: Some(ident("body")),
                vis: syn::Visibility::Public,
                attrs: vec![],
                ty: types::body::ty(),
            });
        }

        let marker_ty = syn::Ty::Rptr(
            Some(lifetime()), 
            Box::new(
                syn::MutTy {
                    ty: ty("( )"),
                    mutability: syn::Mutability::Immutable
                }
            ));

        fields.push(syn::Field {
            ident: Some(ident("_a")),
            vis: syn::Visibility::Inherited,
            attrs: vec![],
            ty: ty_path("PhantomData", vec![], vec![marker_ty])
        });

        let fields = syn::VariantData::Struct(fields);

        let ty = ty_a(self.name.as_ref());

        let item = syn::Item {
            ident: self.name,
            vis: syn::Visibility::Public,
            attrs: vec![],
            node: syn::ItemKind::Struct(fields, generics()),
        };

        (item, ty)
    }
}

impl<'a> From<(&'a (String, parse::Endpoint), &'a syn::Ty)> for RequestParamBuilder {
    fn from(value: (&'a (String, parse::Endpoint), &'a syn::Ty)) -> Self {
        let (&(ref endpoint_name, ref endpoint), ref params_ty) = value;

        let name = format!("{}RequestParams", endpoint_name.into_rust_type());

        let builder = RequestParamBuilder::new(&name, (*params_ty).to_owned()).has_body(endpoint.body.is_some());

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_request_params_ty() {
        let (_, result) = RequestParamBuilder::new("RequestParams", ty_a("UrlParams")).build();

        let expected = quote!(RequestParams<'a>);

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_params() {
        let (result, _) = RequestParamBuilder::new("RequestParams", ty_a("UrlParams")).build();

        let expected = quote!(
            pub struct RequestParams<'a> {
                pub url_params: UrlParams<'a>,
                _a: PhantomData<&'a ()>
            }
        );

        ast_eq(expected, result.into_stmt());
    }

    #[test]
    fn gen_params_enum_from_endpoint() {
        use ::parse::*;

        let endpoint = ("indices.exists_alias".to_string(),
                        Endpoint {
            documentation: String::new(),
            methods: vec![HttpMethod::Get],
            url: get_url(),
            body: Some(Body { description: String::new() }),
        });

        let (result, _) = RequestParamBuilder::from((&endpoint, &ty_a("IndicesExistsAliasUrlParams"))).build();

        let expected = quote!(
            pub struct IndicesExistsAliasRequestParams<'a> {
                pub url_params: IndicesExistsAliasUrlParams<'a>,
                pub body: Body<'a>,
                _a: PhantomData<&'a ()>
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
