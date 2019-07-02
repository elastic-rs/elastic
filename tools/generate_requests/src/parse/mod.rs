use serde::{
    Deserialize,
    Deserializer,
};
use serde_json::Value;

use std::{
    collections::BTreeMap,
    fmt,
};

mod parse;

pub trait ApiEndpoint {
    fn endpoint(self) -> (String, Endpoint);
}

impl ApiEndpoint for BTreeMap<String, Endpoint> {
    fn endpoint(self) -> (String, Endpoint) {
        self.into_iter().next().unwrap()
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Endpoint {
    pub documentation: String,
    pub methods: Vec<Method>,
    pub url: Url,
    pub body: Option<Body>,
}

impl Endpoint {
    pub fn has_body(&self) -> bool {
        self.body.is_some()
            || self
                .methods
                .iter()
                .any(|m| m == &Method::Post || m == &Method::Put)
    }

    pub fn preferred_method(&self) -> Option<Method> {
        let mut iter = self.methods.iter().cloned();
        match iter.len() {
            0 => None,
            1 => iter.next(),
            _ => {
                if iter.any(|m| m == Method::Post) {
                    Some(Method::Post)
                } else {
                    iter.next()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone, Copy)]
pub enum Method {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Url {
    pub path: Path,
    pub paths: Vec<Path>,
    #[serde(default = "BTreeMap::new")]
    pub parts: BTreeMap<String, Type>,
    #[serde(default = "BTreeMap::new")]
    pub params: BTreeMap<String, Type>,
}

impl Url {
    pub fn get_type<'a>(&'a self, name: &str) -> Option<&'a Type> {
        self.parts.get(name).or(self.params.get(name))
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Type {
    #[serde(rename = "type", default)]
    pub ty: TypeKind,
    pub description: String,
    #[serde(default = "Vec::new")]
    pub options: Vec<Value>,
    #[serde(default)]
    pub default: Option<Value>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum TypeKind {
    None,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "duration")]
    Duration,
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::None
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Path(#[serde(deserialize_with = "rooted_path_string")] pub String);

// Ensure all deserialized paths have a leading `/`
fn rooted_path_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if !s.starts_with('/') {
        Ok(format!("/{}", s))
    } else {
        Ok(s)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Path {
    pub fn split<'a>(&'a self) -> Vec<PathPart<'a>> {
        Path::parse(self.0.as_bytes(), PathParseState::Literal, Vec::new())
    }

    pub fn params<'a>(&'a self) -> Vec<&'a str> {
        self.split()
            .iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn parse<'a>(i: &'a [u8], state: PathParseState, r: Vec<PathPart<'a>>) -> Vec<PathPart<'a>> {
        if i.len() == 0 {
            return r;
        }

        let mut r = r;

        match state {
            PathParseState::Literal => {
                let (rest, part) = Path::parse_literal(i);

                if part.len() > 0 {
                    r.push(PathPart::Literal(part));
                }

                Path::parse(rest, PathParseState::Param, r)
            }
            PathParseState::Param => {
                let (rest, part) = Path::parse_param(i);

                if part.len() > 0 {
                    r.push(PathPart::Param(part));
                }

                Path::parse(rest, PathParseState::Literal, r)
            }
        }
    }

    fn parse_literal<'a>(i: &'a [u8]) -> (&'a [u8], &'a str) {
        if i[0] == b'}' {
            let i = parse::shift(i, 1);
            parse::take_while(i, |c| c != b'{')
        } else {
            parse::take_while(i, |c| c != b'{')
        }
    }

    fn parse_param<'a>(i: &'a [u8]) -> (&'a [u8], &'a str) {
        if i[0] == b'{' {
            let i = parse::shift(i, 1);
            parse::take_while(i, |c| c != b'}')
        } else {
            parse::take_while(i, |c| c != b'}')
        }
    }
}

enum PathParseState {
    Literal,
    Param,
}

#[derive(Debug, PartialEq)]
pub enum PathPart<'a> {
    Literal(&'a str),
    Param(&'a str),
}

pub trait PathParams<'a> {
    fn params(&'a self) -> Vec<&'a str>;
}

impl<'a> PathParams<'a> for Vec<PathPart<'a>> {
    fn params(&'a self) -> Vec<&'a str> {
        self.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Body {
    pub description: String,
}

#[cfg(test)]
pub fn get_url() -> Url {
    Url {
        path: Path("/_search".to_string()),
        paths: vec![
            Path("/_search".to_string()),
            Path("/{index}/_search".to_string()),
            Path("/{index}/{type}/_search".to_string()),
        ],
        parts: {
            let mut map = BTreeMap::new();

            map.insert(
                "index".to_string(),
                Type {
                    ty: TypeKind::List,
                    description: "A comma-separated list of index names to search".to_string(),
                    options: vec![],
                    default: None,
                },
            );

            map.insert(
                "type".to_string(),
                Type {
                    ty: TypeKind::List,
                    description: "A comma-separated list of document types to search".to_string(),
                    options: vec![],
                    default: None,
                },
            );

            map
        },
        params: {
            let mut map = BTreeMap::new();

            map.insert(
                "analyzer".to_string(),
                Type {
                    ty: TypeKind::String,
                    description: "The analyzer to use for the query string".to_string(),
                    options: vec![],
                    default: None,
                },
            );

            map
        },
    }
}

#[cfg(test)]
mod tests {
    mod path {
        use crate::parse::{
            Path,
            PathPart,
        };

        #[test]
        fn parse_param_only() {
            let path = Path("{index}".to_string());

            let expected = vec![PathPart::Param("index")];

            assert_eq!(expected, path.split());
        }

        #[test]
        fn parse_param_first() {
            let path = Path("{index}/{type}".to_string());

            let expected = vec![
                PathPart::Param("index"),
                PathPart::Literal("/"),
                PathPart::Param("type"),
            ];

            assert_eq!(expected, path.split());
        }

        #[test]
        fn parse_params_and_literals() {
            let path = Path("/{index}/part/{type}".to_string());

            let expected = vec![
                PathPart::Literal("/"),
                PathPart::Param("index"),
                PathPart::Literal("/part/"),
                PathPart::Param("type"),
            ];

            assert_eq!(expected, path.split());
        }

        #[test]
        fn parse_literal_only() {
            let path = Path("/part".to_string());

            let expected = vec![PathPart::Literal("/part")];

            assert_eq!(expected, path.split());
        }

        #[test]
        fn get_params() {
            let path = Path("/{index}/part/{type}".to_string());

            let expected = vec!["index", "type"];

            assert_eq!(expected, path.params());
        }
    }

    mod endpoint {
        use crate::parse::*;

        #[test]
        fn has_body_if_body_is_some() {
            let endpoint = Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get],
                url: get_url(),
                body: Some(Body {
                    description: String::new(),
                }),
            };

            assert!(endpoint.has_body());
        }

        #[test]
        fn has_body_if_method_is_put() {
            let endpoint = Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get, Method::Put],
                url: get_url(),
                body: None,
            };

            assert!(endpoint.has_body());
        }

        #[test]
        fn has_body_if_method_is_post() {
            let endpoint = Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get, Method::Post],
                url: get_url(),
                body: None,
            };

            assert!(endpoint.has_body());
        }

        #[test]
        fn has_no_body_if_none_and_not_put_or_post() {
            let endpoint = Endpoint {
                documentation: String::new(),
                methods: vec![Method::Get, Method::Delete],
                url: get_url(),
                body: None,
            };

            assert!(!endpoint.has_body());
        }
    }

