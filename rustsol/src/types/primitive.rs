use std::fmt::Debug;
use std::sync::Arc;
use alloy_primitives::U256;
use crate::types::{Mapping, SlotsGetterSetter};
use crate::types::traits::{Position, SlotsGetter};

#[derive(Debug, Default)]
pub struct Primitive<const SIZE: u64> {
    __slot: U256,
    __offset: u8,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<const SIZE: u64> Primitive<SIZE> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> u8 {
        self.__offset
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, self.__offset, SIZE)
    }

    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0] // debug dummy
            },
        }
    }
}

impl<const SIZE: u64> Position for Primitive<SIZE> {
    fn from_position(slot: U256, offset: u8) -> Self {
        Primitive { __slot: slot, __offset: offset, __slot_getter: None }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        SIZE
    }
}

impl<const SIZE: u64> SlotsGetterSetter for Primitive<SIZE> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
