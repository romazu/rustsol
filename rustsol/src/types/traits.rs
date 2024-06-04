use std::fmt::Debug;
use std::sync::Arc;
use alloy_primitives::U256;

pub trait Position {
    fn from_position(slot: U256, offset: usize) -> Self;
    fn size() -> usize;
}

pub trait SlotsGetter {
    fn get_slots(&self, start: U256, n: usize) -> Result<Vec<U256>, String>;
}

pub trait SlotsGetterSetter: Debug {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>);
}

pub trait Value {
    type ValueType;
    fn get_value_from_slots_content(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String>;
}

pub trait FromLESlice {
    fn from(bytes: &[u8]) -> Self;
}
