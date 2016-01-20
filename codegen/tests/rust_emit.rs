#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use syntax::parse::token;
use syntax::parse::ParseSess;
use syntax::feature_gate::GatedCfgAttr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::expand::ExpansionConfig;
use syntax::ext::quote::rt::ToTokens;
use syntax::print::pprust;
use elastic_codegen::emit::*;
use elastic_codegen::emit::rust::*;
use elastic_codegen::gen::rust::*;

#[test]
fn can_emit_ast_tokens() {
	//Create an ExtCtxt to use in the emitter
	let sess = ParseSess::new();
	let mut attrs: Vec<GatedCfgAttr> = Vec::new();
	
	let cx = ExtCtxt::new(
		&sess, 
		Vec::new(), 
		ExpansionConfig::default("".to_string()), 
		&mut attrs
	);
	
	//Create an emitter
	let emitter = RustEmitter::new(cx);
	let mut buf: Vec<u8> = Vec::new();
	
	let token = token::str_to_ident("some_ident");
	let _ = emitter.emit(&token, &mut buf).unwrap();
}

#[test]
fn can_emit_fn() {
	//Create an ExtCtxt to use in the emitter
	let sess = ParseSess::new();
	let mut attrs: Vec<GatedCfgAttr> = Vec::new();
	
	let cx = ExtCtxt::new(
		&sess, 
		Vec::new(), 
		ExpansionConfig::default("".to_string()), 
		&mut attrs
	);
	
	//Create an emitter
	let emitter = RustEmitter::new(cx);
	let mut buf: Vec<u8> = Vec::new();

	//Define a lifetime 'a
	let lifetime = lifetime("'a");

	//Get a function signature
	let mut fun = build_fn("my_fun", vec![
		arg_ptr::<i32>("arg1", Mutability::MutMutable, Some(lifetime)),
		build_arg("arg2", build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
	]);

	let _ = emitter.emit(&fun, &mut buf).unwrap();
}