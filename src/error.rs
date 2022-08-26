//! Errors that can occure while operating on modular bitfields.

use core::fmt::Debug;

/// The given value was out of range for the bitfield.
#[derive(Debug, PartialEq, Eq)]
pub struct OutOfBounds;

impl core::fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "encountered an out of bounds value")
    }
}

/// The bitfield contained an invalid bit pattern.
#[derive(Debug, PartialEq, Eq)]
pub struct InvalidBitPattern<Bytes> {
    pub invalid_bytes: Bytes,
}

impl<Bytes> core::fmt::Display for InvalidBitPattern<Bytes>
where
    Bytes: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "encountered an invalid bit pattern: {:X?}",
            self.invalid_bytes
        )
    }
}

impl<Bytes> InvalidBitPattern<Bytes> {
    /// Returns the invalid bit pattern.
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn invalid_bytes(self) -> Bytes {
        self.invalid_bytes
    }

    /// Creates a new invalid bit pattern error.
    #[inline]
    pub const fn new(invalid_bytes: Bytes) -> Self {
        Self { invalid_bytes }
    }
}
