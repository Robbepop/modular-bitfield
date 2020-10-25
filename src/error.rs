/// Error that can be encountered operating on bitfields.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// A setter received an input that is invalid for the associated bitfield specifier.
    ///
    /// # Example
    ///
    /// Consider a field `a: B2` of a bitfield struct that uses 2 bits.
    /// It having 2 bits the valid bounds of `a` are `0..4`.
    /// The error is returned if a user tries to set its value to a value
    /// that is not within the range `0..4`, e.g. 5.
    OutOfBounds,
    /// Encountered upon using `from_bytes` if too many or too few bytes have been given.
    InvalidBufferLen,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "Encountered an out of bounds value"),
            Error::InvalidBufferLen => {
                write!(f, "Too many or too few bytes given to construct from bytes")
            }
        }
    }
}

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
        write!(f, "encountered an invalid bit pattern: {:X?}", self.invalid_bytes)
    }
}

impl<Bytes> InvalidBitPattern<Bytes> {
    /// Creates a new invalid bit pattern error.
    #[inline]
    pub fn new(invalid_bytes: Bytes) -> Self {
        Self { invalid_bytes }
    }

    /// Returns the invalid bit pattern.
    #[inline]
    pub fn invalid_bytes(self) -> Bytes {
        self.invalid_bytes
    }
}
