use std::sync::Arc;
use alloy_primitives::U256;
use crate::utils::{bytes32_to_u256, keccak256, u256_to_bytes32};
use crate::types::{Primitive};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

// In particular: if the data is at most 31 bytes long, the elements are stored in the higher-order
// bytes (left aligned) and the lowest-order byte stores the value length * 2. For byte arrays that
// store data which is 32 or more bytes long, the main slot p stores length * 2 + 1 and the data is
// stored as usual in keccak256(p). This means that you can distinguish a short array from a long
// array by checking if the lowest bit is set: short (not set) and long (set).
#[derive(Debug, Default)]
pub struct Bytes {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl Bytes {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
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

impl Position for Bytes {
    fn from_position(slot: U256, _: u8) -> Self {
        Bytes { __slot: slot, __slot_getter: None }
    }

    fn size() -> u64 {
        32
    }
}

impl SlotsGetterSetter for Bytes {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
