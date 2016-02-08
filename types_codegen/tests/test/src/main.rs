#![feature(plugin)]
#![plugin(elastic_types_codegen)]

fn main() {
	let key = "mykey";
	let strval = "mystring";
	let doc = 14;

	/*let j = json!(key, doc, strval, { 
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
	});*/

	let j = json!({ 
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
	});

	println!("{}", j);
}