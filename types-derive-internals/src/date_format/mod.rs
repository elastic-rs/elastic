use quote::Tokens;
use syn;
use serde_codegen_internals::{self, attr as serde_attr};

pub fn expand_derive(crate_root: Tokens, input: &syn::MacroInput) -> Vec<Tokens> {
    // parse #[elastic(date_format = "yyyy-MM-dd'T'HH:mm:SS.sssZ")]
    // parse #[elastic(date_format_name = "datetime")]

    // parse "yyyy-MM-dd'T'HH:mm:SS.sssZ" to chrono tokens
    // implement `DateFormat` for the struct
    unimplemented!();
}