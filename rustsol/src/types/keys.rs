use alloy_primitives::{Address, I256, U256};
use crate::utils::{address_to_bytes32, bool_to_bytes32, keccak256, u256_to_bytes32};

macro_rules! impl_key_for {
    ($($type:ty),+) => {
        $(
            impl Key for $type {
                fn to_bytes(self) -> [u8; 32] {
                    #[allow(unused_comparisons)]
                    let mut bytes = if self < 0 {
                        [0xFF; 32]
                    } else {
                        [0u8; 32]
                    };

                    let be_bytes = self.to_be_bytes();
                    let start = 32 - be_bytes.len();
                    bytes[start..].copy_from_slice(&be_bytes);
                    bytes
                }
            }
        )+
    };
}

impl_key_for!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl Key for U256 {
    fn to_bytes(self) -> [u8; 32] {
        self.to_be_bytes()
    }
}

impl Key for I256 {
    fn to_bytes(self) -> [u8; 32] {
        self.to_be_bytes()
    }
}

impl Key for bool {
    fn to_bytes(self) -> [u8; 32] {
        bool_to_bytes32(self)
    }
}

impl Key for Vec<u8> {
    fn to_bytes(self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&keccak256(self.as_slice()));
        bytes
    }
}

impl Key for String {
    fn to_bytes(self) -> [u8; 32] {
        keccak256(self.as_bytes())
    }
}

impl Key for Address {
    fn to_bytes(self) -> [u8; 32] {
        address_to_bytes32(self)
    }
}

pub trait Key {
    fn to_bytes(self) -> [u8; 32];
}
