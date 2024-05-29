use std::marker::PhantomData;
use primitive_types::U256;
use crate::types::Position;
use crate::utils::{bytes32_to_u256, ceil_div, index_to_position, keccak256, u256_to_bytes32};

#[derive(Debug)]
pub struct DynamicArray<Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
}

impl<Value: Position> DynamicArray<Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
    }

    // Return the packing ratio: (n, d).
    // This means that packing is "n slot per d elements"
    // For dynamic array d is always one, i.e., values are not tightly packed.
    pub fn packing_ratio(&self) -> (u64, u64) {
        (ceil_div(Value::size(), 32), 1)
    }
}

impl<Value> Position for DynamicArray<Value> {
    fn from_position(slot: U256, _: u8) -> Self {
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
        let (packing_n, packing_d) = self.packing_ratio(); // Currently in solidity always ratio_d == 1.
        let (index_slot, index_offset) = index_to_position(index, packing_n, packing_d);
        Value::from_position(self.storage() + index_slot, index_offset)
    }
}
