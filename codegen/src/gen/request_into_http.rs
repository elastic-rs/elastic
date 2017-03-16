use syn;
use quote;
use ::parse::{Endpoint, HttpMethod};
use super::types;
use super::helpers::*;

pub struct RequestIntoHttpRequestBuilder {
    req_ty: syn::Ty,
    has_body: bool,
    http_verb: HttpMethod,
}

impl RequestIntoHttpRequestBuilder {
    pub fn new(http_verb: HttpMethod, has_body: bool, request_ty: syn::Ty) -> Self {
        RequestIntoHttpRequestBuilder {
            req_ty: request_ty,
            has_body: has_body,
            http_verb: http_verb,
        }
    }

    pub fn build(self) -> quote::Tokens {
        let req_ty = self.req_ty;

        let method = match self.http_verb {
            HttpMethod::Get => quote!(HttpMethod::Get),
            HttpMethod::Post => quote!(HttpMethod::Post),
            HttpMethod::Put => quote!(HttpMethod::Put),
            HttpMethod::Delete => quote!(HttpMethod::Delete),
            HttpMethod::Head => quote!(HttpMethod::Head),
            HttpMethod::Patch => quote!(HttpMethod::Patch),
        };

        if self.has_body {
            let generic_body = ident(types::body::generic_ident());
            quote!(
                impl <'a, #generic_body> Into<HttpRequest<'a, #generic_body> > for #req_ty {
                    fn into(self) -> HttpRequest<'a, #generic_body> {
                        HttpRequest {
                            url: self.url,
                            method: #method,
                            body: Some(self.body)
                        }
                    }
                }
            )
        }
        else {
            let default_body = ident(types::body::default_ident());

            quote!(
                impl <'a> Into<HttpRequest<'a, #default_body> > for #req_ty {
                    fn into(self) -> HttpRequest<'a, #default_body> {
                        HttpRequest {
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

impl<'a> From<(&'a (String, Endpoint), &'a syn::Ty)> for RequestIntoHttpRequestBuilder {
    fn from(value: (&'a (String, Endpoint), &'a syn::Ty)) -> Self {
        let (&(_, ref endpoint), ref req_ty) = value;

        let has_body = endpoint.has_body();
        let verb = endpoint.methods[0];

        RequestIntoHttpRequestBuilder::new(verb, has_body, (*req_ty).to_owned())
    }
}

#[cfg(test)]
mod tests {
    use ::parse::*;
    use super::*;

    #[test]
    fn gen_into_http_req_with_body() {
        let endpoint = ("indices.exists_alias".to_string(),
                        Endpoint {
            documentation: String::new(),
            methods: vec![HttpMethod::Get],
            url: get_url(),
            body: Some(Body { description: String::new() }),
        });
        let req_ty = ty_path("Request", vec![lifetime()], vec![ty(::gen::types::body::generic_ident())]);

        let result = RequestIntoHttpRequestBuilder::from((&endpoint, &req_ty)).build();

        let expected = quote!(
            impl <'a, R> Into<HttpRequest<'a, R> > for Request<'a, R> {
                fn into(self) -> HttpRequest<'a, R> {
                    HttpRequest {
                        url: self.url,
                        method: HttpMethod::Get,
                        body: Some(self.body)
                    }
                }
            }
        );

        ast_eq(expected, result);
    }

    #[test]
    fn gen_into_http_req_no_body() {
        let endpoint = ("indices.exists_alias".to_string(),
                        Endpoint {
            documentation: String::new(),
            methods: vec![HttpMethod::Get],
            url: get_url(),
            body: None,
        });
        let req_ty = ty_a("Request");

        let result = RequestIntoHttpRequestBuilder::from((&endpoint, &req_ty)).build();

        let expected = quote!(
            impl <'a> Into<HttpRequest<'a, DefaultBody> > for Request<'a> {
                fn into(self) -> HttpRequest<'a, DefaultBody> {
                    HttpRequest {
                        url: self.url,
                        method: HttpMethod::Get,
                        body: None
                    }
                }
            }
        );

        ast_eq(expected, result);
    }
}
