extern crate serde_json;

use std::io::Read;
use serde_json::Value;
use serde_json::value;
use regex::Regex;
use ::ast::ApiEndpoint;

pub fn from_reader<R>(rdr: &mut R) -> Result<ApiEndpoint, &str> where R: Read {
	//Read the file to string
	let mut json = String::new();
	let _ = rdr.read_to_string(&mut json).unwrap();
	json = replace_type_fields(&mut json);

	let root: Value = serde_json::from_str(&json[..]).unwrap();

	match root {
		Value::Object(data) => {
			//Get the name and value of the rest of the ast separately
			let (name, tree) = data.iter().next().unwrap();

			//Deserialise the api ast and set the name
			let mut endpoint = value::from_value::<ApiEndpoint>(tree.clone()).unwrap();
			endpoint.name = Some(name.clone());

			Ok(endpoint)
		},
		_ => Err("unexpected format")
	}
}

pub fn replace_type_fields(json: &str) -> String {
	let mut rpl = Regex::new(r#""(type)"\s?:\s?""#).unwrap();
	let mut rpl_json = rpl.replace_all(json, r#""field_type" : ""#);

	rpl = Regex::new(r"\{\s?(type)\s?\}").unwrap();
	rpl_json = rpl.replace_all(&rpl_json[..], "{field_type}");

	rpl_json
}