use super::config::ConfigValue;
use crate::errors::CombineError;
use proc_macro2::Span;

#[derive(Default, Clone)]
pub struct FieldConfig {
    /// Attributes that are re-expanded and going to be ignored by the rest of the `#[bitfield]` invocation.
    pub retained_attrs: Vec<syn::Attribute>,
    /// An encountered `#[bits = N]` attribute on a field.
    pub bits: Option<ConfigValue<usize>>,
    /// An encountered `#[skip]` attribute on a field.
    pub skip: Option<ConfigValue<SkipWhich>>,
}

/// Controls which parts of the code generation to skip.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SkipWhich {
    /// Skip code generation of getters and setters.
    All,
    /// Skip code generation of only getters.
    ///
    /// For field `f` these include:
    ///
    /// - `f`
    /// - `f_or_err`
    Getters,
    /// Skip code generation of only setters.
    ///
    /// For field `f` these include:
    ///
    /// - `set_f`
    /// - `set_f_checked`
    /// - `with_f`
    /// - `with_f_checked`
    Setters,
}

impl SkipWhich {
    /// Returns `true` if code generation of getters should be skipped.
    pub fn skip_getters(self) -> bool {
        matches!(self, Self::All | Self::Getters)
    }

    /// Returns `true` if code generation of setters should be skipped.
    pub fn skip_setters(self) -> bool {
        matches!(self, Self::All | Self::Setters)
    }

    /// Returns `true` if code generation of getters and setters should be skipped.
    pub fn skip_getters_and_setters(self) -> bool {
        matches!(self, Self::All)
    }
}

impl FieldConfig {
    /// Registers the given attribute to be re-expanded and further ignored.
    pub fn retain_attr(&mut self, attr: syn::Attribute) {
        self.retained_attrs.push(attr);
    }

    /// Sets the `#[bits = N]` if found for a `#[bitfield]` annotated field.
    ///
    /// # Errors
    ///
    /// If previously already registered a `#[bits = M]`.
    pub fn bits(&mut self, amount: usize, span: Span) -> Result<(), syn::Error> {
        match self.bits {
            Some(ref previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate `#[bits = N]` attribute for field"
                )
                .into_combine(format_err!(previous.span, "duplicate `#[bits = M]` here")))
            }
            None => {
                self.bits = Some(ConfigValue {
                    value: amount,
                    span,
                })
            }
        }
        Ok(())
    }

    /// Sets the `#[skip(which)]` if found for a `#[bitfield]` annotated field.
    ///
    /// # Syntax
    ///
    /// - `#[skip]` defaults to `SkipWhich::All`.
    /// - `#[skip(all)]` is the same as `#[skip]`.
    /// - `#[skip(getters)]` is `SkipWhich::Getters`.
    /// - `#[skip(setters)]` is `SkipWhich::Setters`.
    /// - `#[skip(getters, setters)]` is the same as `#[skip(all)]`.
    ///
    /// # Errors
    ///
    /// If previously already registered a `#[bits = M]`.
    pub fn skip(&mut self, which: SkipWhich, span: Span) -> Result<(), syn::Error> {
        match self.skip {
            Some(ref previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate `#[bits = N]` attribute for field"
                )
                .into_combine(format_err!(previous.span, "duplicate `#[bits = M]` here")))
            }
            None => self.skip = Some(ConfigValue { value: which, span }),
        }
        Ok(())
    }

    /// Returns `true` if the config demands that code generation for setters should be skipped.
    pub fn skip_setters(&self) -> bool {
        self.skip
            .as_ref()
            .map(|config| config.value)
            .map(SkipWhich::skip_setters)
            .unwrap_or(false)
    }

    /// Returns `true` if the config demands that code generation for getters should be skipped.
    pub fn skip_getters(&self) -> bool {
        self.skip
            .as_ref()
            .map(|config| config.value)
            .map(SkipWhich::skip_getters)
            .unwrap_or(false)
    }

    /// Returns `true` if the config demands that code generation for getters should be skipped.
    pub fn skip_getters_and_setters(&self) -> bool {
        self.skip
            .as_ref()
            .map(|config| config.value)
            .map(SkipWhich::skip_getters_and_setters)
            .unwrap_or(false)
    }
}