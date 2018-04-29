use syn;
use quote;
use parse::{Endpoint, Method};
use super::types;
use super::helpers::*;

pub struct RequestIntoEndpointBuilder {
    req_ty: syn::Ty,
    has_body: bool,
    http_verb: Method,
}

impl RequestIntoEndpointBuilder {
    pub fn new(http_verb: Method, has_body: bool, request_ty: syn::Ty) -> Self {
        RequestIntoEndpointBuilder {
            req_ty: request_ty,
            has_body: has_body,
            http_verb: http_verb,
        }
    }

    pub fn build(self) -> quote::Tokens {
        let req_ty = self.req_ty;
        let method_ty = types::request::method_ty();

        let method = match self.http_verb {
            Method::Get => quote!(#method_ty ::GET),
            Method::Post => quote!(#method_ty ::POST),
            Method::Put => quote!(#method_ty ::PUT),
            Method::Delete => quote!(#method_ty ::DELETE),
            Method::Head => quote!(#method_ty ::HEAD),
            Method::Patch => quote!(#method_ty ::PATCH),
        };

        let endpoint_ty = ident(types::request::req_ident());

        if self.has_body {
            let generic_body = ident(types::body::ident());
            quote!(
                impl <'a, #generic_body> Into<#endpoint_ty<'a, #generic_body> > for #req_ty {
                    fn into(self) -> #endpoint_ty<'a, #generic_body> {
                        #endpoint_ty {
                            url: self.url,
                            method: #method,
                            body: Some(self.body)
                        }
                    }
                }
            )
        } else {
            let default_body = ident(types::body::default_ident());

            quote!(
                impl <'a> Into<#endpoint_ty<'a, #default_body> > for #req_ty {
                    fn into(self) -> #endpoint_ty<'a, #default_body> {
                        #endpoint_ty {
                            url: self.url,
                            method: #method,
                            body: None
                        }
                    }
                }
            )
        }
    }
}

impl<'a> From<(&'a (String, Endpoint), &'a syn::Ty)> for RequestIntoEndpointBuilder {
    fn from(value: (&'a (String, Endpoint), &'a syn::Ty)) -> Self {
        let (&(_, ref endpoint), ref req_ty) = value;

        let has_body = endpoint.has_body();
        let verb = endpoint.methods[0];

        RequestIntoEndpointBuilder::new(verb, has_body, (*req_ty).to_owned())
    }
}

#[cfg(test)]
mod tests {
    use parse::*;
    use super::*;

    #[test]
    fn gen_into_http_req_with_body() {
        let endpoint = (
            "indices.exists_alias".to_string(),
            Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: Some(Body {
                    description: String::new(),
                }),
            },
        );
        let req_ty = ty_path("Request", vec![lifetime()], vec![types::body::ty()]);

        let result = RequestIntoEndpointBuilder::from((&endpoint, &req_ty)).build();

        let expected = quote!(
            impl <'a, B> Into<Endpoint<'a, B> > for Request<'a, B> {
                fn into(self) -> Endpoint<'a, B> {
                    Endpoint {
                        url: self.url,
                        method: Method::GET,
                        body: Some(self.body)
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_into_http_req_no_body() {
        let endpoint = (
            "indices.exists_alias".to_string(),
            Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: None,
            },
        );
        let req_ty = ty_a("Request");

        let result = RequestIntoEndpointBuilder::from((&endpoint, &req_ty)).build();

        let expected = quote!(
            impl <'a> Into<Endpoint<'a, DefaultBody> > for Request<'a> {
                fn into(self) -> Endpoint<'a, DefaultBody> {
                    Endpoint {
                        url: self.url,
                        method: Method::GET,
                        body: None
                    }
                }
            }
        );

        ast_eq(expected, result);
    }
}
