extern crate reqwest;
extern crate url;
#[macro_use]
extern crate elastic_reqwest;

use reqwest::header::*;
use elastic_reqwest::RequestParams;

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
		.url_param("pretty", "false")
		.url_param("pretty", "true")
		.url_param("q", "*");

	assert_eq!((16, Some(String::from("?pretty=true&q=*"))), req.get_url_qry());
}

#[test]
fn empty_request_params_returns_empty_string() {
	let req = RequestParams::default();

	assert_eq!((0, None), req.get_url_qry());
}