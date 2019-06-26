use crate::gen::helpers;
use inflector::Inflector;
use quote;

pub fn tokens(ty: &str) -> quote::Tokens {
    let ty = ty.to_pascal_case();
    let ty_fn = ty.to_snake_case();

    let ident = helpers::ty(&ty);
    let ty = helpers::ty_a(&ty);
    let ty_fn = helpers::ident(ty_fn);

    quote!(
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #ty(pub ::std::borrow::Cow<'a, str>);

        pub fn #ty_fn<'a, I>(value: I) -> #ty where I: Into<#ty> {
            value.into()
        }

        impl<'a> #ty {
            pub fn to_owned(&self) -> #ident<'static> {
                #ident(::std::borrow::Cow::Owned(match self.0 {
                    ::std::borrow::Cow::Owned(ref value) => value.clone(),
                    ::std::borrow::Cow::Borrowed(value) => value.to_owned(),
                }))
            }
        }

        impl <'a> ::std::fmt::Display for #ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self.0 {
                    ::std::borrow::Cow::Owned(ref value) => ::std::fmt::Display::fmt(value, f),
                    ::std::borrow::Cow::Borrowed(value) => ::std::fmt::Display::fmt(value, f),
                }
            }
        }

        impl<'a> PartialEq<str> for #ty {
            fn eq(&self, other: &str) -> bool {
                self.0.eq(other)
            }
        }

        impl<'a, 'b> PartialEq<#ty> for &'b str {
            fn eq(&self, other: &#ty) -> bool {
                self.eq(&other.0)
            }
        }

        impl <'a> From<&'a str> for #ty {
            fn from(value: &'a str) -> #ty {
                #ident(::std::borrow::Cow::Borrowed(value))
            }
        }

        impl <'a> From<String> for #ty {
            fn from(value: String) -> #ty {
                #ident(::std::borrow::Cow::Owned(value))
            }
        }

        impl <'a> From<&'a String> for #ty {
            fn from(value: &'a String) -> #ty {
                #ident(::std::borrow::Cow::Borrowed(&**value))
            }
        }

        impl <'a> From<#ty> for ::std::borrow::Cow<'a, str> {
            fn from(value: #ty) -> ::std::borrow::Cow<'a, str> {
                value.0
            }
        }

        impl <'a> From<#ty> for String {
            fn from(value: #ty) -> String {
                value.0.into_owned()
            }
        }

        impl <'a> ::std::ops::Deref for #ty {
            type Target = str;

            fn deref(&self) -> &str {
                &self.0
            }
        }
    )
}
