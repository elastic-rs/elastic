use elastic::{
    client::responses::bulk::OkItem,
    error::Error,
    prelude::*,
};
use futures::{
    stream,
    Future,
    Sink,
    Stream,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "bulk_stream")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

#[derive(Clone, Debug)]
pub struct BulkResult {
    requests: u32,
    ops: Vec<OkItem>,
}

test! {
    const description: &'static str = "simple index streaming";

    type Response = BulkResult;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Stream some bulk operations
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let (bulk_stream, bulk_responses) = client.bulk_stream().build();

        let ops = (0..20)
            .into_iter()
            .map(|i| bulk().index(Doc { id: i.to_string() }));

        let req_future = bulk_stream.send_all(stream::iter_ok(ops));

        let res_future = bulk_responses.fold(
            BulkResult {
                requests: 0,
                ops: Vec::new(),
            },
            |mut res, bulk| {
                res.requests += 1;
                res.ops.extend(bulk.into_iter().filter_map(Result::ok));

                Ok(res)
            },
        );

        Box::new(req_future.join(res_future).map(|(_, ops)| ops))
    }

    // Ensure the we see 20 successful items returned from the bulk stream but less than 20 requests sent
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.requests < 20 && res.ops.len() == 20
    }
}
