use ethereum_types::{Address, U256};
use crate::utils::{address_to_bytes32, keccak256, u256_to_bytes32};

#[derive(Debug, Default)]
pub struct PrimitiveKey(pub [u8; 32]);

impl PrimitiveKey {
    fn new(bytes: [u8; 32]) -> Self {
        PrimitiveKey(bytes)
    }
}

macro_rules! impl_from_for {
    ($target:ty, $($type:ty),+) => {
        $(
            impl From<$type> for $target {
                fn from(value: $type) -> Self {
                    #[allow(unused_comparisons)]
                    let mut bytes = if value < 0 {
                        [0xFF; 32]
                    } else {
                        [0u8; 32]
                    };

                    let be_bytes = value.to_be_bytes();
                    let start = 32 - be_bytes.len();
                    bytes[start..].copy_from_slice(&be_bytes);

                    <$target>::new(bytes)
                }
            }
        )+
    };
}

impl_from_for!(PrimitiveKey, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl From<U256> for PrimitiveKey {
    fn from(value: U256) -> Self {
        PrimitiveKey(u256_to_bytes32(value))
    }
}
impl From<Address> for PrimitiveKey {
    fn from(value: Address) -> Self {
        PrimitiveKey(address_to_bytes32(value))
    }
}

#[derive(Debug, Default)]
pub struct BytesKey(pub [u8; 32]);

impl From<&str> for BytesKey {
    fn from(value: &str) -> Self {
        BytesKey(keccak256(value.as_bytes()))
    }
}

impl From<String> for BytesKey {
    fn from(value: String) -> Self {
        BytesKey::from(value.as_str())
    }
}

impl From<&String> for BytesKey {
    fn from(value: &String) -> Self {
        BytesKey::from(value.as_str())
    }
}

impl From<&[u8]> for BytesKey {
    fn from(value: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&keccak256(value)[..32]);
        BytesKey(bytes)
    }
}

impl From<Vec<u8>> for BytesKey {
    fn from(value: Vec<u8>) -> Self {
        BytesKey::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for BytesKey {
    fn from(value: &Vec<u8>) -> Self {
        BytesKey::from(value.as_slice())
    }
}
