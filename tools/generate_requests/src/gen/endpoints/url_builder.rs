use crate::{
    gen::{
        helpers::*,
        http,
    },
    parse::{
        Endpoint,
        PathPart,
    },
};

use syn;

pub struct Builder {
    params_ty: syn::Ty,
    body: syn::Expr,
}

impl Builder {
    pub fn new(url_params_ty: syn::Ty, body: syn::Expr) -> Self {
        Builder {
            params_ty: url_params_ty,
            body,
        }
    }

    pub fn build(self) -> syn::Item {
        let ret_ty = http::url::ty();

        let fndecl = syn::FnDecl {
            inputs: vec![syn::FnArg::SelfValue(syn::Mutability::Immutable)],
            output: syn::FunctionRetTy::Ty(ret_ty),
            variadic: false,
        };

        let (generics, fngenerics) = {
            if self.params_ty.has_lifetime() {
                (generics_a(), generics_none())
            } else {
                (generics_none(), generics_a())
            }
        };

        let item = syn::ImplItem {
            ident: ident("url"),
            vis: syn::Visibility::Public,
            defaultness: syn::Defaultness::Final,
            attrs: vec![],
            node: syn::ImplItemKind::Method(
                syn::MethodSig {
                    unsafety: syn::Unsafety::Normal,
                    constness: syn::Constness::NotConst,
                    abi: None,
                    decl: fndecl,
                    generics: fngenerics,
                },
                syn::Block {
                    stmts: vec![self.body.into_stmt()],
                },
            ),
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
                Box::new(self.params_ty),
                vec![item],
            ),
        }
    }
}

impl<'a> From<(&'a (String, Endpoint), &'a (syn::Item, syn::Ty))> for Builder {
    fn from(value: (&'a (String, Endpoint), &'a (syn::Item, syn::Ty))) -> Self {
        let (&(_, ref endpoint), params) = value;
        let &(_, ref params_ty) = params;

        let bodies: Vec<syn::Block> = endpoint
            .url
            .paths
            .iter()
            .map(|p| ReplaceBuilder::new(p.split()).build())
            .collect();

        let match_expr = MatchBuilder::from((params, bodies)).build();

        Builder::new((*params_ty).to_owned(), match_expr)
    }
}

/// Builder for match statements over a request parameters enum.
struct MatchBuilder {
    url_params: syn::Ty,
    arms: Vec<syn::Arm>,
}

impl MatchBuilder {
    pub fn new(url_params: syn::Ty) -> Self {
        MatchBuilder {
            url_params,
            arms: vec![],
        }
    }

    pub fn with_variant(mut self, variant: &syn::Variant, body: &syn::Block) -> Self {
        let arm = self.match_arm(variant, body);
        self.arms.push(arm);

        self
    }

    /// Create a match arm for a unit or tuple variant.
    fn match_arm(&self, variant: &syn::Variant, body: &syn::Block) -> syn::Arm {
        let path = Self::match_path(self.url_params.to_owned(), variant);
        let fields = Self::match_fields(variant);

        let arm = match fields.len() {
            0 => syn::Pat::Path(None, path),
            _ => syn::Pat::TupleStruct(path, fields, None),
        };

        syn::Arm {
            attrs: vec![],
            pats: vec![arm],
            guard: None,
            body: Box::new(body.to_owned().into_expr()),
        }
    }

    /// Get the path for the enum variant to match.
    fn match_path(url_params: syn::Ty, variant: &syn::Variant) -> syn::Path {
        let mut ty = url_params.get_path().to_owned();

        // Remove lifetimes from the enum type.
        for segment in &mut ty.segments {
            segment.parameters = syn::PathParameters::none();
        }

        ty.segments
            .push(syn::PathSegment::from(variant.ident.to_string()));

        ty
    }

    /// Get the fields for the enum variant to match.
    fn match_fields(variant: &syn::Variant) -> Vec<syn::Pat> {
        match variant.data {
            syn::VariantData::Tuple(ref fields) => fields
                .iter()
                .map(|f| {
                    let path = f.ty.get_ident().into_rust_var();

                    syn::Pat::Ident(
                        syn::BindingMode::ByRef(syn::Mutability::Immutable),
                        ident(path),
                        None,
                    )
                })
                .collect(),
            syn::VariantData::Unit => vec![],
            _ => panic!("Only Unit and Tuple variants are supported."),
        }
    }

    pub fn build(self) -> syn::Expr {
        let to_match = path_none("self").into_expr();

        syn::ExprKind::Match(Box::new(to_match), self.arms).into()
    }
}

