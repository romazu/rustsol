use std::fmt::Debug;
use std::sync::Arc;
use alloy_primitives::U256;

pub trait Position {
    fn from_position(slot: U256, offset: u8) -> Self;
    fn size() -> u64;
}

pub trait SlotsGetter: Debug {
    fn get_slots(&self, start: U256, n: usize) -> Result<Vec<U256>, String>;
}

pub trait SlotsGetterSetter: Debug {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>);
}

pub trait Value {
    type ValueType;
    fn value_from_base_bytes(&self, bytes: &[u8]) -> Result<Self::ValueType, String>;
}
