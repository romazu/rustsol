use std::marker::PhantomData;
use primitive_types::U256;
use crate::keccak::{bytes32_to_u256, ceil_div, keccak256, keccak256_concat, u256_to_bytes32};

pub trait Position {
    fn from_position(slot: U256, offset: u8) -> Self;
    fn size() -> u64;
}

#[derive(Debug, Default)]
pub struct Primitive<const SIZE: u64> {
    __slot: U256,
    __offset: u8,
}

impl<const SIZE: u64> Primitive<SIZE> {
    pub fn size(&self) -> u64 {
        SIZE
    }

    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> u8 {
        self.__offset
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, self.__offset, SIZE)
    }
}

impl<const SIZE: u64> Position for Primitive<SIZE> {
    fn from_position(slot: U256, offset: u8) -> Self {
        Primitive { __slot: slot, __offset: offset }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        SIZE
    }
}

// In particular: if the data is at most 31 bytes long, the elements are stored in the higher-order
// bytes (left aligned) and the lowest-order byte stores the value length * 2. For byte arrays that
// store data which is 32 or more bytes long, the main slot p stores length * 2 + 1 and the data is
// stored as usual in keccak256(p). This means that you can distinguish a short array from a long
// array by checking if the lowest bit is set: short (not set) and long (set).
#[derive(Debug, Default)]
pub struct Bytes {
    __slot: U256,
}

impl Bytes {
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
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


// The value corresponding to a mapping key k is located at keccak256(h(k) . p) where . is
// concatenation and h is a function that is applied to the key depending on its type:
// - for value types, h pads the value to 32 bytes in the same way as when storing the value in memory.
// - for strings and byte arrays, h computes the keccak256 hash of the unpadded data.
#[derive(Debug)]
pub struct Mapping<KeyType, Value> {
    __slot: U256,
    __marker: PhantomData<(KeyType, Value)>,
}

impl<KeyType, Value> Mapping<KeyType, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
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

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
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
pub struct StaticArray<const SIZE: u64, Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

impl<const SIZE: u64, Value: Position> StaticArray<SIZE, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, SIZE)
    }

    pub fn capacity(&self) -> usize {
        let value_size = Value::size();
        let (packing_n, packing_d) = packing_ratio(value_size);
        let capacity = SIZE / 32 * packing_d / packing_n;
        capacity as usize
    }
}

impl<const SIZE: u64, Value> Position for StaticArray<SIZE, Value> {
    fn from_position(slot: U256, offset: u8) -> Self {
        StaticArray::<SIZE, Value> { __slot: slot, __marker: PhantomData }
    }

    fn size() -> u64 {
        SIZE
    }
}

impl<const SIZE: u64, Value> StaticArray<SIZE, Value> {
    pub fn get_item(&self, index: usize) -> Value
        where
            Value: Position,
    {
        let value_size = Value::size();
        let (packing_n, packing_d) = packing_ratio(value_size);
        let capacity = SIZE / 32 * packing_d / packing_n;
        if index >= capacity as usize {
            panic!("Index is outside array capacity: {} vs {}", index, capacity)
        }
        let slot = self.__slot + index as u64 * packing_n / packing_d;
        let offset = (index as u64 * 32 * packing_n / packing_d % 32) as u8;  // guaranteed to fit in u8
        Value::from_position(slot, offset)
    }
}
