use syn;
use ::parse;
use super::types;
use super::helpers::*;

/// Builder for request parameters enum.
pub struct RequestParamBuilder {
    name: syn::Ident,
    has_body: bool,
}

impl RequestParamBuilder {
    pub fn new(name: &str) -> Self {
        RequestParamBuilder {
            name: ident(name),
            has_body: false,
        }
    }

    pub fn has_body(mut self, has_body: bool) -> Self {
        self.has_body = has_body;

        self
    }

    pub fn build(self) -> (syn::Item, syn::Ty) {
        let mut fields = vec![syn::Field {
            ident: Some(ident("url")),
            vis: syn::Visibility::Public,
            attrs: vec![],
            ty: types::url::ty()
        }];

        let mut generics = generics(vec![lifetime()], vec![]);

        if self.has_body {
            fields.push(syn::Field {
                ident: Some(ident("body")),
                vis: syn::Visibility::Public,
                attrs: vec![],
                ty: types::body::ty(),
            });

            generics.ty_params.push(ty_param(types::body::ident(), vec![]));
        }

        let fields = syn::VariantData::Struct(fields);

        let ty = ty_path(self.name.as_ref(), 
                         generics.lifetimes
                                 .iter()
                                 .map(|l| l.lifetime.to_owned())
                                 .collect(), 
                         generics.ty_params
                                 .iter()
                                 .map(|t| ty(t.ident.as_ref()))
                                 .collect());

        let item = syn::Item {
            ident: self.name,
            vis: syn::Visibility::Public,
            attrs: vec![],
            node: syn::ItemKind::Struct(fields, generics),
        };

        (item, ty)
    }
}

impl<'a> From<&'a (String, parse::Endpoint)> for RequestParamBuilder {
    fn from(value: &'a (String, parse::Endpoint)) -> Self {
        let &(ref endpoint_name, ref endpoint) = value;

        let name = format!("{}Request", endpoint_name.into_rust_type());

        let builder = RequestParamBuilder::new(&name).has_body(endpoint.has_body());

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_request_params_ty_no_body() {
        let (_, result) = RequestParamBuilder::new("Request").build();

        let expected = quote!(Request<'a>);

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_params_no_body() {
        let (result, _) = RequestParamBuilder::new("Request").build();

        let expected = quote!(
            pub struct Request<'a> {
                pub url: Url<'a>
            }
        );

        ast_eq(expected, result.into_stmt());
    }

    #[test]
    fn gen_request_params_ty_with_body() {
        let (_, result) = RequestParamBuilder::new("Request").has_body(true).build();

        let expected = quote!(Request<'a, B>);

        ast_eq(expected, result);
    }

    #[test]
    fn gen_request_params_with_body() {
        let (result, _) = RequestParamBuilder::new("Request").has_body(true).build();

        let expected = quote!(
            pub struct Request<'a, B> {
                pub url: Url<'a>,
                pub body: B
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

        let (result, _) = RequestParamBuilder::from(&endpoint).build();

        let expected = quote!(
            pub struct IndicesExistsAliasRequest<'a, B> {
                pub url: Url<'a>,
                pub body: B
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
