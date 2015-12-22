use std::collections::BTreeMap;
use serde::{ Serialize };

#[derive(Serialize, Deserialize)]
pub struct SyntaxTree {
	pub name: Option<String>,
	pub documentation: String,
	methods: Vec<String>,
	pub body: Option<Body>,
	pub url: Url
}

impl SyntaxTree {
	pub fn get_methods(&self) -> Vec<HttpMethod> {
		self.methods.iter().map(|method| HttpMethod::parse(&method[..])).collect()
	}
}

#[derive(PartialEq)]
pub enum HttpMethod {
	Head,
	Get,
	Post,
	Put,
	Patch,
	Delete,
	Other(String)
}

impl HttpMethod {
	pub fn parse(_method: &str) -> HttpMethod {
		match _method {
			"HEAD" => HttpMethod::Head,
			"GET" => HttpMethod::Get,
			"POST" => HttpMethod::Post,
			"PUT" => HttpMethod::Put,
			"PATCH" => HttpMethod::Patch,
			"DELETE" => HttpMethod::Delete,
			m => HttpMethod::Other(m.to_string())
		}
	}
}

#[derive(PartialEq)]
pub enum Type {
	Bool,
	Num,
	Str,
	Time,
	Bin,
	Geo,
	List,
	Enum(Vec<String>),
	Other(String)
}

impl Type {
	pub fn parse(_type: &str, opts: Option<Vec<String>>) -> Type {
		match _type {
			"boolean" => Type::Bool,
			"number"|"long"|"integer"|"short"|"byte"|"double"|"float" => Type::Num,
			"string" => Type::Str,
			"time"|"date" => Type::Time,
			"binary" => Type::Bin,
			"geo_point"|"geo_shape" => Type::Geo,
			"list" => Type::List,
			"enum" => Type::Enum(opts.unwrap()),
			t => Type::Other(t.to_string())
		}
	}
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
	_type: String,
	pub options: Option<Vec<String>>,
	pub description: String
}

impl Part {
	pub fn get_type(&self) -> Type {
		Type::parse(&self._type[..], self.options.clone())
	}
}

#[derive(Serialize, Deserialize)]
pub struct Param {
	#[serde(rename="type")]
	_type: String,
	pub description: String,
	pub options: Option<Vec<String>>
}

impl Param {
	pub fn get_type(&self) -> Type {
		Type::parse(&self._type[..], self.options.clone())
	}
}