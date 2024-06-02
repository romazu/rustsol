use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::utils::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};
use crate::types::{PrimitiveKey, BytesKey, AddressKey};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

// The value corresponding to a mapping key k is located at keccak256(h(k) . p) where . is
// concatenation and h is a function that is applied to the key depending on its type:
// - for value types, h pads the value to 32 bytes in the same way as when storing the value in memory.
// - for strings and byte arrays, h computes the keccak256 hash of the unpadded data.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Mapping<KeyType, Value> {
    __slot: U256,
    __marker: PhantomData<(KeyType, Value)>,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<KeyType, Value> Mapping<KeyType, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }

    fn get_value(&self, key: [u8; 32]) -> Value
        where
            Value: Position + SlotsGetterSetter,
    {
        let value_slot_bytes = keccak256_concat(key, u256_to_bytes32(self.__slot));
        let mut value = Value::from_position(bytes32_to_u256(value_slot_bytes), 0);
        match &self.__slot_getter {
            None => {
                // No slots getter to pass to children.
            }
            Some(getter) => {
                // Set child's slots getter.
                value.set_slots_getter(getter.clone());
            }
        }
        value
    }
}

impl<KeyType, Value> Position for Mapping<KeyType, Value> {
    fn from_position(slot: U256, _: usize) -> Self {
        Mapping::<KeyType, Value> { __slot: slot, __marker: PhantomData, __slot_getter: None }
    }

    fn size() -> usize {
        32
    }
}


impl<Value> Mapping<PrimitiveKey, Value> {
    pub fn get<T>(&self, key: T) -> Value
        where
            T: Into<PrimitiveKey>,
            Value: Position + SlotsGetterSetter,
    {
        self.get_value(key.into().0)
    }
}

impl<Value> Mapping<BytesKey, Value> {
    pub fn get<T>(&self, key: T) -> Value
        where
            T: Into<BytesKey>,
            Value: Position + SlotsGetterSetter,
    {
        self.get_value(key.into().0)
    }
}

impl<Value> Mapping<AddressKey, Value> {
    pub fn get<T>(&self, key: T) -> Value
        where
            T: Into<AddressKey>,
            Value: Position + SlotsGetterSetter,
    {
        self.get_value(key.into().0)
    }
}

impl<KeyType: Debug, ValueType: Debug> SlotsGetterSetter for Mapping<KeyType, ValueType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
