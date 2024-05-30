use ethereum_types::U256;
use crate::types::traits::Position;

#[derive(Debug, Default)]
pub struct Primitive<const SIZE: u64> {
    __slot: U256,
    __offset: u8,
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
}

impl<const SIZE: u64> Position for Primitive<SIZE> {
    fn from_position(slot: U256, offset: u8) -> Self {
        Primitive { __slot: slot, __offset: offset }  // Use the conversion from U256 to u64
    }

    fn size() -> u64 {
        SIZE
    }
}
