use crate::gen::helpers;
use quote;
use syn;

pub fn ident() -> &'static str {
    "IntoEndpoint"
}

pub fn ty() -> syn::Ty {
    helpers::ty(ident())
}

pub fn tokens() -> quote::Tokens {
    quote!(
        /// Trait for converting a request into its endpoint.
        ///
        /// Like `From`, but the client API doesn't need to know the exact
        /// type to convert into.
        pub trait IntoEndpoint<'a> {
            type BodyType;
            fn into_endpoint(self) -> Endpoint<'a, Self::BodyType>;
        }
    )
}
