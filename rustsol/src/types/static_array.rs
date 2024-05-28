use std::marker::PhantomData;
use primitive_types::U256;
use crate::utils;
use crate::types::Position;

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
        let (packing_n, packing_d) = utils::packing_ratio(value_size);
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
        let (packing_n, packing_d) = utils::packing_ratio(value_size);
        let capacity = SIZE / 32 * packing_d / packing_n;
        if index >= capacity as usize {
            panic!("Index is outside array capacity: {} vs {}", index, capacity)
        }
        let slot = self.__slot + index as u64 * packing_n / packing_d;
        let offset = (index as u64 * 32 * packing_n / packing_d % 32) as u8;  // guaranteed to fit in u8
        Value::from_position(slot, offset)
    }
}
