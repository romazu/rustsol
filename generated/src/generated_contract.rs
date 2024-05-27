#![allow(non_snake_case, unused, dead_code, unused_imports)]
use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
pub struct Contract {
    __slot: U256,
    pub markets: Mapping<PrimitiveKey, Primitive<1>>,
    pub universes: Mapping<PrimitiveKey, Primitive<1>>,
    pub crowdsourcers: Mapping<PrimitiveKey, Primitive<1>>,
    pub trustedSender: Mapping<PrimitiveKey, Primitive<1>>,
    pub marketCreationData: Mapping<
        PrimitiveKey,
        IAugurCreationDataGetterMarketCreationData,
    >,
    pub uploader: Primitive<20>,
    pub registry: Mapping<PrimitiveKey, Primitive<20>>,
    pub time: Primitive<20>,
    pub genesisUniverse: Primitive<20>,
    pub forkCounter: Primitive<32>,
    pub universeForkIndex: Mapping<PrimitiveKey, Primitive<32>>,
    pub upgradeTimestamp: Primitive<32>,
    pub cash: Primitive<20>,
}
#[derive(Debug)]
pub struct IAugurCreationDataGetterMarketCreationData {
    __slot: U256,
    pub extraInfo: Bytes,
    pub marketCreator: Primitive<20>,
    pub outcomes: DynamicArray<Primitive<32>>,
    pub displayPrices: DynamicArray<Primitive<32>>,
    pub marketType: Primitive<1>,
    pub recommendedTradeInterval: Primitive<32>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            markets: Mapping::from_position(slot + 0, 0),
            universes: Mapping::from_position(slot + 1, 0),
            crowdsourcers: Mapping::from_position(slot + 2, 0),
            trustedSender: Mapping::from_position(slot + 3, 0),
            marketCreationData: Mapping::from_position(slot + 4, 0),
            uploader: Primitive::from_position(slot + 5, 0),
            registry: Mapping::from_position(slot + 6, 0),
            time: Primitive::from_position(slot + 7, 0),
            genesisUniverse: Primitive::from_position(slot + 8, 0),
            forkCounter: Primitive::from_position(slot + 9, 0),
            universeForkIndex: Mapping::from_position(slot + 10, 0),
            upgradeTimestamp: Primitive::from_position(slot + 11, 0),
            cash: Primitive::from_position(slot + 12, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
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
impl IAugurCreationDataGetterMarketCreationData {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            extraInfo: Bytes::from_position(slot + 0, 0),
            marketCreator: Primitive::from_position(slot + 1, 0),
            outcomes: DynamicArray::from_position(slot + 2, 0),
            displayPrices: DynamicArray::from_position(slot + 3, 0),
            marketType: Primitive::from_position(slot + 4, 0),
            recommendedTradeInterval: Primitive::from_position(slot + 5, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 192)
    }
}
impl Position for IAugurCreationDataGetterMarketCreationData {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        192
    }
}
