use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{empty_body, DefaultBody, IntoBody, Index, IndicesCreateRequest,
                       RequestBuilder, RawRequestBuilder};
use client::responses::CommandResponse;

/** 
A builder for a [`Client.create_index`][Client.create_index] request. 

[Client.create_index]: ../struct.Client.html#method.create_index
*/
pub struct CreateIndexRequestBuilder<TBody> {
    index: Index<'static>,
    body: TBody,
}

impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /** 
    Create a [`RequestBuilder` for a create index request][RequestBuilder.create_index].

    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = ClientBuilder::new().build().unwrap();
    let my_index = index("myindex");

    let response = client.create_index(my_index).send().unwrap();

    assert!(response.acknowledged);
    ```

    Create an index with settings and document mappings for a [`DocumentType`][documents-mod] called `MyType`:

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
    # let client = ClientBuilder::new().build().unwrap();
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

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [RequestBuilder.create_index]: requests/struct.RequestBuilder.html#create-index-builder
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn create_index(&self, index: Index<'static>) -> RequestBuilder<TSender, CreateIndexRequestBuilder<DefaultBody>> {
        RequestBuilder::new(self.clone(),
                            None,
                            CreateIndexRequestBuilder {
                                index: index,
                                body: empty_body(),
                            })
    }
}

impl<TBody> CreateIndexRequestBuilder<TBody> {
    fn into_request(self) -> IndicesCreateRequest<'static, TBody> {
        IndicesCreateRequest::for_index(self.index, self.body)
    }
}

/** 
# Create index builder

A request builder for a [Create Index][docs-create-index] request.

Call [`Client.create_index`][Client.create_index] to get a `RequestBuilder` for a create index request.

[Client.create_index]: ../struct.Client.html#method.create_index
[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
*/
impl<TSender, TBody> RequestBuilder<TSender, CreateIndexRequestBuilder<TBody>>
    where TSender: Sender,
          TBody: Into<TSender::Body>
{
    /** 
    Set the body for the search request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self,
                          body: TNewBody)
                          -> RequestBuilder<TSender, CreateIndexRequestBuilder<TNewBody>>
        where TNewBody: Into<TSender::Body>
    {
        RequestBuilder::new(self.client,
                            self.params,
                            CreateIndexRequestBuilder {
                                index: self.req.index,
                                body: body,
                            })
    }
}

impl<TBody> RequestBuilder<SyncSender, CreateIndexRequestBuilder<TBody>>
    where TBody: Into<<SyncSender as Sender>::Body>
{
    /** Send the create index request. */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.req.into_request();

        RequestBuilder::new(self.client, self.params, RawRequestBuilder::new(req))
            .send_raw()?
            .into_response()
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
