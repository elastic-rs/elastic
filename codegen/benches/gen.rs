#![feature(test)]
extern crate test;

use test::Bencher;

/*
	Thanks @DanielKeep for fixing this up, and providing the only useful implementations.

	Here we're testing options for building up a url efficiently.
*/

#[bench]
fn format_url_macro(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        format!("/{}/_alias/{}", index, name)
    });
}

#[bench]
fn format_url_concat(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        let mut url = "/".to_string();
        url = url + &index[..] + "/_alias/" + &name[..];
        url
    });
}

#[bench]
fn format_url_push(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
    	//Allocate the string with correct length
        let mut url = String::with_capacity(
        	"/".len() + 
        	"/_alias/".len() + 
        	index.len() + 
        	name.len()
        );

        //Push the parts/params in order
        url.push_str("/");
        url.push_str(&index);
        url.push_str("/_alias/");
        url.push_str(&name);

        url
    });
}

#[bench]
fn format_url_write(b: &mut Bencher) {
    let index = "test_idx".to_string();
    let name = "test_alias".to_string();

    b.iter(|| {
        use std::fmt::Write;
        let mut url = String::with_capacity(1 + "/_alias/".len()
            + index.len() + name.len());
        write!(url, "/{}/_alias/{}", index, name).unwrap();
        url
    });
}