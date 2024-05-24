use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use serde_json::Value;
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};

#[derive(Debug)]
pub struct Primitive {
    __slot: U256,
    __offset: U256,
}

impl Primitive {
    fn slot(&self) -> U256 {
        self.__slot
    }

    fn offset(&self) -> U256 {
        self.__offset
    }
}

impl From<U256> for Primitive {
    fn from(u: U256) -> Self {
        Primitive { __slot: u, __offset: U256::zero() }  // Use the conversion from U256 to u64
    }
}


pub struct Mapping<Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

impl<Value> From<U256> for Mapping<Value> {
    fn from(slot: U256) -> Self {
        Self {
            __slot: slot,
            __marker: PhantomData,
        }
    }
}

impl<Value> Mapping<Value> {
    fn slot(&self) -> U256 {
        self.__slot
    }
}

impl<Key, Value> Index<Key> for Mapping<Value>
    where
        Key: Into<CommonKey>,
        Value: From<U256>,
{
    type Output = Value;

    fn index(&self, key: Key) -> &Self::Output {
        // Compute the value on the fly and return a reference to it
        let value = self.get_item(key);
        Box::leak(Box::new(value))
    }
}


impl<Value> Mapping<Value> {
    fn get_item<Key>(&self, key: Key) -> Value
        where
            Key: Into<CommonKey>,
            Value: From<U256>,
    {
        // Convert the key into a common representation [u8; 32]
        let key_bytes = key.into().to_key_bytes();
        let value_slot_bytes = keccak256_concat(key_bytes, u256_to_bytes32(self.__slot));
        let value_slot = bytes32_to_u256(value_slot_bytes);
        Value::from(value_slot)
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
