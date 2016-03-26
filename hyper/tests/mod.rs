extern crate hyper;
extern crate url;
extern crate elastic_hyper;

use hyper::header::*;
use elastic_hyper::RequestParams;

#[test]
fn request_params_has_default_content_type() {
	let req = RequestParams::new(Headers::new());

	assert_eq!(Some(&ContentType::json()), req.headers.get::<ContentType>());
}

#[test]
fn request_params_has_url_query() {
	let req = RequestParams::new(Headers::new())
		.url_params(vec![
			("pretty", "true".to_owned()),
			("q", "*".to_owned())
		]);

	assert_eq!("pretty=true&q=*", &req.get_url_qry());
}