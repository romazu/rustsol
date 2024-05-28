use std::marker::PhantomData;
use primitive_types::U256;
use crate::utils::{bytes32_to_u256, ceil_div, keccak256, u256_to_bytes32};
use crate::types::Position;

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
