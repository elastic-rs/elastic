use crate::gen::helpers;

pub fn ident() -> &'static str {
    "Method"
}

pub fn ty() -> syn::Ty {
    helpers::ty(ident())
}
