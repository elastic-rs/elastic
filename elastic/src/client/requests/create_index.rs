use error::*;
use client::{into_response, Client};
use client::requests::{empty_body, DefaultBody, IntoBody, Index, IndicesCreateRequest,
                       RequestBuilder};
use client::responses::CommandResponse;

/** A builder for a [`create_index`]() request. */
pub struct CreateIndexRequestBuilder<TBody> {
    index: Index<'static>,
    body: TBody,
}

impl Client {
    /** 
    Create a [`RequestBuilder` for a create index request]().

    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = Client::new(RequestParams::default()).unwrap();
    let my_index = index("myindex");

    let response = client.create_index(my_index).send().unwrap();

    assert!(response.acknowledged);
    ```

    Create an index with settings and document mappings for a [`DocumentType`]() called `MyType`:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # extern crate elastic;
    # use elastic::prelude::*;
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # fn main() {
    # let client = Client::new(RequestParams::default()).unwrap();
    let my_index = index("myindex");

    let body = json!({
        "settings": {
            "index": {
                "number_of_shards": 3,
                "number_of_replicas": 2
            }
        },
        "mappings": {
            MyType::name(): IndexDocumentMapping::from(MyType::mapping())
        }
    });

    let response = client.create_index(my_index)
                         .body(body.to_string())
                         .send()
                         .unwrap();

    assert!(response.acknowledged);
    # }
    ```

    For more details on document types and mapping, see the [`types`]() module.
    */
    pub fn create_index<'a>
        (&'a self,
         index: Index<'static>)
         -> RequestBuilder<'a, CreateIndexRequestBuilder<DefaultBody>, DefaultBody> {
        RequestBuilder::new(&self,
                            None,
                            CreateIndexRequestBuilder {
                                index: index,
                                body: empty_body(),
                            })
    }
}

impl<TBody> CreateIndexRequestBuilder<TBody>
    where TBody: IntoBody
{
    fn into_request(self) -> IndicesCreateRequest<'static, TBody> {
        IndicesCreateRequest::for_index(self.index, self.body)
    }
}

/** 
# Create index builder

A request builder for a [`Create Index`]() request.

Call [`Client.create_index`]() to get a `RequestBuilder` for a create index request.
*/
impl<'a, TBody> RequestBuilder<'a, CreateIndexRequestBuilder<TBody>, TBody>
    where TBody: IntoBody
{
    /** 
    Set the body for the search request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self,
                          body: TNewBody)
                          -> RequestBuilder<'a, CreateIndexRequestBuilder<TNewBody>, TNewBody>
        where TNewBody: IntoBody
    {
        RequestBuilder::new(self.client,
                            self.params,
                            CreateIndexRequestBuilder {
                                index: self.req.index,
                                body: body,
                            })
    }

    /** Send the search request. */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.req.into_request();

        RequestBuilder::new(self.client, self.params, req)
            .send_raw()
            .and_then(into_response)
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn default_request() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.create_index(index("testindex")).req.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .create_index(index("testindex"))
            .body("{}")
            .req
            .into_request();

        assert_eq!("{}", req.body);
    }
}
