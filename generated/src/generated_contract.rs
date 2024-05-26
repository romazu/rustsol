use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
#[allow(non_snake_case)]
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
            totalSupply: Primitive::from_position(slot + 0u64, 0u8),
            balanceOf: Mapping::from_position(slot + 1u64, 0u8),
            allowance: Mapping::from_position(slot + 2u64, 0u8),
            DOMAIN_SEPARATOR: Primitive::from_position(slot + 3u64, 0u8),
            nonces: Mapping::from_position(slot + 4u64, 0u8),
            factory: Primitive::from_position(slot + 5u64, 0u8),
            token0: Primitive::from_position(slot + 6u64, 0u8),
            token1: Primitive::from_position(slot + 7u64, 0u8),
            reserve0: Primitive::from_position(slot + 8u64, 0u8),
            reserve1: Primitive::from_position(slot + 8u64, 14u8),
            blockTimestampLast: Primitive::from_position(slot + 8u64, 28u8),
            price0CumulativeLast: Primitive::from_position(slot + 9u64, 0u8),
            price1CumulativeLast: Primitive::from_position(slot + 10u64, 0u8),
            kLast: Primitive::from_position(slot + 11u64, 0u8),
            unlocked: Primitive::from_position(slot + 12u64, 0u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        0u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 0u64)
    }
}
impl Position for Contract {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
