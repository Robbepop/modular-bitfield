#![allow(dead_code)]

use super::field_config::FieldConfig;
use crate::errors::CombineError;
use core::any::TypeId;
use proc_macro2::Span;
use std::collections::{
    hash_map::Entry,
    HashMap,
};

/// The configuration for the `#[bitfield]` macro.
#[derive(Default)]
pub struct Config {
    pub bytes: Option<ConfigValue<usize>>,
    pub bits: Option<ConfigValue<usize>>,
    pub filled: Option<ConfigValue<bool>>,
    pub repr: Option<ConfigValue<ReprKind>>,
    pub derive_debug: Option<ConfigValue<()>>,
    pub derive_specifier: Option<ConfigValue<()>>,
    pub retained_attributes: Vec<syn::Attribute>,
    pub field_configs: HashMap<usize, ConfigValue<FieldConfig>>,
}

/// Kinds of `#[repr(uN)]` annotations for a `#[bitfield]` struct.
#[derive(Copy, Clone)]
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

impl core::fmt::Debug for ReprKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::U8 => write!(f, "#[repr(u8)]"),
            Self::U16 => write!(f, "#[repr(u16)]"),
            Self::U32 => write!(f, "#[repr(u32)]"),
            Self::U64 => write!(f, "#[repr(u64)]"),
            Self::U128 => write!(f, "#[repr(u128)]"),
        }
    }
}

/// A configuration value and its originating span.
#[derive(Clone)]
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
    /// Returns the value of the `filled` parameter if provided and otherwise `true`.
    pub fn filled_enabled(&self) -> bool {
        self.filled
            .as_ref()
            .map(|config| config.value)
            .unwrap_or(true)
    }
}

impl Config {
    /// Returns an error showing both the duplicate as well as the previous parameters.
    fn raise_duplicate_error<T>(
        name: &str,
        span: Span,
        previous: &ConfigValue<T>,
    ) -> syn::Error
    where
        T: core::fmt::Debug + 'static,
    {
        if TypeId::of::<T>() == TypeId::of::<()>() {
            format_err!(span, "encountered duplicate `{}` parameter", name,)
        } else {
            format_err!(
                span,
                "encountered duplicate `{}` parameter: duplicate set to {:?}",
                name,
                previous.value
            )
        }
        .into_combine(format_err!(
            previous.span,
            "previous `{}` parameter here",
            name
        ))
    }

    /// Sets the `bytes: int` #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn bytes(&mut self, value: usize, span: Span) -> Result<(), syn::Error> {
        match &self.bytes {
            Some(previous) => {
                return Err(Self::raise_duplicate_error("bytes", span, previous))
            }
            None => self.bytes = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Sets the `bits: int` #[bitfield] parameter to the given value.
    ///
    /// # Errors
    ///
    /// If the specifier has already been set.
    pub fn bits(&mut self, value: usize, span: Span) -> Result<(), syn::Error> {
        match &self.bits {
            Some(previous) => {
                return Err(Self::raise_duplicate_error("bits", span, previous))
            }
            None => self.bits = Some(ConfigValue::new(value, span)),
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
                return Err(Self::raise_duplicate_error("filled", span, previous))
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
    pub fn repr(&mut self, value: ReprKind, span: Span) -> Result<(), syn::Error> {
        match &self.repr {
            Some(previous) => {
                return Err(Self::raise_duplicate_error("#[repr(uN)]", span, previous))
            }
            None => self.repr = Some(ConfigValue::new(value, span)),
        }
        Ok(())
    }

    /// Registers the `#[derive(Debug)]` attribute for the #[bitfield] macro.
    ///
    /// # Errors
    ///
    /// If a `#[derive(Debug)]` attribute has already been found.
    pub fn derive_debug(&mut self, span: Span) -> Result<(), syn::Error> {
        match &self.derive_debug {
            Some(previous) => {
                return Err(Self::raise_duplicate_error(
                    "#[derive(Debug)]",
                    span,
                    previous,
                ))
            }
            None => self.derive_debug = Some(ConfigValue::new((), span)),
        }
        Ok(())
    }

    /// Registers the `#[derive(BitfieldSpecifier)]` attribute for the #[bitfield] macro.
    ///
    /// # Errors
    ///
    /// If a `#[derive(BitfieldSpecifier)]` attribute has already been found.
    pub fn derive_specifier(&mut self, span: Span) -> Result<(), syn::Error> {
        match &self.derive_specifier {
            Some(previous) => {
                return Err(Self::raise_duplicate_error(
                    "#[derive(BitfieldSpecifier)]",
                    span,
                    previous,
                ))
            }
            None => self.derive_specifier = Some(ConfigValue::new((), span)),
        }
        Ok(())
    }

    /// Pushes another retained attribute that the #[bitfield] macro is going to re-expand and ignore.
    pub fn push_retained_attribute(&mut self, retained_attr: syn::Attribute) {
        self.retained_attributes.push(retained_attr);
    }

    /// Sets the field configuration and retained attributes for the given field.
    ///
    /// By convention we use the fields name to identify the field if existing.
    /// Otherwise we turn the fields discriminant into an appropriate string.
    ///
    /// # Errors
    ///
    /// If duplicate field configurations have been found for a field.
    pub fn field_config(
        &mut self,
        index: usize,
        span: Span,
        config: FieldConfig,
    ) -> Result<(), syn::Error> {
        match self.field_configs.entry(index) {
            Entry::Occupied(occupied) => {
                return Err(format_err!(span, "encountered duplicate config for field")
                    .into_combine(format_err!(
                        occupied.get().span,
                        "previous config here"
                    )))
            }
            Entry::Vacant(vacant) => {
                vacant.insert(ConfigValue::new(config, span));
            }
        }
        Ok(())
    }
}
