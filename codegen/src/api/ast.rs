use std::collections::BTreeMap;
use serde::{ Serialize, Deserialize };
use serde_json::Value;
use serde_json::value;

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
	required: Option<bool>,
	pub serialize: Option<String>,
}

impl Body {
	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
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
	required: Option<bool>,
	pub description: String
}

impl Part {
	pub fn get_type(&self) -> Type {
		Type::parse(&self._type[..], self.options.clone())
	}

	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Param {
	#[serde(rename="type")]
	_type: Option<String>,
	pub description: String,
	pub options: Option<Vec<String>>,
	required: Option<bool>,
	default: Value
}

impl Param {
	pub fn new(_type: &str, desc: &str, default: Value, opts: Option<Vec<String>>) -> Param {
		Param {
			_type: Some(_type.to_string()),
			description: desc.to_string(),
			options: opts,
			required: None,
			default: default
		}
	}

	pub fn get_type(&self) -> Type {
		let _type: String = match self._type.clone() {
			None => "unknown".to_string(),
			Some(t) => t
		};

		Type::parse(&_type[..], self.options.clone())
	}

	pub fn get_default<T: Deserialize>(&self) -> T {
		value::from_value::<T>(self.default.clone()).unwrap()
	}

	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
}