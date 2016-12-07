extern crate elastic_codegen;
extern crate serde_json;

use std::collections::BTreeMap;
use std::io::Read;
use std::fs::{ File, read_dir };
use elastic_codegen::parse::*;

fn from_dir(path: &str) -> Result<Vec<(String, Endpoint)>, String> {
	let mut all_parsed: Vec<(String, Endpoint)> = Vec::new();

	let paths = read_dir(path).unwrap();

	for path in paths {
		let path = path.unwrap().path();
		let display = path.to_string_lossy().into_owned();

		let mut f = File::open(path).unwrap();
		let parsed = try!(from_reader(display, &mut f));

		all_parsed.push(parsed);
	}

	Ok(all_parsed)
}

fn from_reader<R>(name: String, rdr: &mut R) -> Result<(String, Endpoint), String> where R: Read {
	let endpoint: BTreeMap<String, Endpoint> = try!(serde_json::from_reader(rdr).map_err(|e| format!("Failed to parse {} because: {}", name, e)));

	Ok(endpoint.endpoint())
}

fn strip_verbs(endpoint: (String, Endpoint)) -> (String, Endpoint) {
	let (name, mut endpoint) = endpoint;

	// Choose a single HTTP verb per endpoint: either POST or 1st entry
	let mut iter = endpoint.methods.into_iter();
	let verb = match iter.len() {
		0 => unreachable!(),
		1 => iter.next().unwrap(),
		_ => {
			if iter.any(|m| m == HttpMethod::Post) {
				HttpMethod::Post
			}
			else {
				iter.next().unwrap()
			}
		}
	};

	endpoint.methods = vec![verb];

	(name, endpoint)
}

fn rename_types(endpoint: (String, Endpoint)) -> (String, Endpoint) {
	let (name, mut endpoint) = endpoint;

	let mut new_params = BTreeMap::new();

	for (k, v) in endpoint.url.parts {
		let new_key = match (k.as_ref(), &v.ty) {
			("index", &TypeKind::List) => {
				String::from("indices")
			},
			_ => k
		};

		new_params.insert(new_key, v);
	}

	endpoint.url.parts = new_params;

	(name, endpoint)
}

fn main() {
	let dir = "./spec";

	// BTreeMap<String, bool> : <type name, is emitted>
	let mut params_to_emit = BTreeMap::new();
	params_to_emit.insert(String::from("vertices"), false);

	let endpoints: Vec<(String, Endpoint)> = 
		from_dir(dir)
		.expect("Couldn't parse the REST API spec")
		.into_iter()
		.map(|e| strip_verbs(e))
		.map(|e| rename_types(e))
		.collect();

	for e in endpoints {
		println!("{:?}", e);
	}
}