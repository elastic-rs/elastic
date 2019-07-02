use crate::gen::helpers;
use quote;
use syn;

pub fn ident() -> &'static str {
    "UrlPath"
}

pub fn ty() -> syn::Ty {
    helpers::ty_a(ident())
}

pub fn tokens() -> quote::Tokens {
    let url = ty();
    let ident = helpers::ident(ident());

    quote!(
        /// A wrapper around an owned or borrowed url path.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #url(Cow<'a, str>);

        impl <'a> From<&'a str> for #url {
            fn from(value: &'a str) -> #url {
                #ident (Cow::Borrowed(value))
            }
        }

        impl <'a> From<String> for #url {
            fn from(value: String) -> #url {
                #ident (Cow::Owned(value))
            }
        }

        impl <'a> Deref for #url {
            type Target = Cow<'a, str>;

            fn deref(&self) -> &Cow<'a, str> {
                &self.0
            }
        }
    )
}
