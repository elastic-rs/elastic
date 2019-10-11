use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_doc_source_idx")]
pub struct Doc {
    #[elastic(id)]
    id: String,
    title: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_doc_source_idx")]
pub struct UpdatedDoc {
    #[elastic(id)]
    id: String,
    title: String,
}

const EXPECTED_TITLE: &'static str = "Edited title";
const ID: &'static str = "1";

fn doc() -> Doc {
    Doc {
        id: ID.to_owned(),
        title: "Not edited title".to_owned(),
    }
}

test! {
    const description: &'static str = "update and return source";

    type Response = UpdateResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Execute an update request against that index using a new document & request
    // that the updated document's `source` be returned with the response.
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let index_res = client
            .document()
            .index(doc())
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let update_res = client
            .document::<Doc>()
            .update(ID)
            .doc(json!({
                "title": EXPECTED_TITLE.to_owned(),
            }))
            .source()
            .send();

        Box::new(
            index_res
                .and_then(|_| update_res)
                .map(|update| update)
        )
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let updated = res.updated();
        let correct_version = res.version() == Some(2);
        let correct_title = res.into_document::<UpdatedDoc>().unwrap().title == EXPECTED_TITLE;

        updated && correct_version && correct_title
    }
}
