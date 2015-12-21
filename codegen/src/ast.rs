use std::collections::BTreeMap;
use serde::{ Serialize };

#[derive(Serialize, Deserialize)]
pub struct ApiEndpoint {
	pub name: Option<String>,
	pub documentation: String,
	pub methods: Vec<String>,
	pub body: Option<Body>,
	pub url: Url
}

#[derive(Serialize, Deserialize)]
pub struct Body {
	pub description: String,
	pub required: bool,
	pub serialize: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Url {
	pub path: String,
	pub paths: Vec<String>,
	pub parts: BTreeMap<String, Part>,
	pub params: BTreeMap<String, Param>
}

#[derive(Serialize, Deserialize)]
pub struct Part {
	#[serde(rename="type")]
	pub field_type: String,
	pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct Param {
	#[serde(rename="type")]
	pub field_type: String,
	pub description: String,
	pub options: Option<Vec<String>>
}