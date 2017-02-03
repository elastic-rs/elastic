#![feature(proc_macro)]

#[cfg(test)]
#[macro_use]
extern crate json_str;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate syn;
#[macro_use]
extern crate quote;

extern crate inflector;

pub mod parse;
pub mod gen;

use std::collections::BTreeMap;
use std::io::{stdout, Read, Write};
use std::fs::{File, read_dir};

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
        .add_simple_search();

    endpoints = endpoints    
        .into_iter()
        .map(|e| strip_verbs(e))
        .map(|e| dedup_urls(e))
        .collect();

    let http_mod_name = "http";

    build_mod("endpoints", &mut tokens,
        |ref mut tokens| endpoints_mod(tokens, derives.clone(), http_mod_name, endpoints, &mut params_to_emit)
    );

    build_mod(http_mod_name, &mut tokens,
        |ref mut tokens| http_mod(tokens, derives.clone())
    );

    build_mod("params", &mut tokens,
        |ref mut tokens| params_mod(tokens, derives.clone(), params_to_emit)
    );

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
    where R: Read
{
    let endpoint: BTreeMap<String, Endpoint> = try!(serde_json::from_reader(rdr)
        .map_err(|e| format!("Failed to parse {} because: {}", name, e)));

    Ok(endpoint.endpoint())
}

fn strip_verbs(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    // Choose a single HTTP verb per endpoint: either POST or 1st entry
    let mut iter = endpoint.methods.into_iter();
    let verb = match iter.len() {
        0 => unreachable!(),
        1 => iter.next().unwrap(),
        _ => {
            if iter.any(|m| m == HttpMethod::Post) {
                HttpMethod::Post
            } else {
                iter.next().unwrap()
            }
        }
    };

    endpoint.methods = vec![verb];

    (name, endpoint)
}

fn dedup_urls(endpoint: (String, Endpoint)) -> (String, Endpoint) {
    let (name, mut endpoint) = endpoint;

    let mut deduped_paths = BTreeMap::new();

    for path in endpoint.url.paths {
        let key = path.params().join("");

        deduped_paths.insert(key, path);
    }

    endpoint.url.paths = deduped_paths.into_iter()
        .map(|(_, p)| p)
        .collect();

    (name, endpoint)
}

trait AddSimpleSearch {
    fn add_simple_search(self) -> Self;
}

impl AddSimpleSearch for Vec<(String, Endpoint)> {
    fn add_simple_search(mut self) -> Vec<(String, Endpoint)> {
        let mut endpoint = {
            let &(_, ref endpoint) = self
                .iter()
                .find(|ref endpoint| endpoint.0 == "search")
                .unwrap();

            endpoint.clone()
        };

        let name = String::from("simple_search");

        endpoint.methods = vec![HttpMethod::Get];
        endpoint.body = None;

        self.push((name, endpoint));

        self
    }
}

fn endpoints_mod(tokens: &mut Tokens, derives: Tokens, http_mod: &'static str, endpoints: Vec<(String, Endpoint)>, params_to_emit: &mut BTreeMap<String, bool>) {
    let mut http_mod_tokens = Tokens::new();
    http_mod_tokens.append(http_mod);
    
    let uses = quote!(
        use super:: #http_mod_tokens ::*;
        use super::params::*;
        use std::borrow::Cow;
    );

    tokens.append(uses.to_string().as_ref());
    tokens.append("\n\n");

    for e in endpoints {
        for (ty, _) in &e.1.url.parts {
            params_to_emit.insert(ty.to_owned(), true);
        }

        let url_params = gen::url_params::UrlParamBuilder::from(&e).build();
        let (ref url_params_item, _) = url_params;

        let (req_params_item, req_params_ty) =
            gen::request_params::RequestParamBuilder::from(&e).build();

        let req_ctors_item =
            gen::request_ctors::RequestParamsCtorBuilder::from((&e, &req_params_ty, &url_params))
                .build();

        let url_method_item =
            gen::url_builder::UrlMethodBuilder::from((&e, &url_params)).build();

        let req_into_http_item =
            gen::request_into_http::RequestIntoHttpRequestBuilder::from((&e, &req_params_ty))
                .build();

        tokens.append_all(vec![
            derives.clone(),
            quote!(#url_params_item),
            quote!(#url_method_item),
            derives.clone(),
            quote!(#req_params_item),
            quote!(#req_ctors_item),
            quote!(#req_into_http_item)
        ]);
    }
}

fn http_mod(tokens: &mut Tokens, derives: Tokens) {
    let url_tokens = gen::types::url::tokens();
    let body_tokens = gen::types::body::tokens();
    let http_method_item = gen::types::request::method_item();
    let http_req_item = gen::types::request::req_tokens();

    let uses = quote!(
        use std::ops::Deref;
        use std::borrow::Cow;
    );

    tokens.append(uses.to_string().as_ref());

    tokens.append("\n\n");

    tokens.append_all(vec![
        derives.clone(),
        url_tokens, 
        derives.clone(),
        body_tokens, 
        derives.clone(),
        http_req_item, 
        derives.clone(),
        quote!(#http_method_item)
    ]);
}

fn params_mod(tokens: &mut Tokens, derives: Tokens, params_to_emit: BTreeMap<String, bool>) {
    let uses = quote!(
        use std::borrow::Cow;
    );

    tokens.append(uses.to_string().as_ref());
    tokens.append("\n\n");

    let params_to_emit = params_to_emit.iter()
        .filter(|&(_, is_emitted)| *is_emitted);

    for (ty, _) in params_to_emit {
        let ty_item = gen::types::wrapped_ty::item(ty);

        tokens.append_all(vec![
            derives.clone(),
            quote!(#ty_item)
        ]);

        tokens.append("\n\n");
    }
}

fn build_mod<F>(mod_name: &'static str, tokens: &mut Tokens, bldr: F) 
    where F: FnOnce(&mut Tokens) -> ()
{
    tokens.append(&format!("pub mod {} {{", mod_name));

    bldr(tokens);

    tokens.append("}");

    tokens.append("\n\n");
}
