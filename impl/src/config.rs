#![allow(dead_code)]

use crate::errors::CombineError;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};

/// The configuration for the `#[bitfield]` macro.
#[derive(Default)]
pub struct Config {
    pub specifier: Option<ConfigValue<bool>>,
    pub bytes: Option<ConfigValue<usize>>,
    pub filled: Option<ConfigValue<bool>>,
    pub repr: Option<ConfigValue<Repr>>,
    pub retained_attributes: Vec<syn::Attribute>,
}

/// Kinds of `#[repr(uN)]` annotations for a `#[bitfield]` struct.
#[derive(Debug, Copy, Clone)]
pub enum ReprKind {
    /// Found a `#[repr(u8)]` annotation.
    U8,
    /// Found a `#[repr(u16)]` annotation.
    U16,
    /// Found a `#[repr(u32)]` annotation.
    U32,
    /// Found a `#[repr(u64)]` annotation.
    U64,
    /// Found a `#[repr(u128)]` annotation.
    U128,
}

/// A `#[repr(uN)]` annotations for a `#[bitfield]` struct.
#[derive(Debug)]
pub struct Repr {
    /// The optional condition for the `#[repr(uN)]` annotation.
    ///
    /// # Note
    ///
    /// A condition can be found in annotations like these:
    ///
    /// ```
    /// use modular_bitfield::prelude::*;
    ///
    /// #[bitfield]
    /// #[cfg_attr(not(feature = "std"), repr(u32))]
    /// pub struct SignedU32 {
    ///     sign: bool,
    ///     value: B31,
    /// }
    /// ```
    condition: Option<TokenStream2>,
    /// The kind of the `#[repr(uN)]` annotation.
    kind: ReprKind,
}

impl Repr {
    /// Creates an unconditional `#[repr(..)]` attribute.
    pub fn unconditional(kind: ReprKind) -> Self {
        Self {
            condition: None,
            kind,
        }
    }

    /// Creates a conditional `#[cfg_attr(condition, repr(..))]` attribute.
    pub fn conditional(condition: TokenStream2, kind: ReprKind) -> Self {
        Self {
            condition: Some(condition),
            kind,
        }
    }
}

/// A configuration value and its originating span.
pub struct ConfigValue<T> {
    /// The actual value of the config.
    pub value: T,
    /// The originating span of the config.
    pub span: Span,
}

impl<T> ConfigValue<T> {
    /// Creates a new config value.
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl Config {
    /// Returns the value of the `specifier` parameter if provided and otherwise `false`.
    pub fn specifier_enabled(&self) -> bool {
        self.specifier
            .as_ref()
            .map(|config| config.value)
            .unwrap_or(false)
    }

    /// Returns the value of the `filled` parameter if provided and otherwise `true`.
    pub fn filled_enabled(&self) -> bool {
        self.filled
            .as_ref()
            .map(|config| config.value)
            .unwrap_or(true)
    }
}

impl Config {
    /// Sets the `specifier: bool` #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn specifier(&mut self, value: bool, span: Span) -> Result<(), syn::Error> {
        match &self.specifier {
            Some(previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate `specifier` parameter: duplicate set to {:?}",
                    previous.value
                )
                .into_combine(format_err!(
                    previous.span,
                    "previous `specifier` parameter here"
                )))
            }
            None => self.specifier = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Sets the `bytes: int` #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn bytes(&mut self, value: usize, span: Span) -> Result<(), syn::Error> {
        match &self.bytes {
            Some(previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate `bytes` parameter: duplicate set to {:?}",
                    previous.value
                )
                .into_combine(format_err!(
                    previous.span,
                    "previous `bytes` parameter here"
                )))
            }
            None => self.bytes = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Sets the `filled: bool` #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn filled(&mut self, value: bool, span: Span) -> Result<(), syn::Error> {
        match &self.filled {
            Some(previous) => {
                return Err(format_err!(
                    span,
                    "encountered duplicate `filled` parameter: duplicate set to {:?}",
                    previous.value
                )
                .into_combine(format_err!(
                    previous.span,
                    "previous `filled` parameter here"
                )))
            }
            None => self.filled = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Registers the `#[repr(uN)]` attribute for the #[bitfield] macro.
    ///
    /// # Errors
    ///
    /// If a `#[repr(uN)]` attribute has already been found.
    pub fn repr(&mut self, value: Repr, span: Span) -> Result<(), syn::Error> {
        match &self.repr {
            Some(previous) => {
                return Err(format_err!(
                span,
                "encountered duplicate `#[repr(uN)]` attribute: duplicate set to {:?}",
                previous.value
            )
                .into_combine(format_err!(
                    previous.span,
                    "previous `#[repr(uN)]` parameter here"
                )))
            }
            None => self.repr = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Pushes another retained attribute that the #[bitfield] macro is going to re-expand and ignore.
    pub fn push_retained_attribute(&mut self, retained_attr: syn::Attribute) {
        self.retained_attributes.push(retained_attr);
    }
}
