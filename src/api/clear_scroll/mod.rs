use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-scroll.html
pub fn delete<'a>(client: &'a mut Client, req: &'a RequestParams)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 15 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_search/scroll");
    url_fmtd.push_str(url_qry);
    let res = client.delete(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-scroll.html
pub fn delete_scroll_id<'a>(client: &'a mut Client, req: &'a RequestParams,
                        scroll_id: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + scroll_id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_search/scroll/");
    url_fmtd.push_str(scroll_id);
    url_fmtd.push_str(url_qry);
    let res = client.delete(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

