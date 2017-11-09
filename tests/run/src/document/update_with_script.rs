use futures::Future;
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct UpdateWithScript;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    id: i32,
    title: String,
}

const INDEX: &'static str = "update_doc_script_idx";
const EXPECTED_TITLE: &'static str = "Edited title";
const ID: i32 = 1;

fn doc() -> Doc {
    Doc {
        id: ID,
        title: "Not edited title".to_owned(),
    }
}

impl IntegrationTest for UpdateWithScript {
    type Response = (UpdateResponse, GetResponse<Doc>);

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "update_with_script"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index_delete(index(INDEX)).send();

        let index_res = client
            .document_index(index(INDEX), id(ID), doc())
            .params(|p| p.url_param("refresh", true))
            .send();

        Box::new(delete_res.then(|_| index_res).map(|_| ()))
    }

    // Execute an update request against that index using a script
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let update_res = client
            .document_update::<Doc>(index(INDEX), id(ID))
            .script_fluent(
                "ctx._source.title = params.newTitle",
                |s| s.param("newTitle", EXPECTED_TITLE),
            )
            .params(|p| p.url_param("refresh", true))
            .send();

        let get_res = client.document_get(index(INDEX), id(ID)).send();

        Box::new(update_res.and_then(|update| get_res.map(|get| (update, get))))
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
