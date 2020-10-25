use super::checks;

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
#[doc(hidden)]
pub trait PushBits: checks::private::Sealed {
    fn push_bits(&mut self, amount: u32, bits: u8);
}

/// Helper trait for underlying primitives handling of bitfields.
///
/// # Note
///
/// Must not and cannot be implemented by dependencies.
#[doc(hidden)]
pub trait PopBits: checks::private::Sealed {
    fn pop_bits(&mut self, amount: u32) -> u8;
}

/// Trait implemented by primitives that drive bitfield manipulations generically.
#[doc(hidden)]
pub trait SpecifierBytes: checks::private::Sealed {
    /// The base type that the specifier is operating on.
    type Bytes;
}
