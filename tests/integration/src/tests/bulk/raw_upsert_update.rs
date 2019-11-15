use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    #[elastic(id)]
    id: String,
    title: Option<String>,
    timestamp: Option<Date<DefaultDateMapping>>,
}

const INDEX: &str = "raw_bulk_upsert_update";
const ID: &str = "1";

fn doc() -> Doc {
    Doc {
        id: ID.to_owned(),
        title: Some("A document title".to_owned()),
        timestamp: Some(Date::build(2017, 3, 24, 13, 44, 0, 0)),
    }
}

test! {
    const description: &'static str = "raw upsert then update, then get";

    type Response = BulkResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Upsert a document, then update it, then get it
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let bulk_res = client
            .bulk()
            .push(
                bulk_raw()
                    .update(doc())
                    .doc_as_upsert()
                    .index(INDEX)
                    .ty(Doc::static_ty())
                    .id(ID),
            )
            .send();

        let update_res = client
            .bulk()
            .push(
                bulk_raw()
                    .update(Doc {
                        id: ID.to_owned(),
                        title: Some("New Title".to_owned()),
                        timestamp: None,
                    })
                    .index(INDEX)
                    .ty(Doc::static_ty())
                    .id(ID),
            )
            .send();

        Box::new(bulk_res.and_then(|_| update_res))
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let update = res.iter().next().unwrap().unwrap();

        update.action() == BulkAction::Update && !update.created()
    }
}
