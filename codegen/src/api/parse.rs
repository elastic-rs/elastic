extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::fs::read_dir;
use serde_json::{ Value, value };
use super::ast::Endpoint;

/// Parses a Reader to an Elasticsearch API spec to an Endpoint.
pub fn from_reader<R>(rdr: &mut R) -> Result<Endpoint, &'static str> where R: Read {
	//Read the file to string
	let mut json = String::new();
	let _ = rdr.read_to_string(&mut json).unwrap();

	let root: Value = serde_json::from_str(&json[..]).unwrap();

	match root {
		Value::Object(data) => {
			//Get the name and value of the rest of the ast separately
			let (name, tree) = data.iter().next().unwrap();

			//Deserialise the api ast and set the name
			let mut endpoint = value::from_value::<Endpoint>(tree.clone()).unwrap();
			endpoint.name = Some(name.clone());

			Ok(endpoint)
		},
		_ => Err("unexpected format")
	}
}

/// Parses all Elasticsearch API spec files in a directory to Endpoints.
pub fn from_dir(path: &str) -> Result<Vec<Endpoint>, &'static str> {
	let mut all_parsed: Vec<Endpoint> = Vec::new();

	let paths = read_dir(path).unwrap();
	for path in paths {
		let p = path.unwrap().path();
		println!("parsing: {}", p.display());

		let mut f = File::open(p).unwrap();
		let parsed = try!(from_reader(&mut f));

		all_parsed.push(parsed);
	}

	Ok(all_parsed)
}
