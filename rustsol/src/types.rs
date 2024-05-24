use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};
use crate::layout::Member;

#[derive(Debug, Default)]
pub struct Primitive {
    __slot: [u8; 32],
    __offset: U256,
    // __info: Member
}

impl Primitive {
    pub fn slot(&self) -> U256 {
        bytes32_to_u256(self.__slot)
    }

    pub fn offset(&self) -> U256 {
        self.__offset
    }
}

impl From<[u8; 32]> for Primitive {
    fn from(u: [u8; 32]) -> Self {
        Primitive { __slot: u, __offset: U256::zero() }  // Use the conversion from U256 to u64
    }
}

#[derive(Debug, Default)]
pub struct Bytes {
    __slot: [u8; 32],
}

impl Bytes {
    fn slot(&self) -> U256 {
        bytes32_to_u256(self.__slot)
    }
}

impl From<[u8; 32]> for Bytes {
    fn from(u: [u8; 32]) -> Self {
        Bytes { __slot: u }
    }
}

#[derive(Debug, Default)]
pub struct PrimitiveKey([u8; 32]);
impl From<u64> for PrimitiveKey {
    fn from(value: u64) -> Self {
        let mut key_bytes = [0u8; 32];
        key_bytes[24..].copy_from_slice(&value.to_be_bytes());
        PrimitiveKey(key_bytes)
    }
}

#[derive(Debug, Default)]
pub struct BytesKey([u8; 32]);
pub trait ToBytesKey {
    fn into_bytes_key(self) -> BytesKey;
}

impl From<&str> for BytesKey {
    fn from(value: &str) -> Self {
        let mut value = [0u8; 32];
        BytesKey(value)
    }
}


#[derive(Debug)]
pub struct Mapping<KeyType, Value> {
    __slot: [u8; 32],
    __marker: PhantomData<(KeyType, Value)>,
}

// impl<KeyType, Value> fmt::Debug for Mapping<KeyType, Value> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Mapping")
//             .field("__slot", &bytes32_to_u256(self.__slot))
//             .finish()
//     }
// }

impl<KeyType, Value> Default for Mapping<KeyType, Value>
    where
        KeyType: Default,
        Value: Default,
{
    fn default() -> Self {
        Mapping {
            __slot: [0u8; 32],
            __marker: PhantomData,
        }
    }
}

impl<KeyType, Value> From<[u8; 32]> for Mapping<KeyType, Value> {
    fn from(slot: [u8; 32]) -> Self {
        Self {
            __slot: slot,
            __marker: PhantomData,
        }
    }
}

impl<KeyType, Value> Mapping<KeyType, Value> {
    pub fn slot(&self) -> U256 {
        bytes32_to_u256(self.__slot)
    }

    fn get_value(&self, key: [u8; 32]) -> Value
    where
        Value: From<[u8; 32]>,
    {
        let value_slot_bytes = keccak256_concat(key, self.__slot);
        Value::from(value_slot_bytes)
    }
}

impl<Value> Mapping<PrimitiveKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<PrimitiveKey>,
            Value: From<[u8; 32]>,
    {
        self.get_value(key.into().0)
    }
}

impl<Value> Mapping<BytesKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<BytesKey>,
            Value: From<[u8; 32]>,
    {
        self.get_value(key.into().0)
    }
}


#[derive(Debug)]
struct CommonKey([u8; 32]);

// Implement methods for CommonKey
impl CommonKey {
    fn to_key_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl From<u64> for CommonKey {
    fn from(key: u64) -> Self {
        // TODO: Fix conversion.
        let mut bytes = [0u8; 32];
        bytes[0..std::mem::size_of::<u64>()].copy_from_slice(&key.to_be_bytes());
        CommonKey(bytes)
    }
}
