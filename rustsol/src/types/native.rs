use alloy_primitives::{Address, I256, U256};
use crate::types::FromLESlice;


macro_rules! from_le_slice_impl_signed {
    ($($t:ty),+) => {
        $(
            impl FromLESlice for $t {
                fn from(bytes: &[u8]) -> Self {
                    // Check MSB.
                    let is_negative = bytes[bytes.len() - 1] & (1 << 7) != 0;
                    const TYPE_SIZE: usize = (<$t>::BITS / 8) as usize;
                    let mut bytes_sized = if is_negative {
                        [0xFF; TYPE_SIZE]
                    } else {
                        [0u8; TYPE_SIZE]
                    };
                    bytes_sized[0..bytes.len()].copy_from_slice(bytes);
                    Self::from_le_bytes(bytes_sized)
                }
            }
        )+
    };
}

macro_rules! from_le_slice_impl_unsigned {
    ($($t:ty),+) => {
        $(
            impl FromLESlice for $t {
                fn from(bytes: &[u8]) -> Self {
                    const TYPE_SIZE: usize = (<$t>::BITS / 8) as usize;
                    let mut bytes_sized = [0u8; TYPE_SIZE];
                    bytes_sized[0..bytes.len()].copy_from_slice(bytes);
                    Self::from_le_bytes(bytes_sized)
                }
            }
        )+
    };
}

from_le_slice_impl_signed!(i8, i16, i32, i64, i128, I256);
from_le_slice_impl_unsigned!(u8, u16, u32, u64, u128, U256);
impl FromLESlice for bool {
    fn from(bytes: &[u8]) -> Self {
        bytes[0] == 1
    }
}

impl FromLESlice for Address {
    fn from(bytes: &[u8]) -> Self {
        // Convert little-endian bytes to big-endian.
        let mut big_endian_bytes = bytes.to_vec();
        big_endian_bytes.reverse();
        Address::from_slice(&big_endian_bytes)
    }
}

impl FromLESlice for Vec<u8> {
    fn from(bytes: &[u8]) -> Self {
        bytes.to_vec()
    }
}

impl FromLESlice for String {
    fn from(bytes: &[u8]) -> Self {
        String::from_utf8_lossy(bytes).to_string()
    }
}
