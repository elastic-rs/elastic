use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn post_index_type<'a,
                   I: Into<Body<'a>>>(client: &'a mut Client,
                                      req: &'a RequestParams, index: &'a str,
                                      _type: &'a str, body: I)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn get<'a>(client: &'a mut Client, req: &'a RequestParams) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 9 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn get_index_type<'a>(client: &'a mut Client, req: &'a RequestParams,
                      index: &'a str, _type: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 9 + index.len() +
                                  _type.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn post_index<'a,
              I: Into<Body<'a>>>(client: &'a mut Client,
                                 req: &'a RequestParams, index: &'a str,
                                 body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn post<'a,
        I: Into<Body<'a>>>(client: &'a mut Client, req: &'a RequestParams,
                           body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 9 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html
pub fn get_index<'a>(client: &'a mut Client, req: &'a RequestParams,
                 index: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_msearch");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

