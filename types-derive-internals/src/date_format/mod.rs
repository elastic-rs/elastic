use quote::Tokens;
use syn;

mod parse;

use super::{get_elastic_attr_name_value, get_str_from_lit};

/// Derive `DateFormat` for the given input.
/// 
/// The input must satisfy the following rules:
/// 
/// - It must be a unit struct.
/// - It must have an `#[elastic(date_format="<value>")]` attribute.
pub fn expand_derive(crate_root: Tokens,
                     input: &syn::MacroInput)
                     -> Result<Vec<Tokens>, DeriveDateFormatError> {
    // Annotatable item for a unit struct
    match input.body {
        syn::Body::Struct(ref data) => {
            match *data {
                syn::VariantData::Unit => Ok(()),
                _ => Err(DeriveDateFormatError::InvalidInput),
            }
        }
        _ => Err(DeriveDateFormatError::InvalidInput),
    }?;

    let format = get_format_from_attr(input).ok_or(DeriveDateFormatError::MissingFormat)?;

    let name = get_name_from_attr(input).unwrap_or(format);

    let tokens: Vec<Tokens> = parse::to_tokens(format)
        ?
        .into_iter()
        .map(|t| t.into())
        .collect();

    let derived = impl_date_format(crate_root, input, name, &tokens);

    Ok(vec![derived])
}

// Implement DateFormat for the type being derived with the mapping
fn impl_date_format(crate_root: Tokens,
                    item: &syn::MacroInput,
                    name: &str,
                    format: &[Tokens])
                    -> Tokens {
    let ty = &item.ident;

    let parse_fn = quote!(
        fn parse(date: &str) -> ::std::result::Result<::chrono::DateTime<::chrono::UTC>, #crate_root::derive::ParseError> {
            let fmt = vec![ #(#format),* ];

            #crate_root::derive::parse_from_tokens(date, fmt)
        }
    );

    let format_fn = quote!(
        fn format<'a>(date: &::chrono::DateTime<::chrono::UTC>) -> #crate_root::derive::FormattedDate<'a> {
            let fmt = vec![ #(#format),* ];

            #crate_root::derive::format_with_tokens(date, fmt)
        }
    );

    let name_fn = quote!(
        fn name() -> &'static str {
            #name
        }
    );

    quote!(
        impl #crate_root::derive::DateFormat for #ty {
            #parse_fn

            #format_fn

            #name_fn
        }
    )
}

// Get the format string supplied by an #[elastic()] attribute
fn get_format_from_attr<'a>(item: &'a syn::MacroInput) -> Option<&'a str> {
    let val = get_elastic_attr_name_value("date_format", item);

    val.and_then(|v| get_str_from_lit(v).ok())
}

// Get the name string supplied by an #[elastic()] attribute
fn get_name_from_attr<'a>(item: &'a syn::MacroInput) -> Option<&'a str> {
    let val = get_elastic_attr_name_value("date_format_name", item);

    val.and_then(|v| get_str_from_lit(v).ok())
}

impl<'a> Into<Tokens> for parse::DateFormatToken<'a> {
    fn into(self) -> Tokens {
        use self::parse::DateFormatToken::*;

        match self {
            Year => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Year, ::chrono::format::Pad::Zero)),
            Month => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Month, ::chrono::format::Pad::Zero)),
            DayOfMonth => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Day, ::chrono::format::Pad::Zero)),
            DayOfYear => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Ordinal, ::chrono::format::Pad::Zero)),
            Hour => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Hour, ::chrono::format::Pad::Zero)),
            Minute => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Minute, ::chrono::format::Pad::Zero)),
            Second => quote!(::chrono::format::Item::Numeric(::chrono::format::Numeric::Second, ::chrono::format::Pad::Zero)),
            Millisecond => {
                quote!(::chrono::format::Item::Fixed(::chrono::format::Fixed::Nanosecond3))
            }
            Utc => quote!(::chrono::format::Item::Literal("Z")),
            Delim(s) => quote!(::chrono::format::Item::Literal(#s)),
            Escaped(s) => quote!(::chrono::format::Item::Literal(#s)),
        }
    }
}

quick_error!{
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
