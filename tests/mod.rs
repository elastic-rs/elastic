extern crate hyper;
extern crate url;
extern crate elastic_hyper;

use hyper::header::*;
use elastic_hyper::RequestParams;

#[test]
fn request_params_has_default_content_type() {
	let req = RequestParams::default();

	assert_eq!(Some(&ContentType::json()), req.headers.get::<ContentType>());
}

#[test]
fn request_params_has_default_base_url() {
	let req = RequestParams::default();

	assert_eq!("http://localhost:9200", req.base_url);
}

#[test]
fn request_params_has_url_query() {
	let req = RequestParams::default()
		.url_params(vec![
			("pretty", "true".to_owned()),
			("q", "*".to_owned())
		]);

	assert_eq!((16, Some(String::from("?pretty=true&q=*"))), req.get_url_qry());
}

#[test]
fn empty_request_params_returns_empty_string() {
	let req = RequestParams::default();

	assert_eq!((0, None), req.get_url_qry());
}