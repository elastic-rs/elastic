use std::collections::BTreeMap;
use serde::{ Serialize, Serializer, Deserialize, Deserializer };

#[derive(Serialize, Deserialize)]
pub struct ApiEndpoint {
	pub name: Option<String>,
	pub documentation: String,
	pub methods: Vec<String>,
	pub body: Option<Body>,
	pub url: Option<Url>
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
	pub parts: BTreeMap<String, Part>
}

#[derive(Serialize, Deserialize)]
pub struct Part {
	pub field_type: Option<String>,
	pub description: String
}