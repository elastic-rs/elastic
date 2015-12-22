#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate regex;

pub mod parse;
pub mod ast;

#[test]
fn can_parse_from_file() {
	let mut f = std::fs::File::open("api/bulk.json").unwrap();
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
		ast::HttpMethod::Head,
		ast::HttpMethod::Get,
		ast::HttpMethod::Post,
		ast::HttpMethod::Put,
		ast::HttpMethod::Delete,
		ast::HttpMethod::Other("STUFF".to_string())
	);

	let methods: Vec<ast::HttpMethod> = raw_methods.iter().map(|m| ast::HttpMethod::parse(m)).collect();

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
	struct Type {
		name: &'static str,
		opts: Option<Vec<String>>
	}

	let raw_types: Vec<Type> = vec!(
		Type { name: "string", opts: None },
		Type { name: "boolean", opts: None },
		Type { name: "time", opts: None },
		Type { name: "list", opts: None },
		Type { name: "number", opts: None },
		Type { name: "enum", opts: Some(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string())) }
	);

	let expected_types = vec!(
		ast::Type::Str,
		ast::Type::Bool,
		ast::Type::Time,
		ast::Type::List,
		ast::Type::Num,
		ast::Type::Enum(vec!("OpA".to_string(), "OpB".to_string(), "OpC".to_string()))
	);

	let types: Vec<ast::Type> = raw_types.iter().map(|t| ast::Type::parse(t.name, t.opts.clone())).collect();

	let mut success = true;
	for i in 0..types.len() {
		if expected_types[i] != types[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}