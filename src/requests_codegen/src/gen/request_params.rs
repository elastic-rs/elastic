use super::{
    helpers::*,
    types,
};
use parse;
use syn;

/// Builder for request parameters enum.
pub struct RequestParamBuilder {
    name: syn::Ident,
    has_body: bool,
    doc_comment: Option<String>,
}

impl RequestParamBuilder {
    pub fn new(name: &str) -> Self {
        RequestParamBuilder {
            name: ident(name),
            doc_comment: None,
            has_body: false,
        }
    }

    pub fn has_body(mut self, has_body: bool) -> Self {
        self.has_body = has_body;

        self
    }

    pub fn doc_comment<TDoc>(mut self, doc_comment: TDoc) -> Self
    where
        TDoc: Into<String>,
    {
        self.doc_comment = Some(doc_comment.into());

        self
    }

    pub fn build(self) -> (syn::Item, syn::Ty) {
        let mut fields = vec![syn::Field {
            ident: Some(ident("url")),
            vis: syn::Visibility::Public,
            attrs: vec![],
            ty: types::url::ty(),
        }];

        let mut generics = generics(vec![lifetime()], vec![]);

        if self.has_body {
            fields.push(syn::Field {
                ident: Some(ident("body")),
                vis: syn::Visibility::Public,
                attrs: vec![],
                ty: types::body::ty(),
            });

            generics
                .ty_params
                .push(ty_param(types::body::ident(), vec![]));
        }

        let fields = syn::VariantData::Struct(fields);

        let ty = ty_path(
            self.name.as_ref(),
            generics
                .lifetimes
                .iter()
                .map(|l| l.lifetime.to_owned())
                .collect(),
            generics
                .ty_params
                .iter()
                .map(|t| ty(t.ident.as_ref()))
                .collect(),
        );

        let mut attrs = vec![];

        if let Some(doc_comment) = self.doc_comment {
            attrs.push(doc(doc_comment));
        }

        let item = syn::Item {
            ident: self.name,
            vis: syn::Visibility::Public,
            attrs: attrs,
            node: syn::ItemKind::Struct(fields, generics),
        };

        (item, ty)
    }
}

impl<'a> From<&'a (String, parse::Endpoint)> for RequestParamBuilder {
    fn from(value: &'a (String, parse::Endpoint)) -> Self {
        let &(ref endpoint_name, ref endpoint) = value;

        let name = format!("{}Request", endpoint_name.into_rust_type());
        let doc_comment = if let Some(method) = endpoint.preferred_method() {
            format!(
                "`{:?}: {}`\n\n[Elasticsearch Documentation]({})",
                method, endpoint.url.path, endpoint.documentation
            )
        } else {
            format!("`{}`", endpoint.url.path)
        };

        let builder = RequestParamBuilder::new(&name)
            .has_body(endpoint.has_body())
            .doc_comment(doc_comment);

        builder
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
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
                pub url: UrlPath<'a>
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
    fn gen_request_params_with_body_doc() {
        let (result, _) = RequestParamBuilder::new("Request").has_body(true).doc_comment("Some doc").build();

        let expected = quote!(
            #[doc = "Some doc"]
            pub struct Request<'a, B> {
                pub url: UrlPath<'a>,
                pub body: B
            }
        );

        ast_eq(expected, result.into_stmt());
    }

    #[test]
    fn gen_params_enum_from_endpoint() {
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

        let (result, _) = RequestParamBuilder::from(&endpoint).build();

        let expected = quote!(
            #[doc = "`Get: /_search`\n\n[Elasticsearch Documentation]()"]
            pub struct IndicesExistsAliasRequest<'a, B> {
                pub url: UrlPath<'a>,
                pub body: B
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
