use std::marker::PhantomData;
use std::ops::Index;
use primitive_types::U256;
use crate::keccak::{bytes32_to_u256, ceil_div, keccak256, keccak256_concat, u256_to_bytes32};

#[derive(Debug, Default)]
pub struct Primitive<const BYTES: u64> {
    __slot: U256,
    __offset: u8,
}

impl<const BYTES: u64> Primitive<BYTES> {
    pub fn size(&self) -> u64 {
        BYTES
    }

    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> u8 {
        self.__offset
    }
}

impl<const BYTES: u64> Position for Primitive<BYTES> {
    fn from_position(slot: U256, offset: u8) -> Self {
        Primitive { __slot: slot, __offset: offset }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        BYTES
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

impl Position for Bytes {
    fn from_position(slot: U256, offset: u8) -> Self {
        Bytes { __slot: slot }
    }

    fn size() -> u64 {
        32
    }
}

trait FromBytes {
    fn from_bytes(bytes: [u8; 32]) -> Self;
}

#[derive(Debug, Default)]
pub struct PrimitiveKey([u8; 32]);

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

// #[derive(Debug, Default)]
// pub struct IntegerKey([u8; 32]);
//
// impl IntegerKey {
//     fn new(bytes: [u8; 32]) -> Self {
//         IntegerKey(bytes)
//     }
// }
//
// impl_from_for!(IntegerKey, u8, u16, u32, u64, u128);
// impl From<U256> for IntegerKey {
//     fn from(value: U256) -> Self {
//         IntegerKey(u256_to_bytes32(value))
//     }
// }


pub trait Position {
    fn from_position(slot: U256, offset: u8) -> Self;
    fn size() -> u64;
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
            Value: Position,
    {
        let value_slot_bytes = keccak256_concat(key, u256_to_bytes32(self.__slot));
        Value::from_position(bytes32_to_u256(value_slot_bytes), 0)
    }
}

impl<KeyType, Value> Position for Mapping<KeyType, Value> {
    fn from_position(slot: U256, offset: u8) -> Self {
        Mapping::<KeyType, Value> { __slot: slot, __marker: PhantomData }
    }

    fn size() -> u64 {
        32
    }
}


impl<Value> Mapping<PrimitiveKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<PrimitiveKey>,
            Value: Position,
    {
        self.get_value(key.into().0)
    }
}

impl<Value> Mapping<BytesKey, Value> {
    pub fn get_item<T>(&self, key: T) -> Value
        where
            T: Into<BytesKey>,
            Value: Position,
    {
        self.get_value(key.into().0)
    }
}


#[derive(Debug)]
pub struct DynamicArray<Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

impl<Value> DynamicArray<Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}

impl<Value> Position for DynamicArray<Value> {
    fn from_position(slot: U256, offset: u8) -> Self {
        DynamicArray::<Value> { __slot: slot, __marker: PhantomData }
    }

    fn size() -> u64 {
        32
    }
}

impl<Value> DynamicArray<Value> {
    pub fn get_item(&self, index: usize) -> Value
        where
            Value: Position,
    {
        let base_slot_bytes = keccak256(u256_to_bytes32(self.__slot));
        let base_slot = bytes32_to_u256(base_slot_bytes);
        let value_len = ceil_div(Value::size(), 32); // How many slots value occupies.
        let value_slot = base_slot + value_len * index as u64;
        Value::from_position(value_slot, 0)
    }
}

// Return the packing ratio: (n, d).
// This means that packing is "n slot per d elements"
// In the current solidity implementation one element of the pair is always one.
fn packing_ratio(element_size: u64) -> (u64, u64) {
    if element_size > 32 {
        (ceil_div(element_size, 32), 1)
    } else {
        (1, 32 / element_size)
    }
}

#[derive(Debug)]
pub struct StaticArray<const BYTES: u64, Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

impl<const BYTES: u64, Value> StaticArray<BYTES, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}

impl<const BYTES: u64, Value> Position for StaticArray<BYTES, Value> {
    fn from_position(slot: U256, offset: u8) -> Self {
        StaticArray::<BYTES, Value> { __slot: slot, __marker: PhantomData }
    }

    fn size() -> u64 {
        BYTES
    }
}

impl<const BYTES: u64, Value> StaticArray<BYTES, Value> {
    pub fn get_item(&self, index: usize) -> Value
        where
            Value: Position,
    {
        let value_size = Value::size();
        let (packing_n, packing_d) = packing_ratio(value_size);
        let capacity = BYTES / 32 * packing_d / packing_n;
        if index >= capacity as usize {
            panic!("Index is outside array capacity: {} vs {}", index, capacity)
        }
        let slot = self.__slot + index as u64 * packing_n / packing_d;
        let offset = (index as u64 * 32 * packing_n / packing_d % 32) as u8;  // guaranteed to fit in u8
        Value::from_position(slot, offset)
    }
}
