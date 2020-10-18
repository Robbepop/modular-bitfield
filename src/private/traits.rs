use super::{
    checks,
    Bits,
};

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
pub trait SpecifierBase: checks::private::Sealed {
    /// The base type that the specifier is operating on.
    type Base;
}

/// Helper trait to convert to bits.
///
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait IntoBits<T> {
    fn into_bits(self) -> Bits<T>;
}

/// Helper trait to convert from bits.
///
/// # Note
///
/// Implemented by primitive specifier types.
#[doc(hidden)]
pub trait FromBits<T> {
    fn from_bits(bits: Bits<T>) -> Self;
}
