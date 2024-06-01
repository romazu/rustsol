use std::sync::Arc;
use alloy_primitives::U256;
use crate::types::{Bytes};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

#[derive(Debug, Default)]
pub struct Address {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl Address {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 20)
    }
}

impl Position for Address {
    fn from_position(slot: U256, _: u8) -> Self {
        Address { __slot: slot, __slot_getter: None }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        20
    }
}

impl SlotsGetterSetter for Address {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
