#![feature(test, plugin)]
#![plugin(elastic_types_codegen)]

extern crate test;
use test::Bencher;

#[bench]
fn parse_plain_json_sml(b: &mut Bencher) {
	b.iter(|| {
		json!({ 
			"myval1": 7, 
			"myval2": { 
				"mykey": "some stuff",
				"d": 14
			},
			"myval3": [
				"a",
				"b",
				"mystring"
			]
		})
	});
}

#[bench]
fn parse_repl_json_sml(b: &mut Bencher) {
	let key = "mykey";
	let strval = "mystring";
	let doc = 14;

	b.iter(|| {
		json!(key, doc, strval, { 
			"myval1": 7, 
			"myval2": { 
				$key: "some stuff",
				"d": $doc
			},
			"myval3": [
				"a",
				"b",
				$strval
			]
		})
	});
}

//TODO: bench for large json replacement
