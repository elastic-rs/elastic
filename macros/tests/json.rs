#![feature(plugin)]
#![plugin(elastic_macros)]
extern crate serde;
extern crate serde_json;
extern crate elastic_macros;

use elastic_macros::json::*;

#[test]
fn can_generate_json() {
	let j = json!({ 
		"a": 7, 
		"b": { "c": "some stuff" },
		"data": [
			{ "id": 1, "name": "stuff" },
			{ "id": 2, "name": "stuff" }
		]
	});

	assert_eq!("{\"a\":7,\"b\":{\"c\":\"some stuff\"},\"data\":[{\"id\":1,\"name\":\"stuff\"},{\"id\":2,\"name\":\"stuff\"}]}", j);
}

#[test]
fn can_generate_quasi_json() {
	let j = json!({ 
		a: 7, 
		b: { c: "some stuff" },
		data: [
			{ id: 1, name: "stuff" },
			{ id: 2, name: "stuff" }
		]
	});

	assert_eq!("{\"a\":7,\"b\":{\"c\":\"some stuff\"},\"data\":[{\"id\":1,\"name\":\"stuff\"},{\"id\":2,\"name\":\"stuff\"}]}", j);
}

#[test]
fn can_add_replacement_idents_to_json() {
	let a = 7;
	let c = "some stuff";
	let name = "stuff";

	let j = json!(a, c, name, { 
		a: $a, 
		b: { c: $c },
		data: [
			{ id: 1, name: $name },
			{ id: 2, name: $name }
		]
	});

	assert_eq!("{\"a\":7,\"b\":{\"c\":\"some stuff\"},\"data\":[{\"id\":1,\"name\":\"stuff\"},{\"id\":2,\"name\":\"stuff\"}]}", j);
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

#[test]
fn sanitisation_removes_whitespace() {
	let j = "\n{ \"a\" : \"stuff\", \"b\":{  \"c\":[ 0, \r\n1 ] }		,\"d\":14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[0,1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_does_not_affect_strings() {
	let j = "\n{ \"a\" : \"stuff and data.\n 	More.\"}";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff and data.\n 	More.\"}", &sanitised);
}

#[test]
fn sanitisation_standardises_quotes() {
	let j = "{ 'a' : \"stuff\", \"b\":{  \"c\":[ '0', 1 ] },\"d\":14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[\"0\",1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_quotes_unquoted_keys() {
	let j = "{ a : \"stuff\", \"b\":{  c:[ 0, 1 ] },d:14 }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":{\"c\":[0,1]},\"d\":14}", &sanitised);
}

#[test]
fn sanitisation_does_not_quote_special_values() {
	let j = "{ \"a\": \"stuff\", \"b\": true, \"c\": false, \"d\": null }";

	let mut sanitised = String::new();
	sanitise(j.as_bytes(), &mut sanitised);

	assert_eq!("{\"a\":\"stuff\",\"b\":true,\"c\":false,\"d\":null}", &sanitised);
}