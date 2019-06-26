use crate::gen::helpers::*;

use crate::parse;
use inflector::Inflector;
use syn;

/// Builder for request url parameters enum.
///
/// The output of this structure is an enum that only accepts valid parameter combinations,
/// based on what's given in the paths for an endpoint.
pub struct Builder {
    name: syn::Ident,
    variants: Vec<syn::Variant>,
    has_lifetime: bool,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Builder {
            name: ident(name),
            variants: vec![],
            has_lifetime: false,
        }
    }

    pub fn name(&self) -> &syn::Ident {
        &self.name
    }

    /// Add a param set to this builder
    pub fn with_param_set(mut self, params: Vec<&str>) -> Self {
        let variant = match params.len() {
            0 => Self::param_none(),
            _ => {
                self.has_lifetime = true;

                let cased: Vec<String> = params.iter().map(|i| i.to_pascal_case()).collect();

                let name = cased.join("");

                Self::param(&name, cased)
            }
        };

        self.variants.push(variant);

        self
    }

    /// AST for a param variant.
    fn param(name: &str, params: Vec<String>) -> syn::Variant {
        syn::Variant {
            ident: ident(name),
            attrs: vec![],
            discriminant: None,
            data: syn::VariantData::Tuple(
                params
                    .iter()
                    .map(|param| syn::Field {
                        ident: None,
                        vis: syn::Visibility::Inherited,
                        attrs: vec![],
                        ty: ty_a(param.as_ref()),
                    })
                    .collect(),
            ),
        }
    }

    /// AST for a `None` param variant.
    fn param_none() -> syn::Variant {
        syn::Variant {
            ident: ident("None"),
            attrs: vec![],
            data: syn::VariantData::Unit,
            discriminant: None,
        }
    }

    /// Build this enum and return an AST for its declaration and an AST for its type.
    pub fn build(self) -> (syn::Item, syn::Ty) {
        let variants = match self.variants.len() {
            0 => vec![Self::param_none()],
            _ => self.variants,
        };

        let (ty, generics) = {
            if self.has_lifetime {
                (ty_a(self.name.as_ref()), generics_a())
            } else {
                (ty(self.name.as_ref()), generics_none())
            }
        };

        let item = syn::Item {
            ident: self.name,
            vis: syn::Visibility::Inherited,
            attrs: vec![syn::Attribute {
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
            }],
            node: syn::ItemKind::Enum(variants, generics),
        };

        (item, ty)
    }
}

impl<'a> From<&'a (String, parse::Endpoint)> for Builder {
    fn from(value: &'a (String, parse::Endpoint)) -> Self {
        let name = format!("{}UrlParams", value.0.into_rust_type());

        let endpoint = &value.1;

        let mut builder = Builder::new(&name);

        for path in &endpoint.url.paths {
            let param_set = path.params();

            builder = builder.with_param_set(param_set);
        }

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
                body: None,
            },
        );

        let (result, _) = Builder::from(&endpoint).build();

        let expected = quote!(
            #[derive(Debug, Clone, PartialEq)]
            enum IndicesExistsAliasUrlParams<'a> {
                None,
                Index(Index<'a>),
                IndexType(Index<'a>, Type<'a>),
            }
        );

        ast_eq(expected, result.into_stmt());
    }
}
