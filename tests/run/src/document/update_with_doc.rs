use elastic::error::Error;
use elastic::prelude::*;
use futures::Future;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct UpdateWithDoc;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_doc_idx")]
pub struct Doc {
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

impl IntegrationTest for UpdateWithDoc {
    type Response = (UpdateResponse, GetResponse<Doc>);

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "update_with_doc"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index(Doc::static_index()).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute an update request against that index using a new document
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let index_res = client.document().index(doc()).params_fluent(|p| p.url_param("refresh", true)).send();

        let update_res = client
            .document::<Doc>()
            .update(ID)
            .doc(json!({
                "title": EXPECTED_TITLE.to_owned(),
            }))
            .params_fluent(|p| p.url_param("refresh", true))
            .send();

        let get_res = client.document().get(ID).send();

        Box::new(index_res.and_then(|_| update_res).and_then(|update| get_res.map(|get| (update, get))))
    }

    // Ensure the response contains the expected document
    fn assert_ok(&self, res: &Self::Response) -> bool {
        let update = &res.0;
        let get = &res.1;

        let updated = update.updated();
        let correct_version = update.version() == Some(2);
        let correct_title = get.document().map(|doc| doc.title.as_ref()) == Some(EXPECTED_TITLE);

        updated && correct_version && correct_title
    }
}
