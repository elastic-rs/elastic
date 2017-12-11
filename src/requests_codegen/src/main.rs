#![recursion_limit = "200"]

#[cfg(test)]
#[macro_use]
extern crate json_str;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate quote;
extern crate syn;

extern crate inflector;

pub mod parse;
pub mod gen;

use std::collections::BTreeMap;
use std::io::{stdout, Read, Write};
use std::fs::{read_dir, File};

use quote::Tokens;
use parse::*;

fn main() {
    start_comment_block_for_logging();

    println!("This code is automatically generated");

    let dir = "./spec";

    // BTreeMap<String, bool> : <url param type name, is emitted>
    let mut params_to_emit = BTreeMap::new();
    params_to_emit.insert(String::from("vertices"), false);

    let derives = quote!(#[derive(Debug, PartialEq, Clone)]);

    let mut tokens = quote::Tokens::new();

    let mut endpoints = from_dir(dir)
        .expect("Couldn't parse the REST API spec")
        .add_simple_search()
        .add_get_ping_req();

    endpoints = endpoints
        .into_iter()
        .map(|e| strip_methods(e))
        .map(|e| dedup_urls(e))
        .collect();

    let http_mod_name = "http";

    build_mod("endpoints", &mut tokens, |ref mut tokens| {
        endpoints_mod(
            tokens,
            derives.clone(),
            http_mod_name,
            endpoints,
            &mut params_to_emit,
        )
    });

    build_mod(http_mod_name, &mut tokens, |ref mut tokens| {
        http_mod(tokens, derives.clone())
    });

    build_mod("params", &mut tokens, |ref mut tokens| {
        params_mod(tokens, derives.clone(), params_to_emit)
    });

    end_comment_block_for_logging();

    stdout().write(tokens.to_string().as_bytes()).unwrap();
}

fn start_comment_block_for_logging() {
    stdout().write(b"/*").unwrap();
}

fn end_comment_block_for_logging() {
    stdout().write(b"*/").unwrap();
}

fn from_dir(path: &str) -> Result<Vec<(String, Endpoint)>, String> {
    let mut all_parsed: Vec<(String, Endpoint)> = Vec::new();

    let paths = read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let display = path.to_string_lossy().into_owned();

        let mut f = File::open(path).unwrap();
        let parsed = try!(from_reader(display, &mut f));

        all_parsed.push(parsed);
    }

    Ok(all_parsed)
}

fn from_reader<R>(name: String, rdr: &mut R) -> Result<(String, Endpoint), String>
where
    R: Read,
{
    let endpoint: BTreeMap<String, Endpoint> = try!(serde_json::from_reader(rdr).map_err(|e| format!("Failed to parse {} because: {}", name, e)));

    Ok(endpoint.endpoint())
}

fn strip_methods(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    let preferred_method = endpoint
        .preferred_method()
        .expect("there should always be at least 1 method");

    endpoint.methods = vec![preferred_method];

    (name, endpoint)
}

fn dedup_urls(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    let mut deduped_paths = BTreeMap::new();

    for path in endpoint.url.paths {
        let key = path.params().join("");

        deduped_paths.insert(key, path);
    }

    endpoint.url.paths = deduped_paths.into_iter().map(|(_, p)| p).collect();

    (name, endpoint)
}

trait CustomEndpoints {
    fn add_simple_search(self) -> Self;
    fn add_get_ping_req(self) -> Self;
}

impl CustomEndpoints for Vec<(String, Endpoint)> {
    fn add_simple_search(self) -> Vec<(String, Endpoint)> {
        self.into_iter()
            .fold(vec![], |mut endpoints, (name, endpoint)| {
                match name.as_ref() {
                    "search" => {
                        let mut simple_search_endpoint = endpoint.clone();
                        simple_search_endpoint.methods = vec![Method::Get];
                        simple_search_endpoint.body = None;

                        endpoints.push((String::from("simple_search"), simple_search_endpoint));
                        endpoints.push((String::from("search"), endpoint));
                    }
                    _ => endpoints.push((name, endpoint)),
                }

                endpoints
            })
    }

