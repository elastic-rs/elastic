use syn;
use ::parse::Endpoint;
use super::helpers::*;

pub struct RequestIntoHttpRequestBuilder {
    req_ty: syn::Ty,
    has_body: bool,
}

impl RequestIntoHttpRequestBuilder {
    pub fn new(has_body: bool, request_ty: syn::Ty) -> Self {
        RequestIntoHttpRequestBuilder {
            req_ty: request_ty,
            has_body: has_body,
        }
    }

    pub fn build(self) -> syn::Item {
        let req_ty = {
            let mut path = self.req_ty.get_path().to_owned();
            path.segments[0].parameters = syn::PathParameters::none();
            path.into_ty()
        };

        let body = {
            if self.has_body {
                quote!(Some(&self.body))
            } else {
                quote!(None)
            }
        };

        parse_item(quote!(
            impl <'a, 'b: 'a> Into<HttpRequest<'a>> for &'a #req_ty<'b> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url(),
                        method: HttpMethod::Post,
                        body: #body
                    }
                }
            }
        ))
    }
}

impl<'a> From<(&'a (String, Endpoint), &'a syn::Ty)> for RequestIntoHttpRequestBuilder {
    fn from(value: (&'a (String, Endpoint), &'a syn::Ty)) -> Self {
        let (&(_, ref endpoint), ref req_ty) = value;

        let has_body = endpoint.body.is_some();

        RequestIntoHttpRequestBuilder::new(has_body, (*req_ty).to_owned())
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
        let req_ty = ty_a("IndicesExistsAliasRequestParams");

        let result = RequestIntoHttpRequestBuilder::from((&endpoint, &req_ty)).build();

        let expected = quote!(
            impl <'a, 'b: 'a> Into<HttpRequest<'a> > for &'a IndicesExistsAliasRequestParams<'b> {
                fn into(self) -> HttpRequest<'a> {
                    HttpRequest {
                        url: self.url(),
                        method: HttpMethod::Post,
                        body: Some(&self.body)
                    }
                }
            }
        );

        ast_eq(expected, result);
    }
}
