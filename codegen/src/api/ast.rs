//! API Spec Abstract Syntax Tree
//! 
//! Contains Rust structures for the API specification.
//! Structs in this module are designed for inspecting after instantiation by `serde`, rather than constructing directly.

use std::collections::BTreeMap;
use serde;
use serde::{ Deserialize, Deserializer };
use serde_json::Value;
use serde_json::value;

/// Elasticsearch API Endpoint.
/// 
/// Represents an endpoint on the Elasticsearch REST API.
#[derive(Deserialize)]
pub struct Endpoint {
	/// The name of the `Endpoint`.
	/// 
	/// This may contain a hierarchy of features, separated by `.`. For example: `cluster.health`, `cluster.state` and `cluster.reroute` all belong to `cluster`.
	pub name: Option<String>,
	/// A link to the Elasticsearch documentation for this `Endpoint`.
	pub documentation: String,
	/// The allowed HTTP verbs.
	/// 
	/// All possible `Url`s for the `Endpoint` are assumed to be valid for all HTTP verbs.
	pub methods: Vec<HttpVerb>,
	/// The `Body` on a request (if required).
	pub body: Option<Body>,
	/// The available `Url`s for this `Endpoint`.
	pub url: Url
}

/// Represents a HTTP verb
#[derive(PartialEq)]
pub enum HttpVerb {
	/// HEAD
	Head,
	/// GET
	Get,
	/// POST
	Post,
	/// PUT
	Put,
	/// PATCH
	Patch,
	/// DELETE
	Delete,
	/// Unknown verb
	Other(String)
}

impl HttpVerb {
	/// Parses a HTTP verb from a string.
	/// 
	/// # Examples
	/// 
	/// This method is case sensitive. So:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::HttpVerb;
	/// 
	/// let verb = HttpVerb::parse("GET");
	/// assert!(verb == HttpVerb::Get);
	/// ```
	/// 
	/// but:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::HttpVerb;
	/// 
	/// let verb = HttpVerb::parse("get");
	/// assert!(verb == HttpVerb::Other("get".to_string()));
	/// ```
	pub fn parse(_method: &str) -> HttpVerb {
		match _method {
			"HEAD" => HttpVerb::Head,
			"GET" => HttpVerb::Get,
			"POST" => HttpVerb::Post,
			"PUT" => HttpVerb::Put,
			"PATCH" => HttpVerb::Patch,
			"DELETE" => HttpVerb::Delete,
			//TODO: prefer &str over String, should just be able to HttpVerb::Other(_method)
			m => HttpVerb::Other(m.to_string())
		}
	}
}

impl Deserialize for HttpVerb {
	fn deserialize<D>(deserializer: &mut D) -> Result<HttpVerb, D::Error> where D: Deserializer,
    {
        deserializer.visit_str(HttpVerbVisitor)
    }
}

struct HttpVerbVisitor;
impl serde::de::Visitor for HttpVerbVisitor {
	type Value = HttpVerb;

	fn visit_str<E>(&mut self, v: &str) -> Result<HttpVerb, E> where E: serde::de::Error {
		let result = HttpVerb::parse(v);
		Ok(result)
	}
}

/// Represents a `Param` or `Part` type.
#[derive(PartialEq)]
pub enum Type {
	/// boolean
	Bool,
	/// number|long|integer|short|byte|double|float
	Num,
	/// string
	Str,
	/// time|date
	Time,
	/// binary
	Bin,
	/// geo_point|geo_shape
	Geo,
	/// list
	List,
	/// enum
	Enum(Vec<String>),
	/// unknown
	Other(String)
}

impl Type {
	/// Parses a type name from a string.
	/// 
	/// This parser is lossy, it only retains the minimal amount of info that could be needed for code generation.
	/// It also doesn't cover all possible Elasticsearch types, only those that appear in the API spec.
	pub fn parse(_type: &str, opts: Option<Vec<String>>) -> Type {
		match _type {
			"boolean" => Type::Bool,
			//TODO: Split into Type::Num(Num::I64|I32|I16|Flt)
			"number"|"long"|"integer"|"short"|"byte"|"double"|"float" => Type::Num,
			"string" => Type::Str,
			"time"|"date" => Type::Time,
			"binary" => Type::Bin,
			"geo_point"|"geo_shape" => Type::Geo,
			"list" => Type::List,
			"enum" => Type::Enum(opts.unwrap()),
			//TODO: prefer &str over String, should just be able to Type::Other(_type)
			t => Type::Other(t.to_string())
		}
	}
}

/// The HTTP body of an API request.
/// 
/// Represents the JSON body of an Elasticsearch request. Extra details are in the `Params` on the `Url`.
#[derive(Serialize, Deserialize)]
pub struct Body {
	required: Option<bool>,
	/// A description of the `Body`.
	pub description: String,
	/// A serialization format.
	pub serialize: Option<String>,
}

