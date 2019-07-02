#![recursion_limit = "200"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

pub mod gen;
pub mod parse;

use std::{
    collections::BTreeMap,
    fs::{
        read_dir,
        File,
    },
    io::{
        stdout,
        Read,
        Write,
    },
};

use crate::parse::*;
use quote::Tokens;

fn main() {
    start_comment_block_for_logging();

    println!("This code is automatically generated");
    println!("run the `tools/generate_requests.sh` script to update it");

    let dir = "./tools/generate_requests/spec";

    // BTreeMap<String, bool> : <url param type name, is emitted>
    let mut params_to_emit = BTreeMap::new();
    params_to_emit.insert(String::from("vertices"), false);

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
        endpoints_mod(tokens, http_mod_name, endpoints, &mut params_to_emit)
    });

    build_mod(http_mod_name, &mut tokens, |ref mut tokens| {
        http_mod(tokens)
    });

    build_mod("params", &mut tokens, |ref mut tokens| {
        params_mod(tokens, params_to_emit)
    });

    end_comment_block_for_logging();

    stdout().write(tokens.to_string().as_bytes()).unwrap();
}

fn start_comment_block_for_logging() {
    stdout().write(b"/*\n").unwrap();
}

fn end_comment_block_for_logging() {
    stdout().write(b"*/").unwrap();
}

fn from_dir(path: &str) -> Result<Vec<(String, Endpoint)>, String> {
    let mut all_parsed: Vec<(String, Endpoint)> = Vec::new();

    let paths = read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let name = path.file_name().map(|path| path.to_string_lossy());
        let display = path.to_string_lossy().into_owned();

        if name.map(|name| !name.starts_with("_")).unwrap_or(true) {
            let mut f = File::open(&path).unwrap();
            let parsed = from_reader(display, &mut f)?;

            all_parsed.push(parsed);
        }
    }

    // Sort the endpoints parsed from disk so we have a stable ordering
    all_parsed.sort_by(|&(ref a_name, _), &(ref b_name, _)| a_name.cmp(b_name));

    Ok(all_parsed)
}

fn from_reader<R>(name: String, rdr: &mut R) -> Result<(String, Endpoint), String>
where
    R: Read,
{
    let endpoint: BTreeMap<String, Endpoint> = serde_json::from_reader(rdr)
        .map_err(|e| format!("Failed to parse {} because: {}", name, e))?;

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

fn endpoints_mod(
    tokens: &mut Tokens,
    http_mod: &'static str,
    endpoints: Vec<(String, Endpoint)>,
    params_to_emit: &mut BTreeMap<String, bool>,
) {
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

        let url_params = gen::endpoints::url_params::Builder::from(&e).build();
        let (ref url_params_item, _) = url_params;

        let (req_params_item, req_params_ty) = gen::endpoints::item::Builder::from(&e).build();

        let req_ctors_item =
            gen::endpoints::ctors::Builder::from((&e, &req_params_ty, &url_params)).build();
        let url_method_item = gen::endpoints::url_builder::Builder::from((&e, &url_params)).build();

        let req_into_http_item =
            gen::endpoints::into_endpoint::Builder::from((&e, &req_params_ty)).build();

        tokens.append_all(vec![
            quote!(#url_params_item),
            quote!(#url_method_item),
            quote!(#req_params_item),
            quote!(#req_ctors_item),
            quote!(#req_into_http_item),
        ]);
    }
}

fn http_mod(tokens: &mut Tokens) {
    let url_tokens = gen::http::url::tokens();

    let body_tokens = gen::http::body::tokens();

    let uses = quote!(
        use std::borrow::Cow;
        use std::ops::Deref;

        pub use crate::http::Method;
    );

    let http_req_item = gen::http::endpoint::tokens();

    tokens.append(uses.to_string());

    tokens.append("\n\n");

    tokens.append_all(vec![url_tokens, http_req_item, body_tokens]);
}

fn params_mod(tokens: &mut Tokens, params_to_emit: BTreeMap<String, bool>) {
    tokens.append("\n\n");

    tokens.append(r#"include!("genned.params.rs");"#);
    tokens.append("\n\n");

    let params_to_emit = params_to_emit.iter().filter(|&(_, is_emitted)| *is_emitted);

    for (ty, _) in params_to_emit {
        let ty_item = gen::params::tokens(ty);

        tokens.append(quote!(#ty_item));

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