    mod url {
        use crate::parse::*;

        #[test]
        fn lookup_param_type_in_part() {
            let url = get_url();

            let expected = Type {
                ty: TypeKind::List,
                description: "A comma-separated list of index names to search".to_string(),
                options: vec![],
                default: None,
            };

            let ty = url.get_type("index").unwrap();

            assert_eq!(expected, *ty);
        }

        #[test]
        fn lookup_param_type_in_param() {
            let url = get_url();

            let expected = Type {
                ty: TypeKind::String,
                description: "The analyzer to use for the query string".to_string(),
                options: vec![],
                default: None,
            };

            let ty = url.get_type("analyzer").unwrap();

            assert_eq!(expected, *ty);
        }
    }

    mod ser {
        use crate::parse::*;
        use serde_json::{
            self,
            value::to_value,
        };
        use std::collections::BTreeMap;

        fn http_eq(expected: Method, ser: &'static str) {
            assert_eq!(expected, serde_json::from_str::<Method>(ser).unwrap());
        }

        #[test]
        fn deserialise_http_method() {
            http_eq(Method::Head, "\"HEAD\"");
            http_eq(Method::Get, "\"GET\"");
            http_eq(Method::Put, "\"PUT\"");
            http_eq(Method::Post, "\"POST\"");
            http_eq(Method::Patch, "\"PATCH\"");
            http_eq(Method::Delete, "\"DELETE\"");
        }

        fn type_kind_eq(expected: TypeKind, ser: &'static str) {
            assert_eq!(expected, serde_json::from_str::<TypeKind>(ser).unwrap());
        }