    fn add_get_ping_req(self) -> Vec<(String, Endpoint)> {
        self.into_iter()
            .fold(vec![], |mut endpoints, (name, endpoint)| {
                match name.as_ref() {
                    "ping" => {
                        let mut get_endpoint = endpoint.clone();
                        get_endpoint.methods = vec![Method::Get];

                        endpoints.push((String::from("ping"), get_endpoint));
                        endpoints.push((String::from("ping_head"), endpoint));
                    }
                    _ => endpoints.push((name, endpoint)),
                }

                endpoints
            })
    }
}

fn endpoints_mod(tokens: &mut Tokens, derives: Tokens, http_mod: &'static str, endpoints: Vec<(String, Endpoint)>, params_to_emit: &mut BTreeMap<String, bool>) {
    let mut http_mod_tokens = Tokens::new();
    http_mod_tokens.append(http_mod);

    let uses = quote!(
        use super:: #http_mod_tokens ::*;
        use super::params::*;
    );

    tokens.append(uses.to_string());
    tokens.append("\n\n");

    for e in endpoints {
        for (ty, _) in &e.1.url.parts {
            params_to_emit.insert(ty.to_owned(), true);
        }

        let url_params = gen::url_params::UrlParamBuilder::from(&e).build();
        let (ref url_params_item, _) = url_params;

        let (req_params_item, req_params_ty) = gen::request_params::RequestParamBuilder::from(&e).build();

        let req_ctors_item = gen::request_ctors::RequestParamsCtorBuilder::from((&e, &req_params_ty, &url_params)).build();
        let url_method_item = gen::url_builder::UrlMethodBuilder::from((&e, &url_params)).build();

        let req_into_http_item = gen::request_into_endpoint::RequestIntoEndpointBuilder::from((&e, &req_params_ty)).build();

        tokens.append_all(vec![
            derives.clone(),
            quote!(#url_params_item),
            quote!(#url_method_item),
            derives.clone(),
            quote!(#req_params_item),
            quote!(#req_ctors_item),
            quote!(#req_into_http_item),
        ]);
    }
}

fn http_mod(tokens: &mut Tokens, derives: Tokens) {
    let url_tokens = gen::types::url::tokens();

    let body_tokens = gen::types::body::tokens();

    let uses = quote!(
        use std::borrow::Cow;
        use std::ops::Deref;

        extern crate http;
        pub use self::http::Method;
    );

    let http_req_item = gen::types::request::req_tokens();

    tokens.append(uses.to_string());

    tokens.append("\n\n");

    tokens.append_all(vec![
        derives.clone(),
        url_tokens,
        derives.clone(),
        http_req_item,
        body_tokens,
    ]);
}

fn params_mod(tokens: &mut Tokens, derives: Tokens, params_to_emit: BTreeMap<String, bool>) {
    let uses = quote!(
        use std::borrow::Cow;
    );

    tokens.append(uses.to_string());
    tokens.append("\n\n");

    tokens.append(r#"include!("genned.params.rs");"#);
    tokens.append("\n\n");

    let params_to_emit = params_to_emit.iter().filter(|&(_, is_emitted)| *is_emitted);

    for (ty, _) in params_to_emit {
        let ty_item = gen::types::wrapped_ty::item(ty);

        tokens.append_all(vec![derives.clone(), quote!(#ty_item)]);

        tokens.append("\n\n");
    }
}

fn build_mod<F>(mod_name: &'static str, tokens: &mut Tokens, bldr: F)
where
    F: FnOnce(&mut Tokens) -> (),
{
    tokens.append(&format!("pub mod {} {{", mod_name));

    bldr(tokens);

    tokens.append("}");

    tokens.append("\n\n");
}
