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
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, PartialEq, Clone, Copy)]
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
	Delete
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
	/// assert!(verb == HttpVerb::Other("get"));
	/// ```
	pub fn parse(_method: &str) -> HttpVerb {
		match _method {
			"HEAD" => HttpVerb::Head,
			"POST" => HttpVerb::Post,
			"PUT" => HttpVerb::Put,
			"PATCH" => HttpVerb::Patch,
			"DELETE" => HttpVerb::Delete,
            _ => HttpVerb::Get
		}
	}
}

impl  Deserialize for HttpVerb {
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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type<'a> {
	/// boolean
	Bool,
	/// number
	Number(NumberKind),
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
	Enum(&'a Option<Vec<String>>),
	/// unknown
	Other(&'a str)
}

/// Represents a number type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberKind {
	/// long
	Long,
	/// integer
	Int,
	/// short
	Short,
	/// byte
	Byte,
	/// double
	Double,
	/// float
	Float
}

impl <'a> Type<'a> {
	/// Parses a type name from a string.
	/// 
	/// This parser is lossy, it only retains the minimal amount of info that could be needed for code generation.
	/// It also doesn't cover all possible Elasticsearch types, only those that appear in the API spec.
	pub fn parse(_type: &'a str, opts: &'a Option<Vec<String>>) -> Type<'a> {
		match _type {
			"boolean" => Type::Bool,
			"number"|"long" => Type::Number(NumberKind::Long),
			"integer" => Type::Number(NumberKind::Int),
			"short" => Type::Number(NumberKind::Short),
			"byte" => Type::Number(NumberKind::Byte),
			"double" => Type::Number(NumberKind::Double),
			"float" => Type::Number(NumberKind::Float),
			"string" => Type::Str,
			"time"|"date" => Type::Time,
			"binary" => Type::Bin,
			"geo_point"|"geo_shape" => Type::Geo,
			"list" => Type::List,
			"enum" => Type::Enum(opts),
			t => Type::Other(t)
		}
	}
}

/// The HTTP body of an API request.
/// 
/// Represents the JSON body of an Elasticsearch request. Extra details are in the `Params` on the `Url`.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Url {
	/// The default right side of the `Url`.
	/// 
	/// For example `/cluster/health`. The `path` is also the first entry in `paths` if there are alternative urls.
	/// The url may also contain `Parts` for replacement in braces, for example: `/{index}/{type}`.
	/// Each of these `Parts` will have an entry in the `parts` field.
	pub path: String,
	/// Alternatives to `path` containing literal segments and url parameters as `Path`s.
	pub paths: Vec<String>,
	/// `Url` paramaters.
	pub parts: BTreeMap<String, Part>,
	/// `Body` parameters.
	pub params: BTreeMap<String, Param>
}

/// A `Url` parameter for an `Endpoint`.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
	pub fn get_type<'a>(&'a self) -> Type<'a> {
		Type::parse(&self._type, &self.options)
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

	/// Create a new i64 `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::long(false, Some(18i64));
	/// ```
	pub fn long(required: bool, default: Option<i64>) -> Param {
		let def: Value = match default {
			Some(n) => Value::I64(n),
			None => Value::Null
		};

		Param {
			_type: Some("long".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}

	/// Create a new i32 `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::int(false, Some(18i32));
	/// ```
	pub fn int(required: bool, default: Option<i32>) -> Param {
		let def: Value = match default {
			Some(n) => Value::I64(n as i64),
			None => Value::Null
		};

		Param {
			_type: Some("integer".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}

	/// Create a new i16 `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::short(false, Some(18i16));
	/// ```
	pub fn short(required: bool, default: Option<i16>) -> Param {
		let def: Value = match default {
			Some(n) => Value::I64(n as i64),
			None => Value::Null
		};

		Param {
			_type: Some("short".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}

	/// Create a new f64 `Param`.
	/// 
	/// # Examples
	/// 
	/// With a default value:
	/// 
	/// ```
	/// use elastic_codegen::api::ast::Param;
	/// 
	/// let param = Param::float(false, Some(18.00f64));
	/// ```
	pub fn float(required: bool, default: Option<f64>) -> Param {
		let def: Value = match default {
			Some(n) => Value::F64(n),
			None => Value::Null
		};

		Param {
			_type: Some("long".to_string()),
			default: def,
			required: Some(required),
			options: None,
			description: String::new()
		}
	}
	
	/// Get the `Type` for the `Param`.
	pub fn get_type<'a>(&'a self) -> Type<'a> {
		match self._type {
            Some(ref t) => Type::parse(t, &self.options),
            None => Type::parse("unknown", &self.options)
        }
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
			ref d => Some(value::from_value::<T>(d.clone()).unwrap())
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
