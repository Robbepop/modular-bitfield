use crate::errors::CombineError;
use core::convert::TryFrom;
use proc_macro2::Span;
use syn::spanned::Spanned;

#[derive(Default)]
pub struct Config {
    pub specifier: Option<ConfigValue<bool>>,
    pub bytes: Option<ConfigValue<usize>>,
}

pub struct ConfigValue<T> {
    pub value: T,
    pub span: Span,
}

impl<T> ConfigValue<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl Config {
    /// Returns `true` if the `specifier` parameter has been set to `true` and otherwise `false`.
    pub fn specifier_enabled(&self) -> bool {
        self.specifier.as_ref().map(|config| config.value).unwrap_or(false)
    }
}

impl Config {
    /// Sets the specifier #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn specifier(&mut self, value: bool, span: Span) -> Result<(), syn::Error> {
        match &self.specifier {
            Some(previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate specifier parameter: duplicate set to {:?}",
                    previous.value
                )
                .into_combine(format_err!(previous.span, "previous specifier parameter here")))
            }
            None => self.specifier = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Sets the bytes #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn bytes(&mut self, value: usize, span: Span) -> Result<(), syn::Error> {
        match &self.bytes {
            Some(previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate bytes parameter: duplicate set to {:?}",
                    previous.value
                )
                .into_combine(format_err!(previous.span, "previous bytes parameter here")))
            }
            None => self.bytes = Some(ConfigValue::new(value, span)),
        }
        Ok(())
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
                                            "encountered invalid value argument for #[bitfield] specifier parameter",
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
        Ok(builder)
    }
}
