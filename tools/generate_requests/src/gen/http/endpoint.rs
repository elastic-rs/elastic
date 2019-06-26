use super::{
    body,
    method,
    url,
};
use crate::gen::helpers;
use quote;
use syn;

pub fn ident() -> &'static str {
    "Endpoint"
}

pub fn ty(body_generic: syn::Ty) -> syn::Ty {
    helpers::ty_path(ident(), vec![helpers::lifetime()], vec![body_generic])
}

pub fn tokens() -> quote::Tokens {
    let method_ty = method::ty();

    let request_ty = helpers::ty(ident());

    let url_ty = url::ty();

    let body_ty = body::ty();

    quote!(
        /// A general request type that all endpoints can be converted into.
        #[derive(Debug, Clone, PartialEq)]
        pub struct #request_ty<'a, #body_ty> {
            pub url: #url_ty,
            pub method: #method_ty,
            pub body: Option<#body_ty>
        }
    )
}