        #[test]
        fn deserialise_type_kind_method() {
            type_kind_eq(TypeKind::List, "\"list\"");
            type_kind_eq(TypeKind::Enum, "\"enum\"");
            type_kind_eq(TypeKind::String, "\"string\"");
            type_kind_eq(TypeKind::Text, "\"text\"");
            type_kind_eq(TypeKind::Boolean, "\"boolean\"");
            type_kind_eq(TypeKind::Number, "\"number\"");
            type_kind_eq(TypeKind::Float, "\"float\"");
            type_kind_eq(TypeKind::Integer, "\"integer\"");
            type_kind_eq(TypeKind::Time, "\"time\"");
            type_kind_eq(TypeKind::Duration, "\"duration\"");
        }

        #[test]
        fn deserialise_part_as_type() {
            let ser = json!({
                "type" : "list",
                "description" : "A comma-separated list of index names to search"
            });

            let expected = Type {
                ty: TypeKind::List,
                description: "A comma-separated list of index names to search".to_string(),
                options: vec![],
                default: None,
            };

            assert_eq!(expected, serde_json::from_value::<Type>(ser).unwrap());
        }

        #[test]
        fn deserialise_param_as_type() {
            let ser = json!({
                "type" : "enum",
                "options" : [ "AND","OR" ],
                "default" : "OR",
                "description" : "The default operator for query string query (AND or OR)"
            });

            let expected = Type {
                ty: TypeKind::Enum,
                description: "The default operator for query string query (AND or OR)".to_string(),
                options: vec![to_value("AND").unwrap(), to_value("OR").unwrap()],
                default: Some(to_value("OR").unwrap()),
            };

            assert_eq!(expected, serde_json::from_value::<Type>(ser).unwrap());
        }

        #[test]
        fn deserialise_body_some() {
            let ser = json!({
                "description": "The search definition using the Query DSL"
            });

            let expected = Some(Body {
                description: "The search definition using the Query DSL".to_string(),
            });

            assert_eq!(
                expected,
                serde_json::from_value::<Option<Body>>(ser).unwrap()
            );
        }

        #[test]
        fn deserialise_body_none() {
            let expected: Option<Body> = None;

            assert_eq!(
                expected,
                serde_json::from_str::<Option<Body>>("null").unwrap()
            );
        }

        #[test]
        fn deserialise_url() {
            let ser = json!({
                "path": "/_search",
                "paths": ["/_search", "/{index}/_search", "/{index}/{type}/_search"],
                "parts": {
                    "index": {
                        "type" : "list",
                        "description" : "A comma-separated list of index names to search"
                    },
                    "type": {
                        "type" : "list",
                        "description" : "A comma-separated list of document types to search"
                    }
                },
                "params": {
                    "analyzer": {
                        "type" : "string",
                        "description" : "The analyzer to use for the query string"
                    }
                }
            });

            let expected = Url {
                path: Path("/_search".to_string()),
                paths: vec![
                    Path("/_search".to_string()),
                    Path("/{index}/_search".to_string()),
                    Path("/{index}/{type}/_search".to_string()),
                ],
                parts: {
                    let mut map = BTreeMap::new();

                    map.insert(
                        "index".to_string(),
                        Type {
                            ty: TypeKind::List,
                            description: "A comma-separated list of index names to search"
                                .to_string(),
                            options: vec![],
                            default: None,
                        },
                    );

                    map.insert(
                        "type".to_string(),
                        Type {
                            ty: TypeKind::List,
                            description: "A comma-separated list of document types to search"
                                .to_string(),
                            options: vec![],
                            default: None,
                        },
                    );

                    map
                },
                params: {
                    let mut map = BTreeMap::new();

                    map.insert(
                        "analyzer".to_string(),
                        Type {
                            ty: TypeKind::String,
                            description: "The analyzer to use for the query string".to_string(),
                            options: vec![],
                            default: None,
                        },
                    );

                    map
                },
            };

            assert_eq!(expected, serde_json::from_value::<Url>(ser).unwrap());
        }

        #[test]
        fn deserialise_endpoint() {
            let ser = json!({
                "search": {
                    "documentation": "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html",
                    "methods": ["GET", "POST"],
                    "url": {
                        "path": "/_search",
                        "paths": [],
                        "parts": { },
                        "params": { }
                    },
                    "body": {
                        "description": "The search definition using the Query DSL"
                    }
                }
            });

            let mut expected = BTreeMap::new();
            expected.insert(
                "search".to_string(),
                Endpoint {
                    documentation: "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html".to_string(),
                    methods: vec![Method::Get, Method::Post],
                    url: Url {
                        path: Path("/_search".to_string()),
                        paths: vec![],
                        parts: BTreeMap::new(),
                        params: BTreeMap::new(),
                    },
                    body: Some(Body {
                        description: "The search definition using the Query DSL".to_string(),
                    }),
                },
            );

            let de: BTreeMap<String, Endpoint> = serde_json::from_value(ser).unwrap();

            assert_eq!(expected.endpoint(), de.endpoint());
        }
    }
}
