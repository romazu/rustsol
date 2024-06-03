#![allow(unused_imports, non_snake_case, unused, dead_code)]
use std::sync::Arc;
use rustsol::types::Derivative;
use rustsol::types::{Position, SlotsGetter, SlotsGetterSetter, Value};
use rustsol::types::{Primitive, Bytes, Address, Mapping, DynamicArray, StaticArray};
use rustsol::types::{PrimitiveKey, BytesKey, AddressKey};
use alloy_primitives;
use alloy_primitives::U256;
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Augur {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub markets: Mapping<AddressKey, Primitive<1>>,
    pub universes: Mapping<AddressKey, Primitive<1>>,
    pub crowdsourcers: Mapping<AddressKey, Primitive<1>>,
    pub trustedSender: Mapping<AddressKey, Primitive<1>>,
    pub marketCreationData: Mapping<
        AddressKey,
        IAugurCreationDataGetterMarketCreationData,
    >,
    pub uploader: Address,
    pub registry: Mapping<PrimitiveKey, Address>,
    pub time: Address,
    pub genesisUniverse: Address,
    pub forkCounter: Primitive<32>,
    pub universeForkIndex: Mapping<AddressKey, Primitive<32>>,
    pub upgradeTimestamp: Primitive<32>,
    pub cash: Address,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct IAugurCreationDataGetterMarketCreationData {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub extraInfo: Bytes,
    pub marketCreator: Address,
    pub outcomes: DynamicArray<Primitive<32>>,
    pub displayPrices: DynamicArray<Primitive<32>>,
    pub marketType: Primitive<1>,
    pub recommendedTradeInterval: Primitive<32>,
}
impl Augur {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            markets: Mapping::from_position(slot + U256::from(0), 0),
            universes: Mapping::from_position(slot + U256::from(1), 0),
            crowdsourcers: Mapping::from_position(slot + U256::from(2), 0),
            trustedSender: Mapping::from_position(slot + U256::from(3), 0),
            marketCreationData: Mapping::from_position(slot + U256::from(4), 0),
            uploader: Address::from_position(slot + U256::from(5), 0),
            registry: Mapping::from_position(slot + U256::from(6), 0),
            time: Address::from_position(slot + U256::from(7), 0),
            genesisUniverse: Address::from_position(slot + U256::from(8), 0),
            forkCounter: Primitive::from_position(slot + U256::from(9), 0),
            universeForkIndex: Mapping::from_position(slot + U256::from(10), 0),
            upgradeTimestamp: Primitive::from_position(slot + U256::from(11), 0),
            cash: Address::from_position(slot + U256::from(12), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 416)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 13)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for Augur {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        416
    }
}
impl SlotsGetterSetter for Augur {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.markets.set_slots_getter(getter.clone());
        self.universes.set_slots_getter(getter.clone());
        self.crowdsourcers.set_slots_getter(getter.clone());
        self.trustedSender.set_slots_getter(getter.clone());
        self.marketCreationData.set_slots_getter(getter.clone());
        self.uploader.set_slots_getter(getter.clone());
        self.registry.set_slots_getter(getter.clone());
        self.time.set_slots_getter(getter.clone());
        self.genesisUniverse.set_slots_getter(getter.clone());
        self.forkCounter.set_slots_getter(getter.clone());
        self.universeForkIndex.set_slots_getter(getter.clone());
        self.upgradeTimestamp.set_slots_getter(getter.clone());
        self.cash.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct AugurValue {
    pub markets: Mapping<AddressKey, Primitive<1>>,
    pub universes: Mapping<AddressKey, Primitive<1>>,
    pub crowdsourcers: Mapping<AddressKey, Primitive<1>>,
    pub trustedSender: Mapping<AddressKey, Primitive<1>>,
    pub marketCreationData: Mapping<
        AddressKey,
        IAugurCreationDataGetterMarketCreationData,
    >,
    pub uploader: alloy_primitives::Address,
    pub registry: Mapping<PrimitiveKey, Address>,
    pub time: alloy_primitives::Address,
    pub genesisUniverse: alloy_primitives::Address,
    pub forkCounter: U256,
    pub universeForkIndex: Mapping<AddressKey, Primitive<32>>,
    pub upgradeTimestamp: U256,
    pub cash: alloy_primitives::Address,
}
impl Value for Augur {
    type ValueType = AugurValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(AugurValue {
            markets: self
                .markets
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            universes: self
                .universes
                .get_value_from_slots_content(slot_values[1..2].to_vec())?,
            crowdsourcers: self
                .crowdsourcers
                .get_value_from_slots_content(slot_values[2..3].to_vec())?,
            trustedSender: self
                .trustedSender
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            marketCreationData: self
                .marketCreationData
                .get_value_from_slots_content(slot_values[4..5].to_vec())?,
            uploader: self
                .uploader
                .get_value_from_slots_content(slot_values[5..6].to_vec())?,
            registry: self
                .registry
                .get_value_from_slots_content(slot_values[6..7].to_vec())?,
            time: self.time.get_value_from_slots_content(slot_values[7..8].to_vec())?,
            genesisUniverse: self
                .genesisUniverse
                .get_value_from_slots_content(slot_values[8..9].to_vec())?,
            forkCounter: self
                .forkCounter
                .get_value_from_slots_content(slot_values[9..10].to_vec())?,
            universeForkIndex: self
                .universeForkIndex
                .get_value_from_slots_content(slot_values[10..11].to_vec())?,
            upgradeTimestamp: self
                .upgradeTimestamp
                .get_value_from_slots_content(slot_values[11..12].to_vec())?,
            cash: self.cash.get_value_from_slots_content(slot_values[12..13].to_vec())?,
        })
    }
}
impl IAugurCreationDataGetterMarketCreationData {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            extraInfo: Bytes::from_position(slot + U256::from(0), 0),
            marketCreator: Address::from_position(slot + U256::from(1), 0),
            outcomes: DynamicArray::from_position(slot + U256::from(2), 0),
            displayPrices: DynamicArray::from_position(slot + U256::from(3), 0),
            marketType: Primitive::from_position(slot + U256::from(4), 0),
            recommendedTradeInterval: Primitive::from_position(slot + U256::from(5), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 192)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 6)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for IAugurCreationDataGetterMarketCreationData {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        192
    }
}
impl SlotsGetterSetter for IAugurCreationDataGetterMarketCreationData {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.extraInfo.set_slots_getter(getter.clone());
        self.marketCreator.set_slots_getter(getter.clone());
        self.outcomes.set_slots_getter(getter.clone());
        self.displayPrices.set_slots_getter(getter.clone());
        self.marketType.set_slots_getter(getter.clone());
        self.recommendedTradeInterval.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct IAugurCreationDataGetterMarketCreationDataValue {
    pub extraInfo: Vec<u8>,
    pub marketCreator: alloy_primitives::Address,
    pub outcomes: Vec<U256>,
    pub displayPrices: Vec<U256>,
    pub marketType: U256,
    pub recommendedTradeInterval: U256,
}
impl Value for IAugurCreationDataGetterMarketCreationData {
    type ValueType = IAugurCreationDataGetterMarketCreationDataValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(IAugurCreationDataGetterMarketCreationDataValue {
            extraInfo: self
                .extraInfo
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            marketCreator: self
                .marketCreator
                .get_value_from_slots_content(slot_values[1..2].to_vec())?,
            outcomes: self
                .outcomes
                .get_value_from_slots_content(slot_values[2..3].to_vec())?,
            displayPrices: self
                .displayPrices
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            marketType: self
                .marketType
                .get_value_from_slots_content(slot_values[4..5].to_vec())?,
            recommendedTradeInterval: self
                .recommendedTradeInterval
                .get_value_from_slots_content(slot_values[5..6].to_vec())?,
        })
    }
}
