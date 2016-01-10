extern crate elastic_codegen;

use elastic_codegen::api::gen::*;
use elastic_codegen::api::gen::rust::*;

#[test]
fn can_parse_params_from_es_url() {
	let url = "/{index}/{type}/_bulk";

	let params = parse_path_params(url);

	let expected = vec![
		"index".to_string(),
		"type".to_string()
	];

	let mut success = true;
	for i in 0..params.len() {
		if expected[i] != params[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn can_parse_parts_from_es_url() {
	let url = "/{index}/_alias/{name}";

	let parts = parse_path_parts(url);

	let expected = vec![
		"/".to_string(),
		"/_alias/".to_string()
	];

	let mut success = true;
	for i in 0..parts.len() {
		if expected[i] != parts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
<<<<<<< HEAD:codegen/tests/api_gen.rs
fn can_build_url_format() {
	let fmt = parse_fmt("/{index}/_alias/{name}");

	assert_eq!("/{}/_alias/{}", &fmt[..]);
=======
fn can_build_url_format_stmt() {
	let (ident, stmt) = url_fmt_dec("/{}/_alias/{}", Vec::new());
>>>>>>> 14b407a32ab4b313e376c4d8c605b285c210466c:codegen/tests/api_gen.rs
}