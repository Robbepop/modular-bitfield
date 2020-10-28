use crate::config::Config;
use core::convert::TryFrom;
use syn::spanned::Spanned;

/// Raises an unsupported argument compile time error.
fn unsupported_argument<T>(arg: T) -> syn::Error
where
    T: Spanned,
{
    format_err!(arg, "encountered unsupported #[bitfield] attribute")
}

/// The parameters given to the `#[bitfield]` proc. macro.
///
/// # Example
///
/// ```rust
/// # use modular_bitfield::prelude::*;
/// #
/// #[bitfield(specifier = true, bytes = 4, filled = true)]
/// pub struct SignedInteger {
///     sign: bool,
///     value: B31,
/// }
/// ```
pub struct AttributeArgs {
    args: syn::AttributeArgs,
}

impl syn::parse::Parse for AttributeArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let punctuated =
            <syn::punctuated::Punctuated<_, syn::Token![,]>>::parse_terminated(input)?;
        Ok(Self {
            args: punctuated.into_iter().collect::<Vec<_>>(),
        })
    }
}

impl TryFrom<AttributeArgs> for Config {
    type Error = syn::Error;

    fn try_from(attribute_args: AttributeArgs) -> Result<Self, Self::Error> {
        let mut builder = Config::default();
        let AttributeArgs { args } = attribute_args;
        for nested_meta in args {
            match nested_meta {
                syn::NestedMeta::Meta(meta) => {
                    match meta {
                        syn::Meta::NameValue(name_value) => {
                            if name_value.path.is_ident("specifier") {
                                match &name_value.lit {
                                    syn::Lit::Bool(lit_bool) => {
                                        builder.specifier(lit_bool.value, name_value.span())?;
                                    }
                                    invalid => {
                                        return Err(format_err!(
                                            invalid,
                                            "encountered invalid value argument for #[bitfield] `specifier` parameter",
                                        ))
                                    }
                                }
                            } else if name_value.path.is_ident("bytes") {
                                match &name_value.lit {
                                    syn::Lit::Int(lit_int) => {
                                        let span = lit_int.span();
                                        let value = lit_int.base10_parse::<usize>().map_err(|err| {
                                            format_err!(span, "encountered malformatted integer value for bytes parameter: {}", err)
                                        })?;
                                        builder.bytes(value, name_value.span())?;
                                    }
                                    invalid => {
                                        return Err(format_err!(
                                            invalid,
                                            "encountered invalid value argument for #[bitfield] `bytes` parameter",
                                        ))
                                    }
                                }
                            } else if name_value.path.is_ident("filled") {
                                match &name_value.lit {
                                    syn::Lit::Bool(lit_bool) => {
                                        builder.filled(lit_bool.value, name_value.span())?;
                                    }
                                    invalid => {
                                        return Err(format_err!(
                                            invalid,
                                            "encountered invalid value argument for #[bitfield] `filled` parameter",
                                        ))
                                    }
                                }
                            } else {
                                return Err(unsupported_argument(name_value))
                            }
                        }
                        unsupported => return Err(unsupported_argument(unsupported)),
                    }
                }
                unsupported => return Err(unsupported_argument(unsupported)),
            }
        }
        Ok(builder)
    }
}
