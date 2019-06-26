use crate::gen::helpers;
use quote;
use syn;

pub fn ident() -> &'static str {
    "B"
}

pub fn default_ident() -> &'static str {
    "DefaultBody"
}

pub fn ty() -> syn::Ty {
    helpers::ty(ident())
}

pub fn tokens() -> quote::Tokens {
    let default_body = helpers::ident(default_ident());

    quote!(
        /// A default body type.
        pub type #default_body = &'static [u8];

        /// A convenience method for a default, empty body.
        /// This method doesn't allocate.
        pub fn empty_body() -> #default_body {
            &[]
        }
    )
}
