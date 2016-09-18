use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// https://www.elastic.co/guide/en/elasticsearch/plugins/master/ingest.html
pub fn put_id<'a,
          I: Into<Body<'a>>>(client: &'a mut Client, req: &'a RequestParams,
                             id: &'a str, body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 18 + id.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_ingest/pipeline/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res =
        client.put(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

