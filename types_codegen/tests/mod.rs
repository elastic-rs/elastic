#![feature(plugin)]
#![plugin(elastic_types_codegen)]
extern crate chrono;
use chrono::format::{ Item, Fixed, Numeric, Pad };

#[test]
fn can_generate_date_formats() {
	let _ = date_fmt!("yyyyMMddTHHmmss.SSSZ");
}