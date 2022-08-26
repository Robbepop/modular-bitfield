use core::convert::TryInto;

use crate::{
    private::{
        PopBits,
        PopBuffer,
        PushBits,
        PushBuffer,
    },
    Specifier,
};

/// Creates a new push buffer with all bits initialized to 0.
#[inline]
fn push_buffer<T>() -> PushBuffer<<T as Specifier>::Bytes>
where
    T: Specifier,
    PushBuffer<T::Bytes>: Default,
{
    <PushBuffer<<T as Specifier>::Bytes> as Default>::default()
}

#[doc(hidden)]
#[inline]
#[must_use]
pub fn read_specifier<T>(bytes: &[u8], offset: usize) -> <T as Specifier>::Bytes
where
    T: Specifier,
    PushBuffer<T::Bytes>: Default + PushBits,
{
    let end = offset + <T as Specifier>::BITS;
    let least_sb = offset / 8; // compile-time
    let most_sb = (end - 1) / 8; // compile-time
    let lsb_offset = offset % 8; // compile-time
    let msb_offset = end % 8; // compile-time
    let msb_offset = if msb_offset == 0 { 8 } else { msb_offset };

    let mut buffer = push_buffer::<T>();

    if lsb_offset == 0 && msb_offset == 8 {
        // Edge-case for whole bytes manipulation.
        for byte in bytes[least_sb..=most_sb].iter().rev() {
            buffer.push_bits(8, *byte);
        }
    } else {
        if least_sb != most_sb {
            // Most-significant byte
            buffer.push_bits(msb_offset.try_into().unwrap(), bytes[most_sb]);
        }
        if most_sb - least_sb >= 2 {
            // Middle bytes
            for byte in bytes[(least_sb + 1)..most_sb].iter().rev() {
                buffer.push_bits(8, *byte);
            }
        }
        if least_sb == most_sb {
            buffer.push_bits(
                <T as Specifier>::BITS.try_into().unwrap(),
                bytes[least_sb] >> lsb_offset,
            );
        } else {
            buffer.push_bits(
                (0 - lsb_offset).try_into().unwrap(),
                bytes[least_sb] >> lsb_offset,
            );
        }
    }
    buffer.into_bytes()
}

#[doc(hidden)]
#[inline]
pub fn write_specifier<T>(
    bytes: &mut [u8],
    offset: usize,
    new_val: <T as Specifier>::Bytes,
) where
    T: Specifier,
    PopBuffer<T::Bytes>: PopBits,
{
    let end = offset + <T as Specifier>::BITS;
    let least_sb = offset / 8; // compile-time
    let most_sb = (end - 1) / 8; // compile-time
    let lsb_offset = offset % 8; // compile-time
    let msb_offset = end % 8; // compile-time
    let msb_offset = if msb_offset == 0 { 8 } else { msb_offset };

    let mut buffer = <PopBuffer<T::Bytes>>::from_bytes(new_val);

    if lsb_offset == 0 && msb_offset == 8 {
        // Edge-case for whole bytes manipulation.
        for byte in bytes[least_sb..=most_sb].iter_mut() {
            *byte = buffer.pop_bits(8);
        }
    } else {
        // Least-significant byte
        let stays_same = bytes[least_sb]
            & (if least_sb == most_sb && msb_offset != 8 {
                !((0x01 << msb_offset) - 1)
            } else {
                0u8
            } | ((0x01 << lsb_offset) - 1));
        let overwrite = buffer.pop_bits((8 - lsb_offset).try_into().unwrap());
        bytes[least_sb] = stays_same | (overwrite << lsb_offset);
        if most_sb - least_sb >= 2 {
            // Middle bytes
            for byte in bytes[(least_sb + 1)..most_sb].iter_mut() {
                *byte = buffer.pop_bits(8);
            }
        }
        if least_sb != most_sb {
            // Most-significant byte
            if msb_offset == 8 {
                // We don't need to respect what was formerly stored in the byte.
                bytes[most_sb] = buffer.pop_bits(msb_offset.try_into().unwrap());
            } else {
                // All bits that do not belong to this field should be preserved.
                let stays_same = bytes[most_sb] & !((0x01 << msb_offset) - 1);
                let overwrite = buffer.pop_bits(msb_offset.try_into().unwrap());
                bytes[most_sb] = stays_same | overwrite;
            }
        }
    }
}
