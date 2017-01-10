#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json_str;

extern crate serde;
extern crate serde_json;
extern crate elastic;

use elastic::client::*;
use elastic::types::prelude::*;

#[derive(Serialize, ElasticType)]
struct MyType {
	id: i32,
	title: &'static str
}

#[test]
fn index_request_from_doc() {
	let doc = MyType {
		id: 1,
		title: "A title"
	};
}