#![feature(test)]
extern crate test;
extern crate elastic_codegen;
extern crate serde_json;

use std::fs::File;
use std::io::{ Read, Cursor };
use test::Bencher;
use elastic_codegen::api::ast::*;
use elastic_codegen::api::parse;

#[bench]
fn parse_api_spec(b: &mut Bencher) {
	//Get an API spec file
	let mut f = File::open("spec/api/bulk.json").unwrap();

	//Read to an in-memory buffer
	let mut bytes: Vec<u8> = Vec::new();
	f.read_to_end(&mut bytes).unwrap();

	let mut reader = Cursor::new(bytes);

	b.iter(|| {
		let _ = parse::from_reader(&mut reader).unwrap();
		reader.set_position(0);
	});
}