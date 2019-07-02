pub mod endpoints;
pub mod http;
pub mod params;

mod helpers {
    use inflector::Inflector;
    use quote;
    use syn;

    fn sanitise_ident(ident: &str) -> &str {
        match ident {
            "type" => "ty",
            i => i,
        }
    }

    /// Build a sanitised `Ident`.
    pub fn ident<I: AsRef<str>>(ident: I) -> syn::Ident {
        let ident = ident.as_ref();

        syn::Ident::new(sanitise_ident(ident))
    }

    /// Build a literal
    pub fn lit<I: Into<String>>(lit: I) -> syn::Lit {
        syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
    }

    /// A standard `'a` lifetime.
    pub fn lifetime() -> syn::Lifetime {
        syn::Lifetime {
            ident: syn::Ident::new("'a"),
        }
    }

    /// Generics with a standard `'a` lifetime.
    pub fn generics_a() -> syn::Generics {
        generics(vec![lifetime()], vec![])
    }

    /// Generics with no parameters.
    pub fn generics_none() -> syn::Generics {
        generics(vec![], vec![])
    }

    /// Generics with the given lifetimes and type bounds.
    pub fn generics(lifetimes: Vec<syn::Lifetime>, types: Vec<syn::TyParam>) -> syn::Generics {
        syn::Generics {
            lifetimes: lifetimes
                .into_iter()
                .map(|l| syn::LifetimeDef {
                    attrs: vec![],
                    lifetime: l,
                    bounds: vec![],
                })
                .collect(),
            ty_params: types,
            where_clause: syn::WhereClause::none(),
        }
    }

    /// AST for a path type with a `'a` lifetime.
    pub fn ty_a(ty: &str) -> syn::Ty {
        ty_path(ty, vec![lifetime()], vec![])
    }

    /// AST for a simple path type.
    pub fn ty(ty: &str) -> syn::Ty {
        ty_path(ty, vec![], vec![])
    }

    /// AST for a simple type param.
    pub fn ty_param(ty: &str, bounds: Vec<syn::TyParamBound>) -> syn::TyParam {
        syn::TyParam {
            attrs: vec![],
            ident: ident(ty),
            bounds: bounds,
            default: None,
        }
    }

    /// AST for a generic type param bound.
    pub fn ty_bound(trait_ref: syn::Path) -> syn::TyParamBound {
        syn::TyParamBound::Trait(
            syn::PolyTraitRef {
                bound_lifetimes: vec![],
                trait_ref: trait_ref,
            },
            syn::TraitBoundModifier::None,
        )
    }

    /// AST for a path type with lifetimes and type parameters.
    pub fn ty_path(ty: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Ty>) -> syn::Ty {
        syn::Ty::Path(None, path(ty, lifetimes, types))
    }

    /// AST for a simple path variable.
    pub fn path_none(path_ident: &str) -> syn::Path {
        path(path_ident, vec![], vec![])
    }

    /// AST for a path variable.
    pub fn path(path: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Ty>) -> syn::Path {
        path_segments(vec![(path, lifetimes, types)])
    }

    /// AST for a path variable.
    pub fn path_segments(paths: Vec<(&str, Vec<syn::Lifetime>, Vec<syn::Ty>)>) -> syn::Path {
        syn::Path {
            global: false,
            segments: paths
                .into_iter()
                .map(|(path, lifetimes, types)| syn::PathSegment {
                    ident: syn::Ident::new(sanitise_ident(path)),
                    parameters: syn::PathParameters::AngleBracketed(
                        syn::AngleBracketedParameterData {
                            lifetimes: lifetimes,
                            types: types,
                            bindings: vec![],
                        },
                    ),
                })
                .collect(),
        }
    }

    /// AST for a simple method call.
    pub fn method(method: &str, args: Vec<&str>) -> syn::Expr {
        syn::ExprKind::MethodCall(
            ident(method),
            vec![],
            args.iter().map(|a| path_none(a).into_expr()).collect(),
        )
        .into()
    }

