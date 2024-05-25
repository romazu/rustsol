use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use crate::keccak::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};

#[derive(Debug, Default)]
pub struct Primitive {
    __slot: U256,
    __offset: U256,
}

impl Primitive {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> U256 {
        self.__offset
    }
}
impl FromPosition for Primitive {
    fn from_position(slot: U256, offset: U256) -> Self {
        Primitive { __slot: slot, __offset: offset }  // Use the conversion from U256 to u64
    }
}

#[derive(Debug, Default)]
pub struct Bytes {
    __slot: U256,
}

impl Bytes {
    fn slot(&self) -> U256 {
        self.__slot
    }
}

impl FromPosition for Bytes {
    fn from_position(slot: U256, offset: U256) -> Self {
        Bytes { __slot: slot }
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

pub trait FromPosition {
    fn from_position(slot: U256, offset: U256) -> Self;
}

#[derive(Debug)]
pub struct Mapping<KeyType, Value> {
    __slot: U256,
    __marker: PhantomData<(KeyType, Value)>,
}

impl<KeyType, Value> Mapping<KeyType, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    fn get_value(&self, key: [u8; 32]) -> Value
    where
        Value: FromPosition,
    {
        let value_slot_bytes = keccak256_concat(key, u256_to_bytes32(self.__slot));
        Value::from_position(bytes32_to_u256(value_slot_bytes), U256::zero())
    }
}

impl<KeyType, Value> FromPosition for Mapping<KeyType, Value> {
    fn from_position(slot: U256, offset: U256) -> Self {
        Mapping::<KeyType, Value> { __slot: slot, __marker: PhantomData }
    }
}


impl<Value> Mapping<PrimitiveKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<PrimitiveKey>,
            Value: FromPosition,
    {
        self.get_value(key.into().0)
    }
}

impl<Value> Mapping<BytesKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<BytesKey>,
            Value: FromPosition,
    {
        self.get_value(key.into().0)
    }
}
