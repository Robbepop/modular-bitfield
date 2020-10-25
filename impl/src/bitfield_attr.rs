use crate::errors::CombineError;
use core::convert::TryFrom;
use proc_macro2::Span;
use syn::spanned::Spanned;

pub struct Config {
    pub specifier: bool,
}

#[derive(Default)]
pub struct ConfigBuilder {
    specifier: Option<(bool, Span)>,
}

impl ConfigBuilder {
    /// Sets the specifier to the given value.
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

    /// Converts the config builder into a config.
    pub fn into_config(self) -> Config {
        Config {
            specifier: self.specifier.map(|(value, _)| value).unwrap_or(false),
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