    /// AST for an outer doc comment.
    pub fn doc(comment: String) -> syn::Attribute {
        syn::Attribute {
            style: syn::AttrStyle::Outer,
            value: syn::MetaItem::NameValue(ident("doc"), lit(comment)),
            is_sugared_doc: true,
        }
    }

    /// Parse quoted tokens to an expression.
    pub fn parse_expr(input: quote::Tokens) -> syn::Expr {
        syn::parse_expr(input.to_string().as_ref()).unwrap()
    }

    /// Parse a name to a Rust PascalCase type name.
    pub trait IntoRustTypeName {
        fn into_rust_type(self) -> String;
    }

    impl<'a> IntoRustTypeName for &'a str {
        fn into_rust_type(self) -> String {
            str::replace(self, ".", "_").to_pascal_case()
        }
    }

    impl<'a> IntoRustTypeName for &'a syn::Ident {
        fn into_rust_type(self) -> String {
            (&self.to_string()).into_rust_type()
        }
    }

    /// Parse a name to a Rust snake_case variable name.
    pub trait IntoRustVarName {
        fn into_rust_var(self) -> String;
    }

    impl<'a> IntoRustVarName for &'a str {
        fn into_rust_var(self) -> String {
            let ident = self.split(".").last().unwrap().to_snake_case();

            sanitise_ident(&ident).to_string()
        }
    }

    impl<'a> IntoRustVarName for &'a syn::Ident {
        fn into_rust_var(self) -> String {
            (&self.to_string()).into_rust_var()
        }
    }

    pub trait GetPath {
        fn get_path(&self) -> &syn::Path;
    }

    impl GetPath for syn::Ty {
        fn get_path(&self) -> &syn::Path {
            match self {
                &syn::Ty::Path(_, ref p) => &p,
                _ => panic!("Only path types are supported."),
            }
        }
    }

    impl GetPath for syn::Path {
        fn get_path(&self) -> &syn::Path {
            &self
        }
    }

    pub trait GetIdent {
        fn get_ident(&self) -> &syn::Ident;
    }

    impl<T: GetPath> GetIdent for T {
        fn get_ident(&self) -> &syn::Ident {
            &self.get_path().segments[0].ident
        }
    }

    pub trait HasLifetime {
        fn has_lifetime(&self) -> bool;
    }

    impl<T: GetPath> HasLifetime for T {
        fn has_lifetime(&self) -> bool {
            match &self.get_path().segments[0].parameters {
                &syn::PathParameters::AngleBracketed(ref params) => params.lifetimes.len() > 0,
                _ => false,
            }
        }
    }

    /// Helper for wrapping a value as a quotable statement.
    pub trait IntoTy {
        fn into_ty(self) -> syn::Ty;
    }

    impl<T: GetPath> IntoTy for T {
        fn into_ty(self) -> syn::Ty {
            syn::Ty::Path(None, self.get_path().to_owned())
        }
    }

    /// Helper for wrapping a value as a quotable statement.
    pub trait IntoStmt {
        fn into_stmt(self) -> syn::Stmt;
    }

    impl IntoStmt for syn::Item {
        fn into_stmt(self) -> syn::Stmt {
            syn::Stmt::Item(Box::new(self))
        }
    }

    impl IntoStmt for syn::Expr {
        fn into_stmt(self) -> syn::Stmt {
            syn::Stmt::Expr(Box::new(self))
        }
    }

    /// Helper for wrapping a value as a quotable expression.
    pub trait IntoExpr {
        fn into_expr(self) -> syn::Expr;
    }

    impl IntoExpr for syn::Path {
        fn into_expr(self) -> syn::Expr {
            syn::ExprKind::Path(None, self).into()
        }
    }

    impl IntoExpr for syn::Block {
        fn into_expr(self) -> syn::Expr {
            syn::ExprKind::Block(syn::Unsafety::Normal, self).into()
        }
    }

    #[cfg(test)]
    pub fn ast_eq<T: quote::ToTokens>(expected: quote::Tokens, actual: T) {
        assert_eq!(expected.to_string(), quote!(#actual).to_string());
    }
}
