use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use crate::types::{Mapping, Position, SlotsGetter, SlotsGetterSetter};
use crate::utils::{index_to_position, ceil_div};

#[derive(Debug)]
pub struct StaticArray<const SIZE: u64, Value> {
    __slot: U256,
    __marker: PhantomData<Value>,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<const SIZE: u64, Value: Position> StaticArray<SIZE, Value> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, SIZE)
    }

    pub fn storage(&self) -> U256 {
        self.__slot
    }

    // Return the packing ratio: (n, d).
    // This means that packing is "n slot per d elements"
    // In the current solidity implementation one element of the pair is always one.
    pub fn packing_ratio(&self) -> (u64, u64) {
        let element_size = Value::size();
        if element_size > 32 {
            (ceil_div(element_size, 32), 1)
        } else {
            (1, 32 / element_size)
        }
    }

    pub fn capacity(&self) -> usize {
        let (packing_n, packing_d) = self.packing_ratio();
        let capacity = SIZE / 32 * packing_d / packing_n;
        capacity as usize
    }
}

impl<const SIZE: u64, Value> Position for StaticArray<SIZE, Value> {
    fn from_position(slot: U256, _: u8) -> Self {
        StaticArray::<SIZE, Value> { __slot: slot, __marker: PhantomData, __slot_getter: None }
    }

    fn size() -> u64 {
        SIZE
    }
}

impl<const SIZE: u64, Value> StaticArray<SIZE, Value> {
    pub fn get(&self, index: usize) -> Value
        where
            Value: Position,
    {
        let (packing_n, packing_d) = self.packing_ratio();
        let capacity = SIZE / 32 * packing_d / packing_n;
        if index >= capacity as usize {
            panic!("Index is outside array capacity: {} vs {}", index, capacity)
        }
        let (index_slot, index_offset) = index_to_position(index, packing_n, packing_d);
        Value::from_position(self.storage() + index_slot, index_offset)
    }
}

impl<const SIZE: u64, Value: Debug> SlotsGetterSetter for StaticArray<SIZE, Value> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
