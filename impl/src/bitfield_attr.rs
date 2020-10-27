use crate::errors::CombineError;
use core::convert::TryFrom;
use proc_macro2::Span;
use syn::spanned::Spanned;

pub struct Config {
    pub specifier: bool,
    pub bytes: Option<usize>,
}

#[derive(Default)]
pub struct ConfigBuilder {
    specifier: Option<(bool, Span)>,
    bytes: Option<(usize, Span)>,
}

impl ConfigBuilder {
    /// Sets the specifier #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn specifier(&mut self, value: bool, span: Span) -> Result<(), syn::Error> {
        match self.specifier {
            Some((specifier, previous)) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate specifier parameter: duplicate set to {:?}",
                    specifier
                )
                .into_combine(format_err!(previous, "previous specifier parameter here")))
            }
            None => self.specifier = Some((value, span)),
        }
        Ok(())
    }

    /// Sets the bytes #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn bytes(&mut self, value: usize, span: Span) -> Result<(), syn::Error> {
        match self.bytes {
            Some((bytes, previous)) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate bytes parameter: duplicate set to {:?}",
                    bytes
                )
                .into_combine(format_err!(previous, "previous bytes parameter here")))
            }
            None => self.bytes = Some((value, span)),
        }
        Ok(())
    }

    /// Converts the config builder into a config.
    pub fn into_config(self) -> Config {
        Config {
            specifier: self.specifier.map(|(value, _)| value).unwrap_or(false),
            bytes: self.bytes.map(|(value, _)| value),
        }
    }
}

fn unsupported_argument<T>(arg: T) -> syn::Error
where
    T: Spanned,
{
    format_err!(arg, "encountered unsupported #[bitfield] attribute")
}

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
        let mut builder = ConfigBuilder::default();
        let AttributeArgs { args } = attribute_args;
        for nested_meta in args {
            match nested_meta {
                syn::NestedMeta::Meta(meta) => {
                    match meta {
                        syn::Meta::NameValue(name_value) => {
                            if name_value.path.is_ident("specifier") {
                                match name_value.lit {
                                    syn::Lit::Bool(lit_bool) => {
                                        builder.specifier(lit_bool.value, lit_bool.span())?;
                                    }
                                    invalid => {
                                        return Err(format_err!(
                                            invalid,
                                            "encountered invalid value argument for #[bitfield] specifier parameter",
                                        ))
                                    }
                                }
                            } else if name_value.path.is_ident("bytes") {
                                match name_value.lit {
                                    syn::Lit::Int(lit_int) => {
                                        let span = lit_int.span();
                                        let value = lit_int.base10_parse::<usize>().map_err(|err| {
                                            format_err!(span, "encountered malformatted integer value for bytes parameter: {}", err)
                                        })?;
                                        builder.bytes(value, span)?;
                                    }
                                    invalid => {
                                        return Err(format_err!(
                                            invalid,
                                            "encountered invalid value argument for #[bitfield] bytes parameter",
                                        ))
                                    }
                                }
                            }
                        }
                        unsupported => return Err(unsupported_argument(unsupported)),
                    }
                }
                unsupported => {
                    return Err(format_err!(
                        unsupported,
                        "encountered unsupported #[bitfield] attribute"
                    ))
                }
            }
        }
        Ok(builder.into_config())
    }
}
