#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use std::collections::BTreeMap;
use syntax::ast::*;
use syntax::attr::AttrMetaMethods;
use syntax::parse::token;
use syntax::print::pprust;
use elastic_codegen::api::ast::*;
use elastic_codegen::api::gen::*;
use elastic_codegen::api::gen::rust::*;

#[test]
fn can_parse_params_from_es_url() {
	let url = "/{index}/{type}/_bulk";

	let params = parse_path_params(url).unwrap();

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

	let parts = parse_path_parts(url).unwrap();

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
fn can_get_rust_doc_comment_for_endpoint() {
    let endpoint = Endpoint {
          name: None,
          documentation: "My docs".to_string(),
          methods: Vec::new(),
          body: None,
          url: Url {
              path: String::new(),
              paths: Vec::new(),
              parts: BTreeMap::new(),
              params: BTreeMap::new()
          }
    };
    
    let docs = endpoint.get_doc();
    
    //TODO: Get the '///' or '//!' prepended
    assert_eq!("My docs", pprust::attr_to_string(&docs));
}