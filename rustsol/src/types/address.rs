use alloy_primitives::U256;
use crate::types::traits::Position;

#[derive(Debug, Default)]
pub struct Address {
    __slot: U256,
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
        Address { __slot: slot }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        20
    }
}
