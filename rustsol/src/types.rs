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

macro_rules! impl_from_for_primitive_key {
    ($($type:ty),+) => {
        $(
            impl From<$type> for PrimitiveKey {
                fn from(value: $type) -> Self {
                    let mut bytes = [0u8; 32];
                    let be_bytes = value.to_be_bytes();
                    let start = 32 - be_bytes.len();
                    bytes[start..].copy_from_slice(&be_bytes);
                    PrimitiveKey(bytes)
                }
            }
        )+
    };
}

impl_from_for_primitive_key!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl From<U256> for PrimitiveKey {
    fn from(value: U256) -> Self {
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        PrimitiveKey(bytes)
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
