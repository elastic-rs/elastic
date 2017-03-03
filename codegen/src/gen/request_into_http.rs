use syn;
use quote;
use ::parse::{Endpoint, HttpMethod};
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
        let req_ty = {
            let mut path = self.req_ty.get_path().to_owned();
            path.segments[0].parameters = syn::PathParameters::none();
            path.into_ty()
        };

        let (brw_body, owned_body) = {
            if self.has_body {
                (quote!(Some(self.body.as_ref().into())), quote!(Some(self.body)))
            } else {
                (quote!(None), quote!(None))
            }
        };

        let method = match self.http_verb {
            HttpMethod::Get => quote!(HttpMethod::Get),
            HttpMethod::Post => quote!(HttpMethod::Post),
            HttpMethod::Put => quote!(HttpMethod::Put),
            HttpMethod::Delete => quote!(HttpMethod::Delete),
            HttpMethod::Head => quote!(HttpMethod::Head),
            HttpMethod::Patch => quote!(HttpMethod::Patch),
        };

        let brw_item = quote!(
            impl <'a, 'b: 'a> Into<HttpRequest<'a> > for &'a #req_ty<'b> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url.as_ref().into(),
                        method: #method,
                        body: #brw_body
                    }
                }
            }
        );

        let owned_item = quote!(
            impl <'a> Into<HttpRequest<'a> > for #req_ty<'a> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url,
                        method: #method,
                        body: #owned_body
                    }
                }
            }
        );

        quote!(
            #brw_item

            #owned_item
        )
    }
}

impl<'a> From<(&'a (String, Endpoint), &'a syn::Ty)> for RequestIntoHttpRequestBuilder {
    fn from(value: (&'a (String, Endpoint), &'a syn::Ty)) -> Self {
        let (&(_, ref endpoint), ref req_ty) = value;

        let has_body = endpoint.body.is_some();
        let verb = endpoint.methods[0];

        RequestIntoHttpRequestBuilder::new(verb, has_body, (*req_ty).to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_into_http_req() {
        use ::parse::*;

        let endpoint = ("indices.exists_alias".to_string(),
                        Endpoint {
            documentation: String::new(),
            methods: vec![HttpMethod::Get],
            url: get_url(),
            body: Some(Body { description: String::new() }),
        });
        let req_ty = ty_a("IndicesExistsAliasRequest");

        let result = RequestIntoHttpRequestBuilder::from((&endpoint, &req_ty)).build();

        let brw_item = quote!(
            impl <'a, 'b: 'a> Into<HttpRequest<'a> > for &'a IndicesExistsAliasRequest<'b> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url.as_ref().into(),
                        method: HttpMethod::Get,
                        body: Some(self.body.as_ref().into())
                    }
                }
            }
        );

        let owned_item = quote!(
            impl <'a> Into<HttpRequest<'a> > for IndicesExistsAliasRequest<'a> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url,
                        method: HttpMethod::Get,
                        body: Some(self.body)
                    }
                }
            }
        );

        let expected = quote!(
            #brw_item

            #owned_item
        );

        ast_eq(expected, result);
    }
}
