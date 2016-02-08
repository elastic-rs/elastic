#![feature(plugin)]
#![plugin(elastic_types_codegen)]
extern crate chrono;
extern crate elastic_types_codegen;

use elastic_types_codegen::json::*;

#[test]
fn can_generate_json() {
	let j = json!({ 
		"a": 7, 
		"b": { 
			"c": "some stuff" 
		} 
	});

	assert_eq!("{ \"a\" : 7 , \"b\" : { \"c\" : \"some stuff\" } }", j);
}

#[test]
fn can_parse_json_to_parts() {
	let j = "{ \"a\" : $abc , \"b\" : { \"c\" : $b, $c: \"stuff\", $doc:[ {}, { \"e\" : 15 }] } }";
	let bytes = j.as_bytes();

	let mut tree = Vec::new();
	parse_to_replacement(bytes, &mut tree);

	let expected = vec![
		JsonPart::Literal("{ \"a\" : ".to_string()),
		JsonPart::Replacement("abc".to_string(), ReplacementPart::Value),
		JsonPart::Literal(" , \"b\" : { \"c\" : ".to_string()),
		JsonPart::Replacement("b".to_string(), ReplacementPart::Value),
		JsonPart::Literal(", ".to_string()),
		JsonPart::Replacement("c".to_string(), ReplacementPart::Key),
		JsonPart::Literal(": \"stuff\", ".to_string()),
		JsonPart::Replacement("doc".to_string(), ReplacementPart::Key),
		JsonPart::Literal(":[ {}, { \"e\" : 15 }] } }".to_string())
	];

	let mut success = true;
	for i in 0..tree.len() {
		if expected[i] != tree[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

//TODO: Tests for json replacement, json with nested json, building json with iterator, json with new lines after replacement keys
