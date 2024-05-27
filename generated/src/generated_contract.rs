#![allow(non_snake_case, unused, dead_code, unused_imports)]
use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
pub struct Contract {
    __slot: U256,
    pub owner: Primitive<20>,
    pub pendingOwner: Primitive<20>,
    pub s_billing: OffchainAggregatorBillingBilling,
    pub s_linkToken: Primitive<20>,
    pub s_billingAccessController: Primitive<20>,
    pub s_oracleObservationsCounts: StaticArray<64, Primitive<2>>,
    pub s_payees: Mapping<PrimitiveKey, Primitive<20>>,
    pub s_proposedPayees: Mapping<PrimitiveKey, Primitive<20>>,
    pub s_gasReimbursementsLinkWei: StaticArray<992, Primitive<32>>,
    pub s_oracles: Mapping<PrimitiveKey, OffchainAggregatorBillingOracle>,
    pub s_signers: DynamicArray<Primitive<20>>,
    pub s_transmitters: DynamicArray<Primitive<20>>,
    pub s_hotVars: OffchainAggregatorHotVars,
    pub s_transmissions: Mapping<PrimitiveKey, OffchainAggregatorTransmission>,
    pub s_configCount: Primitive<4>,
    pub s_latestConfigBlockNumber: Primitive<4>,
    pub s_validatorConfig: OffchainAggregatorValidatorConfig,
    pub s_requesterAccessController: Primitive<20>,
    pub s_description: Bytes,
    pub checkEnabled: Primitive<1>,
    pub accessList: Mapping<PrimitiveKey, Primitive<1>>,
}
#[derive(Debug)]
pub struct OffchainAggregatorBillingBilling {
    __slot: U256,
    pub maximumGasPrice: Primitive<4>,
    pub reasonableGasPrice: Primitive<4>,
    pub microLinkPerEth: Primitive<4>,
    pub linkGweiPerObservation: Primitive<4>,
    pub linkGweiPerTransmission: Primitive<4>,
}
#[derive(Debug)]
pub struct OffchainAggregatorBillingOracle {
    __slot: U256,
    pub index: Primitive<1>,
    pub role: Primitive<1>,
}
#[derive(Debug)]
pub struct OffchainAggregatorHotVars {
    __slot: U256,
    pub latestConfigDigest: Primitive<16>,
    pub latestEpochAndRound: Primitive<5>,
    pub threshold: Primitive<1>,
    pub latestAggregatorRoundId: Primitive<4>,
}
#[derive(Debug)]
pub struct OffchainAggregatorTransmission {
    __slot: U256,
    pub answer: Primitive<24>,
    pub timestamp: Primitive<8>,
}
#[derive(Debug)]
pub struct OffchainAggregatorValidatorConfig {
    __slot: U256,
    pub validator: Primitive<20>,
    pub gasLimit: Primitive<4>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            owner: Primitive::from_position(slot + 0, 0),
            pendingOwner: Primitive::from_position(slot + 1, 0),
            s_billing: OffchainAggregatorBillingBilling::from_position(slot + 2, 0),
            s_linkToken: Primitive::from_position(slot + 3, 0),
            s_billingAccessController: Primitive::from_position(slot + 4, 0),
            s_oracleObservationsCounts: StaticArray::from_position(slot + 5, 0),
            s_payees: Mapping::from_position(slot + 7, 0),
            s_proposedPayees: Mapping::from_position(slot + 8, 0),
            s_gasReimbursementsLinkWei: StaticArray::from_position(slot + 9, 0),
            s_oracles: Mapping::from_position(slot + 40, 0),
            s_signers: DynamicArray::from_position(slot + 41, 0),
            s_transmitters: DynamicArray::from_position(slot + 42, 0),
            s_hotVars: OffchainAggregatorHotVars::from_position(slot + 43, 0),
            s_transmissions: Mapping::from_position(slot + 44, 0),
            s_configCount: Primitive::from_position(slot + 45, 0),
            s_latestConfigBlockNumber: Primitive::from_position(slot + 45, 4),
            s_validatorConfig: OffchainAggregatorValidatorConfig::from_position(
                slot + 46,
                0,
            ),
            s_requesterAccessController: Primitive::from_position(slot + 47, 0),
            s_description: Bytes::from_position(slot + 48, 0),
            checkEnabled: Primitive::from_position(slot + 49, 0),
            accessList: Mapping::from_position(slot + 50, 0),
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
impl OffchainAggregatorBillingBilling {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            maximumGasPrice: Primitive::from_position(slot + 0, 0),
            reasonableGasPrice: Primitive::from_position(slot + 0, 4),
            microLinkPerEth: Primitive::from_position(slot + 0, 8),
            linkGweiPerObservation: Primitive::from_position(slot + 0, 12),
            linkGweiPerTransmission: Primitive::from_position(slot + 0, 16),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for OffchainAggregatorBillingBilling {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl OffchainAggregatorBillingOracle {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            index: Primitive::from_position(slot + 0, 0),
            role: Primitive::from_position(slot + 0, 1),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for OffchainAggregatorBillingOracle {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl OffchainAggregatorHotVars {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            latestConfigDigest: Primitive::from_position(slot + 0, 0),
            latestEpochAndRound: Primitive::from_position(slot + 0, 16),
            threshold: Primitive::from_position(slot + 0, 21),
            latestAggregatorRoundId: Primitive::from_position(slot + 0, 22),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for OffchainAggregatorHotVars {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl OffchainAggregatorTransmission {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            answer: Primitive::from_position(slot + 0, 0),
            timestamp: Primitive::from_position(slot + 0, 24),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for OffchainAggregatorTransmission {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl OffchainAggregatorValidatorConfig {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            validator: Primitive::from_position(slot + 0, 0),
            gasLimit: Primitive::from_position(slot + 0, 20),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for OffchainAggregatorValidatorConfig {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
