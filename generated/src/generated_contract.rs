#![allow(non_snake_case, unused, dead_code, unused_imports)]
use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
pub struct Contract {
    __slot: U256,
    pub totalSupply: Primitive<32>,
    pub balanceOf: Mapping<PrimitiveKey, Primitive<32>>,
    pub allowance: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive<32>>>,
    pub DOMAIN_SEPARATOR: Primitive<32>,
    pub nonces: Mapping<PrimitiveKey, Primitive<32>>,
    pub factory: Primitive<20>,
    pub token0: Primitive<20>,
    pub token1: Primitive<20>,
    pub reserve0: Primitive<14>,
    pub reserve1: Primitive<14>,
    pub blockTimestampLast: Primitive<4>,
    pub price0CumulativeLast: Primitive<32>,
    pub price1CumulativeLast: Primitive<32>,
    pub kLast: Primitive<32>,
    pub unlocked: Primitive<32>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            totalSupply: Primitive::from_position(slot + 0, 0),
            balanceOf: Mapping::from_position(slot + 1, 0),
            allowance: Mapping::from_position(slot + 2, 0),
            DOMAIN_SEPARATOR: Primitive::from_position(slot + 3, 0),
            nonces: Mapping::from_position(slot + 4, 0),
            factory: Primitive::from_position(slot + 5, 0),
            token0: Primitive::from_position(slot + 6, 0),
            token1: Primitive::from_position(slot + 7, 0),
            reserve0: Primitive::from_position(slot + 8, 0),
            reserve1: Primitive::from_position(slot + 8, 14),
            blockTimestampLast: Primitive::from_position(slot + 8, 28),
            price0CumulativeLast: Primitive::from_position(slot + 9, 0),
            price1CumulativeLast: Primitive::from_position(slot + 10, 0),
            kLast: Primitive::from_position(slot + 11, 0),
            unlocked: Primitive::from_position(slot + 12, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 0)
    }
}
impl Position for Contract {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        0
    }
}
