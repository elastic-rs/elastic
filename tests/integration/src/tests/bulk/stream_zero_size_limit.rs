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
#[elastic(index = "bulk_stream_zero_size")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

test! {
    const description: &'static str = "stream with zero byte request size";

    type Response = Vec<OkItem>;

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
        let (bulk_stream, bulk_responses) = client.bulk_stream().body_size_bytes(0).build();

        let ops = (0..20)
            .into_iter()
            .map(|i| bulk().index(Doc { id: i.to_string() }));

        let req_future = bulk_stream.send_all(stream::iter_ok(ops));

        let res_future = bulk_responses.fold(Vec::new(), |mut ops, bulk| {
            ops.extend(bulk.into_iter().filter_map(Result::ok));

            Ok(ops)
        });

        Box::new(req_future.join(res_future).map(|(_, ops)| ops))
    }

    // Ensure the we see 20 successful items returned from the bulk stream
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.len() == 20
    }
}
