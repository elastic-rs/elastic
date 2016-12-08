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

use parse::*;

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

fn main() {
    start_comment_block_for_logging();

    println!("This code is automatically generated");

    let dir = "./spec";

    // BTreeMap<String, bool> : <type name, is emitted>
    let mut params_to_emit = BTreeMap::new();
    params_to_emit.insert(String::from("vertices"), false);

    let mut tokens = quote::Tokens::new();

    let uses = quote!(
    	use std::marker::PhantomData;
    	use std::ops::Deref;
    	use std::borrow::Cow;
    );

    tokens.append(uses.to_string().as_ref());
    tokens.append("\n\n");

    let body_tokens = gen::types::body::tokens();
    let http_method_item = gen::types::request::method_item();
    let http_req_item = gen::types::request::req_item();

    tokens.append_all(vec![
    	body_tokens, 
    	quote!(#http_method_item), 
    	quote!(#http_req_item)
    ]);
    tokens.append("\n\n");

    let endpoints: Vec<(String, Endpoint)> = from_dir(dir)
        .expect("Couldn't parse the REST API spec")
        .into_iter()
        .map(|e| strip_verbs(e))
        .map(|e| dedup_urls(e))
        .collect();


    for e in endpoints {
        for (ty, _) in &e.1.url.parts {
            params_to_emit.insert(ty.to_owned(), true);
        }

        let url_params = gen::url_params::UrlParamBuilder::from(&e).build();
        let (ref url_params_item, ref url_params_ty) = url_params;

        let (req_params_item, req_params_ty) =
            gen::request_params::RequestParamBuilder::from((&e, url_params_ty)).build();

        let req_ctors_item =
            gen::request_ctors::RequestParamsCtorBuilder::from((&e, &req_params_ty, &url_params))
                .build();

        let req_url_method_item =
            gen::url_builder::UrlMethodBuilder::from((&e, &req_params_ty, &url_params)).build();

        let req_into_http_item =
            gen::request_into_http::RequestIntoHttpRequestBuilder::from((&e, &req_params_ty))
                .build();

        tokens.append_all(vec![
        	quote!(#url_params_item),
        	quote!(#req_params_item),
        	quote!(#req_ctors_item),
        	quote!(#req_url_method_item),
        	quote!(#req_into_http_item)
        ]);
        tokens.append("\n\n");
    }

    let params_to_emit = params_to_emit.iter()
        .filter(|&(_, is_emitted)| *is_emitted);

    for (ty, _) in params_to_emit {
        let ty_item = gen::types::wrapped_ty::item(ty);

        tokens.append(ty_item.to_string().as_ref());
        tokens.append("\n\n");
    }

    end_comment_block_for_logging();

    stdout().write(tokens.to_string().as_bytes()).unwrap();
}
