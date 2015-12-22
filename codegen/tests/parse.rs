extern crate elastic_codegen;

use std::fs::File;
use elastic_codegen::parse;
use elastic_codegen::ast::*;
use elastic_codegen::parse::*;

#[test]
fn can_parse_from_file() {
	let mut f = File::open("api/bulk.json").unwrap();
	let parsed = parse::from_reader(&mut f).unwrap();

	assert!(parsed.name.unwrap() == "bulk".to_string());
}

#[test]
fn can_parse_http_method() {
	let raw_methods = vec!(
		"HEAD",
		"GET",
		"POST",
		"PUT",
		"DELETE",
		"STUFF"
	);

	let expected_methods = vec!(
		HttpMethod::Head,
		HttpMethod::Get,
		HttpMethod::Post,
		HttpMethod::Put,
		HttpMethod::Delete,
		HttpMethod::Other("STUFF".to_string())
	);

	let methods: Vec<HttpMethod> = raw_methods.iter().map(|m| HttpMethod::parse(m)).collect();

	let mut success = true;
	for i in 0..methods.len() {
		if expected_methods[i] != methods[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn can_parse_type() {
	struct TypeRef {
		name: &'static str,
		opts: Option<Vec<String>>
	}

	let raw_types: Vec<TypeRef> = vec!(
		TypeRef { name: "string", opts: None },
		TypeRef { name: "boolean", opts: None },
		TypeRef { name: "time", opts: None },
		TypeRef { name: "list", opts: None },
		TypeRef { name: "number", opts: None },
		TypeRef { name: "enum", opts: Some(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string())) },
		TypeRef { name: "stuff", opts: None }
	);

	let expected_types = vec!(
		Type::Str,
		Type::Bool,
		Type::Time,
		Type::List,
		Type::Num,
		Type::Enum(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string())),
		Type::Other("stuff".to_string())
	);

	let types: Vec<Type> = raw_types.iter().map(|t| Type::parse(t.name, t.opts.clone())).collect();

	let mut success = true;
	for i in 0..types.len() {
		if expected_types[i] != types[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}