impl<'a> From<(&'a (syn::Item, syn::Ty), Vec<syn::Block>)> for MatchBuilder {
    fn from(value: (&'a (syn::Item, syn::Ty), Vec<syn::Block>)) -> Self {
        let (&(ref params_item, ref params_ty), ref bodies) = value;

        let mut builder = MatchBuilder::new(params_ty.to_owned());

        match params_item.node {
            syn::ItemKind::Enum(ref variants, _) => {
                for (variant, body) in variants.iter().zip(bodies.iter()) {
                    builder = builder.with_variant(variant, body);
                }
            }
            _ => panic!("expected syn::ItemKind::Enum"),
        };

        builder
    }
}

/// Builder for an efficient url value replacer.
///
/// The inputs are expected to be `AsRef<str>` and the output is a `UrlPath<'a>`.
struct ReplaceBuilder<'a> {
    url: Vec<PathPart<'a>>,
}

impl<'a> ReplaceBuilder<'a> {
    pub fn new(url: Vec<PathPart<'a>>) -> Self {
        ReplaceBuilder { url }
    }

    /// Build an allocated url from the path literals and params.
    fn build_owned(self) -> syn::Block {
        let lit_len_expr = Self::literal_length_expr(&self.url);

        let mut params_len_exprs = Self::parameter_length_exprs(&self.url);

        let mut len_exprs = vec![lit_len_expr];
        len_exprs.append(&mut params_len_exprs);

        let len_expr = Self::summed_length_expr(len_exprs);

        let url_ident = ident("url");
        let url_ty = ident(http::url::ident());

        let let_stmt = Self::let_url_stmt(url_ident.clone(), len_expr);

        let mut push_stmts = Self::push_part_stmts(url_ident.clone(), &self.url);

        let return_expr = syn::Stmt::Expr(Box::new(parse_expr(quote!(#url_ty ::from(#url_ident)))));

        let mut stmts = vec![let_stmt];

        stmts.append(&mut push_stmts);

        stmts.push(return_expr);

        syn::Block { stmts }
    }

    /// Build a non-allocated url from the path literals.
    fn build_borrowed(self) -> syn::Block {
        let path: Vec<&'a str> = self
            .url
            .iter()
            .map(|p| match *p {
                PathPart::Literal(p) => p,
                _ => panic!("Only PathPart::Literal is supported by a borrowed url."),
            })
            .collect();

        let path = path.join("");

        let lit = syn::Lit::Str(path, syn::StrStyle::Cooked);
        let url_ty = ident(http::url::ident());

        let expr = parse_expr(quote!(#url_ty ::from(#lit)));

        syn::Block {
            stmts: vec![syn::Stmt::Expr(Box::new(expr))],
        }
    }

    /// Get the number of chars in all literal parts for the url.
    fn literal_length_expr(url: &[PathPart<'a>]) -> syn::Expr {
        let len = url
            .iter()
            .filter_map(|p| match *p {
                PathPart::Literal(p) => Some(p),
                _ => None,
            })
            .fold(0, |acc, p| acc + p.len());

        syn::ExprKind::Lit(syn::Lit::Int(len as u64, syn::IntTy::Usize)).into()
    }

    /// Get an expression to find the number of chars in each parameter part for the url.
    fn parameter_length_exprs(url: &[PathPart<'a>]) -> Vec<syn::Expr> {
        url.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(
                    syn::ExprKind::MethodCall(
                        ident("len"),
                        vec![],
                        vec![syn::ExprKind::Path(None, path_none(p)).into()],
                    )
                    .into(),
                ),
                _ => None,
            })
            .collect()
    }

    /// Get an expression that is the binary addition of each of the given expressions.
    fn summed_length_expr(len_exprs: Vec<syn::Expr>) -> syn::Expr {
        match len_exprs.len() {
            1 => len_exprs.into_iter().next().unwrap(),
            _ => {
                let mut len_iter = len_exprs.into_iter();

                let first_expr = Box::new(len_iter.next().unwrap());

                *(len_iter.map(Box::new).fold(first_expr, |acc, p| {
                    Box::new(syn::ExprKind::Binary(syn::BinOp::Add, acc, p).into())
                }))
            }
        }
    }

    /// Get a statement to build a `String` with a capacity of the given expression.
    fn let_url_stmt(url_ident: syn::Ident, len_expr: syn::Expr) -> syn::Stmt {
        let string_with_capacity = syn::ExprKind::Call(
            Box::new(
                syn::ExprKind::Path(None, {
                    let mut method = path_none("String");
                    method
                        .segments
                        .push(syn::PathSegment::from("with_capacity"));
                    method
                })
                .into(),
            ),
            vec![len_expr],
        )
        .into();

        syn::Stmt::Local(Box::new(syn::Local {
            pat: Box::new(syn::Pat::Ident(
                syn::BindingMode::ByValue(syn::Mutability::Mutable),
                url_ident,
                None,
            )),
            ty: None,
            init: Some(Box::new(string_with_capacity)),
            attrs: vec![],
        }))
    }

    /// Get a list of statements that append each part to a `String` in order.
    fn push_part_stmts(url_ident: syn::Ident, url: &[PathPart<'a>]) -> Vec<syn::Stmt> {
        url.iter()
            .map(|p| match *p {
                PathPart::Literal(p) => {
                    let lit = syn::Lit::Str(p.to_string(), syn::StrStyle::Cooked);

                    syn::Stmt::Semi(Box::new(parse_expr(quote!(#url_ident.push_str(#lit)))))
                }
                PathPart::Param(p) => {
                    let ident = ident(p);

                    syn::Stmt::Semi(Box::new(parse_expr(
                        quote!(#url_ident.push_str(#ident.as_ref())),
                    )))
                }
            })
            .collect()
    }

    pub fn build(self) -> syn::Block {
        let has_params = self.url.iter().any(|p| match *p {
            PathPart::Param(_) => true,
            _ => false,
        });

        if has_params {
            self.build_owned()
        } else {
            self.build_borrowed()
        }
    }
}

impl<'a, I: IntoIterator<Item = PathPart<'a>>> From<I> for ReplaceBuilder<'a> {
    fn from(value: I) -> Self {
        ReplaceBuilder::new(value.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
    use super::*;
    use crate::{
        gen::endpoints::url_params,
        parse::{
            Method,
            Body,
            get_url,
        },
    };
    use syn;

    #[test]
    fn gen_url_match() {
        let params = url_params::Builder::new("RequestParams").with_param_set(vec![]).with_param_set(vec!["Index"]).with_param_set(vec!["Index", "Type", "Id"]).build();

        let bodies = vec![syn::Block { stmts: vec![] }, syn::Block { stmts: vec![] }, syn::Block { stmts: vec![] }];

        let result = MatchBuilder::from((&params, bodies)).build();

        let expected = quote!(match self {
            RequestParams::None => {}
            RequestParams::Index(ref index) => {}
            RequestParams::IndexTypeId(ref index, ref ty, ref id) => {}
        });

        ast_eq(expected, result);
    }

    #[test]
    fn gen_url_no_params() {
        let result = ReplaceBuilder::from(vec![PathPart::Literal("/_search")]).build();

        let expected = quote!({ UrlPath::from("/_search") });

        ast_eq(expected, result);
    }

    #[test]
    fn gen_url_with_params() {
        let result = ReplaceBuilder::from(vec![PathPart::Literal("/"), PathPart::Param("index"), PathPart::Literal("/_search/"), PathPart::Param("type")]).build();

        let expected = quote!({
            let mut url = String::with_capacity(10usize + index.len() + ty.len());
            url.push_str("/");
            url.push_str(index.as_ref());
            url.push_str("/_search/");
            url.push_str(ty.as_ref());

            UrlPath::from(url)
        });

        ast_eq(expected, result);
    }

    #[test]
    fn gen_url_method_no_params() {
        use syn;

        let result = Builder::new(ty("UrlParams"), syn::Block { stmts: vec![] }.into_expr()).build();

        let expected = quote!(
            impl UrlParams {
                pub fn url<'a>(self) -> UrlPath<'a> {
                    { }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_url_with_params_from_endpoint() {
        let endpoint = (
            "indices.exists_alias".to_string(),
            Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: Some(Body { description: String::new() }),
            },
        );
        let params = url_params::Builder::from(&endpoint).build();

        let result = Builder::from((&endpoint, &params)).build();

        let none_arm = quote!(
            IndicesExistsAliasUrlParams::None => {
                UrlPath::from("/_search")
            }
        );
        let index_arm = quote!(
            IndicesExistsAliasUrlParams::Index(ref index) => {
                let mut url = String::with_capacity(9usize + index.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/_search");

                UrlPath::from(url)
            }
        );
        let index_ty_arm = quote!(
            IndicesExistsAliasUrlParams::IndexType(ref index, ref ty) => {
                let mut url = String::with_capacity(10usize + index.len() + ty.len());
                url.push_str("/");
                url.push_str(index.as_ref());
                url.push_str("/");
                url.push_str(ty.as_ref());
                url.push_str("/_search");

                UrlPath::from(url)
            }
        );
        let expected = quote!(
            impl <'a> IndicesExistsAliasUrlParams<'a> {
                pub fn url(self) -> UrlPath<'a> {
                    match self {
                        #none_arm
                        #index_arm
                        #index_ty_arm
                    }
                }
            }
        );

        ast_eq(expected, result);
    }
}
