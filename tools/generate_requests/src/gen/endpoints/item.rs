use crate::{
    gen::{
        helpers::*,
        http,
    },
    parse::Endpoint,
};

use syn;

/// Builder for request parameters enum.
pub struct Builder {
    name: syn::Ident,
    has_body: bool,
    doc_comment: Option<String>,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Builder {
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
            ty: http::url::ty(),
        }];

        let mut generics = generics(vec![lifetime()], vec![]);

        if self.has_body {
            fields.push(syn::Field {
                ident: Some(ident("body")),
                vis: syn::Visibility::Public,
                attrs: vec![],
                ty: http::body::ty(),
            });

            generics
                .ty_params
                .push(ty_param(http::body::ident(), vec![]));
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

        let mut attrs = vec![syn::Attribute {
            is_sugared_doc: false,
            style: syn::AttrStyle::Outer,
            value: syn::MetaItem::List(
                ident("derive"),
                vec![
                    syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("Debug"))),
                    syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("Clone"))),
                    syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("PartialEq"))),
                ],
            ),
        }];

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

impl<'a> From<&'a (String, Endpoint)> for Builder {
    fn from(value: &'a (String, Endpoint)) -> Self {
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

        let builder = Builder::new(&name)
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
    fn gen_params_enum_from_endpoint() {
        use crate::parse::*;

        let endpoint = (
            "indices.exists_alias".to_string(),
            Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: Some(Body { description: String::new() }),
            },
        );

        let (result, _) = Builder::from(&endpoint).build();

        let expected = quote!(
            #[derive(Debug, Clone, PartialEq)]
            #[doc = "`Get: /_search`\n\n[Elasticsearch Documentation]()"]
            pub struct IndicesExistsAliasRequest<'a, B> {
                pub url: UrlPath<'a>,
                pub body: B
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