impl Body {
	/// Create a new `Body`.
	pub fn new(required: bool) -> Body {
		Body {
			required: Some(required),
			description: String::new(),
			serialize: None
		}
	}

	/// Get whether or not the `Body` is required.
	/// 
	/// If the `required` field is not specified in the API spec, then it's assumed `false`.
	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
}

/// A potential `Url` for an API `Endpoint`.
/// 
/// Each `Url` may be a template containing `Parts` and `Params`.
#[derive(Serialize, Deserialize)]
pub struct Url {
	/// The default right side of the `Url`.
	/// 
	/// For example `/cluster/health`. The `path` is also the first entry in `paths` if there are alternative urls.
	/// The url may also contain `Parts` for replacement in braces, for example: `/{index}/{type}`.
	/// Each of these `Parts` will have an entry in the `parts` field.
	pub path: String,
	/// Alternatives to `path`.
	pub paths: Vec<String>,
	/// Details for replacement parts in `path` and `paths`.
	pub parts: BTreeMap<String, Part>,
	/// `Body` parameters.
	pub params: BTreeMap<String, Param>
}

/// A replacement part of a `Url`.
#[derive(Serialize, Deserialize)]
pub struct Part {
	#[serde(rename="type")]
	_type: String,
	required: Option<bool>,
	/// Options for an `Enum` part.
	pub options: Option<Vec<String>>,
	/// A description of the `Part`.
	pub description: String
}

impl Part {
	/// Get the `Type` for the `Part`.
	pub fn get_type(&self) -> Type {
		Type::parse(&self._type[..], self.options.clone())
	}

	/// Get whether or not the `Part` is required.
	/// 
	/// If the `required` field is not specified in the API spec, then it's assumed `false`.
	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
}

/// A `Body` parameter for an `Endpoint`.
#[derive(Serialize, Deserialize)]
pub struct Param {
	#[serde(rename="type")]
	_type: Option<String>,
	required: Option<bool>,
	default: Value,
	/// A description of the `Param`.
	pub description: String,
	/// Options for an `Enum` param.
	pub options: Option<Vec<String>>
}

impl Param {
	/// Create a new `Param`.
	/// 
	/// # Examples
	/// 
	/// Create a new enum `Param`:
	/// 
	/// ```
	/// # extern crate serde_json;
	/// # extern crate elastic_codegen;
	/// # fn main() {
	/// use serde_json::Value;
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::new(
	/// 	"enum", 
	/// 	Value::String("op1".to_string()), 
	/// 	Some(vec![
	/// 		"op1".to_string(), 
	/// 		"op2".to_string()]
	/// 	)
	/// );
	/// # }
	/// ```
	pub fn new(_type: &str, default: Value, opts: Option<Vec<String>>) -> Param {
		Param {
			_type: Some(_type.to_string()),
			required: None,
			default: default,
			options: opts,
			description: String::new(),
		}
	}

	/// Create a new string `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::string(false, Some("my default".to_string()));
	/// ```
	pub fn string(required: bool, default: Option<String>) -> Param {
		let def: Value = match default {
			Some(s) => Value::String(s),
			None => Value::Null
		};

		Param {
			_type: Some("string".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}

	/// Create a new bool `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::bool(false, Some(true));
	/// ```
	pub fn bool(required: bool, default: Option<bool>) -> Param {
		let def: Value = match default {
			Some(b) => Value::Bool(b),
			None => Value::Null
		};

		Param {
			_type: Some("boolean".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}
	
	//TODO: impls for num and borrow instead of clone for get_type

	/// Get the `Type` for the `Param`.
	pub fn get_type(&self) -> Type {
		let _type: String = match self._type.clone() {
			None => "unknown".to_string(),
			Some(t) => t
		};

		Type::parse(&_type[..], self.options.clone())
	}

	/// Get the default value for the `Param`.
	/// 
	/// # Examples
	/// 
	/// Get the default value for a `Param`:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::bool(false, Some(true));
	/// let default = param.get_default::<bool>().unwrap();
	/// 
	/// assert_eq!(true, default);
	/// ```
	pub fn get_default<T: Deserialize>(&self) -> Option<T> {
		match self.default {
			Value::Null => None,
			_ => Some(value::from_value::<T>(self.default.clone()).unwrap())
		}
	}

	/// Get whether or not the `Param` is required.
	/// 
	/// If the `required` field is not specified in the API spec, then it's assumed `false`.
	pub fn required(&self) -> bool {
		match self.required {
			None => false,
			Some(r) => r
		}
	}
}
