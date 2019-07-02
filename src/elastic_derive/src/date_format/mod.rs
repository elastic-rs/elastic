use quote::Tokens;
use syn;

mod parse;

use super::{
    expect_name_value,
    get_elastic_meta_items,
    get_str_from_lit,
};

/**
Derive `DateFormat` for the given input.

The input must satisfy the following rules:

- It must be a unit struct.
- It must have an `#[elastic(date_format="<value>")]` attribute.
*/
pub fn expand_derive(
    crate_root: Tokens,
    input: &syn::MacroInput,
) -> Result<Vec<Tokens>, DeriveDateFormatError> {
    // Annotatable item for a unit struct
    match input.body {
        syn::Body::Struct(ref data) => match *data {
            syn::VariantData::Unit => Ok(()),
            _ => Err(DeriveDateFormatError::InvalidInput),
        },
        _ => Err(DeriveDateFormatError::InvalidInput),
    }?;

    let format = get_format_from_attr(input).ok_or(DeriveDateFormatError::MissingFormat)?;

    let name = get_name_from_attr(input).unwrap_or_else(|| format.clone());

    let tokens: Vec<Tokens> = parse::to_tokens(&format)?
        .into_iter()
        .map(|t| t.into_tokens(&crate_root))
        .collect();

    let derived = impl_date_format(crate_root, input, &name, &tokens);

    Ok(vec![derived])
}

// Implement DateFormat for the type being derived with the mapping
fn impl_date_format(
    crate_root: Tokens,
    item: &syn::MacroInput,
    name: &str,
    format: &[Tokens],
) -> Tokens {
    let ty = &item.ident;

    let parse_fn = quote!(
        fn parse(date: &str) -> ::std::result::Result<#crate_root::__derive::DateValue, #crate_root::__derive::ParseError> {
            let fmt = vec![ #(#format),* ];

            #crate_root::__derive::parse_from_tokens(date, fmt)
        }
    );

    let format_fn = quote!(
        fn format<'a>(date: &'a #crate_root::__derive::DateValue) -> #crate_root::__derive::FormattedDate<'a> {
            let fmt = vec![ #(#format),* ];

            #crate_root::__derive::format_with_tokens(date, fmt)
        }
    );

    let name_fn = quote!(
        fn name() -> &'static str {
            #name
        }
    );

    quote!(
        impl #crate_root::__derive::DateFormat for #ty {
            #parse_fn

            #format_fn

            #name_fn
        }
    )
}

// Get the format string supplied by an #[elastic()] attribute
fn get_format_from_attr<'a>(item: &'a syn::MacroInput) -> Option<String> {
    let val = get_elastic_meta_items(&item.attrs);

    let val = val
        .iter()
        .filter_map(|meta| expect_name_value("date_format", &meta))
        .next();

    val.and_then(|v| get_str_from_lit(v).ok().map(Into::into))
}

// Get the name string supplied by an #[elastic()] attribute
fn get_name_from_attr<'a>(item: &'a syn::MacroInput) -> Option<String> {
    let val = get_elastic_meta_items(&item.attrs);

    let val = val
        .iter()
        .filter_map(|meta| expect_name_value("date_format_name", &meta))
        .next();

    val.and_then(|v| get_str_from_lit(v).ok().map(Into::into))
}

impl<'a> parse::DateFormatToken<'a> {
    fn into_tokens(self, crate_root: &Tokens) -> Tokens {
        use self::parse::DateFormatToken::*;

        match self {
            Year => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Year, #crate_root::__derive::Pad::Zero))
            }
            Month => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Month, #crate_root::__derive::Pad::Zero))
            }
            DayOfMonth => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Day, #crate_root::__derive::Pad::Zero))
            }
            DayOfYear => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Ordinal, #crate_root::__derive::Pad::Zero))
            }
            Hour => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Hour, #crate_root::__derive::Pad::Zero))
            }
            Minute => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Minute, #crate_root::__derive::Pad::Zero))
            }
            Second => {
                quote!(#crate_root::__derive::Item::Numeric(#crate_root::__derive::Numeric::Second, #crate_root::__derive::Pad::Zero))
            }
            Millisecond => {
                quote!(#crate_root::__derive::Item::Fixed(#crate_root::__derive::Fixed::Nanosecond3))
            }
            Utc => quote!(#crate_root::__derive::Item::Literal("Z")),
            Delim(s) => quote!(#crate_root::__derive::Item::Literal(#s)),
            Escaped(s) => quote!(#crate_root::__derive::Item::Literal(#s)),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum DeriveDateFormatError {
        InvalidInput {
            display("deriving a date format is only valid for unit structs")
        }
        MissingFormat {
            display("missing date format. Add a `#[elastic(date_format=\"<format here>\")]`")
        }
        InvalidFormat(err: parse::Error) {
            display("error parsing date format")
            from()
        }
    }
}
