extern crate serde_json;

use std::io::Read;
use serde_json::Value;
use serde_json::value;
use ::ast::SyntaxTree;

pub fn from_reader<R>(rdr: &mut R) -> Result<SyntaxTree, &'static str> where R: Read {
	//Read the file to string
	let mut json = String::new();
	let _ = rdr.read_to_string(&mut json).unwrap();

	let root: Value = serde_json::from_str(&json[..]).unwrap();

	match root {
		Value::Object(data) => {
			//Get the name and value of the rest of the ast separately
			let (name, tree) = data.iter().next().unwrap();

			//Deserialise the api ast and set the name
			let mut endpoint = value::from_value::<SyntaxTree>(tree.clone()).unwrap();
			endpoint.name = Some(name.clone());

			Ok(endpoint)
		},
		_ => Err("unexpected format")
	}
}