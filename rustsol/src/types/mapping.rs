use std::marker::PhantomData;
use primitive_types::U256;
use crate::utils::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};
use crate::types::keys::{BytesKey, PrimitiveKey};
use crate::types::Position;

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
    fn from_position(slot: U256, _: u8) -> Self {
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
