use crate::{
    error::{
        InvalidBitPattern,
        OutOfBounds,
    },
    Specifier,
};

impl Specifier for bool {
    const BITS: usize = 1;
    type Bytes = u8;
    type InOut = Self;

    #[inline]
    fn from_bytes(
        bytes: Self::Bytes,
    ) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
        match bytes {
            0 => Ok(false),
            1 => Ok(true),
            invalid_bytes => Err(InvalidBitPattern { invalid_bytes }),
        }
    }

    #[inline]
    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
        Ok(u8::from(input))
    }
}

macro_rules! impl_specifier_for_primitive {
    ( $( ($prim:ty: $bits:literal) ),* $(,)? ) => {
        $(
            #[automatically_derived]
            impl Specifier for $prim {
                const BITS: usize = $bits;
                type Bytes = Self;
                type InOut = Self;

                #[inline]
                fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                    Ok(input)
                }

                #[inline]
                fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                    Ok(bytes)
                }
            }
        )*
    };
}
impl_specifier_for_primitive!(
    (u8: 8),
    (u16: 16),
    (u32: 32),
    (u64: 64),
    (u128: 128),
);
