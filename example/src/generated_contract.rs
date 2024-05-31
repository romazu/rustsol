#![allow(unused_imports, non_snake_case, unused, dead_code)]
use std::sync::Arc;
use rustsol::types::{Position, SlotsGetter, SlotsGetterSetter};
use rustsol::types::{Primitive, Bytes, Address, Mapping, DynamicArray, StaticArray};
use rustsol::types::{PrimitiveKey, BytesKey, AddressKey};
use alloy_primitives::U256;
#[derive(Debug)]
pub struct UniswapV3Pool {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub slot0: UniswapV3PoolSlot0,
    pub feeGrowthGlobal0X128: Primitive<32>,
    pub feeGrowthGlobal1X128: Primitive<32>,
    pub protocolFees: UniswapV3PoolProtocolFees,
    pub liquidity: Primitive<16>,
    pub ticks: Mapping<PrimitiveKey, TickInfo>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive<32>>,
    pub positions: Mapping<PrimitiveKey, PositionInfo>,
    pub observations: StaticArray<2097120, OracleObservation>,
}
#[derive(Debug)]
pub struct UniswapV3PoolSlot0 {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub sqrtPriceX96: Primitive<20>,
    pub tick: Primitive<3>,
    pub observationIndex: Primitive<2>,
    pub observationCardinality: Primitive<2>,
    pub observationCardinalityNext: Primitive<2>,
    pub feeProtocol: Primitive<1>,
    pub unlocked: Primitive<1>,
}
#[derive(Debug)]
pub struct UniswapV3PoolProtocolFees {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub token0: Primitive<16>,
    pub token1: Primitive<16>,
}
#[derive(Debug)]
pub struct TickInfo {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub liquidityGross: Primitive<16>,
    pub liquidityNet: Primitive<16>,
    pub feeGrowthOutside0X128: Primitive<32>,
    pub feeGrowthOutside1X128: Primitive<32>,
    pub tickCumulativeOutside: Primitive<7>,
    pub secondsPerLiquidityOutsideX128: Primitive<20>,
    pub secondsOutside: Primitive<4>,
    pub initialized: Primitive<1>,
}
#[derive(Debug)]
pub struct PositionInfo {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub liquidity: Primitive<16>,
    pub feeGrowthInside0LastX128: Primitive<32>,
    pub feeGrowthInside1LastX128: Primitive<32>,
    pub tokensOwed0: Primitive<16>,
    pub tokensOwed1: Primitive<16>,
}
#[derive(Debug)]
pub struct OracleObservation {
    __slot: U256,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
    pub blockTimestamp: Primitive<4>,
    pub tickCumulative: Primitive<7>,
    pub secondsPerLiquidityCumulativeX128: Primitive<20>,
    pub initialized: Primitive<1>,
}
impl UniswapV3Pool {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            slot0: UniswapV3PoolSlot0::from_position(slot + U256::from(0), 0),
            feeGrowthGlobal0X128: Primitive::from_position(slot + U256::from(1), 0),
            feeGrowthGlobal1X128: Primitive::from_position(slot + U256::from(2), 0),
            protocolFees: UniswapV3PoolProtocolFees::from_position(
                slot + U256::from(3),
                0,
            ),
            liquidity: Primitive::from_position(slot + U256::from(4), 0),
            ticks: Mapping::from_position(slot + U256::from(5), 0),
            tickBitmap: Mapping::from_position(slot + U256::from(6), 0),
            positions: Mapping::from_position(slot + U256::from(7), 0),
            observations: StaticArray::from_position(slot + U256::from(8), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 0)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.slot0.set_slots_getter(getter.clone());
        self.feeGrowthGlobal0X128.set_slots_getter(getter.clone());
        self.feeGrowthGlobal1X128.set_slots_getter(getter.clone());
        self.protocolFees.set_slots_getter(getter.clone());
        self.liquidity.set_slots_getter(getter.clone());
        self.ticks.set_slots_getter(getter.clone());
        self.tickBitmap.set_slots_getter(getter.clone());
        self.positions.set_slots_getter(getter.clone());
        self.observations.set_slots_getter(getter.clone())
    }
}
impl Position for UniswapV3Pool {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        0
    }
}
impl SlotsGetterSetter for UniswapV3Pool {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
impl UniswapV3PoolSlot0 {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            sqrtPriceX96: Primitive::from_position(slot + U256::from(0), 0),
            tick: Primitive::from_position(slot + U256::from(0), 20),
            observationIndex: Primitive::from_position(slot + U256::from(0), 23),
            observationCardinality: Primitive::from_position(slot + U256::from(0), 25),
            observationCardinalityNext: Primitive::from_position(
                slot + U256::from(0),
                27,
            ),
            feeProtocol: Primitive::from_position(slot + U256::from(0), 29),
            unlocked: Primitive::from_position(slot + U256::from(0), 30),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.sqrtPriceX96.set_slots_getter(getter.clone());
        self.tick.set_slots_getter(getter.clone());
        self.observationIndex.set_slots_getter(getter.clone());
        self.observationCardinality.set_slots_getter(getter.clone());
        self.observationCardinalityNext.set_slots_getter(getter.clone());
        self.feeProtocol.set_slots_getter(getter.clone());
        self.unlocked.set_slots_getter(getter.clone())
    }
}
impl Position for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl SlotsGetterSetter for UniswapV3PoolSlot0 {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
impl UniswapV3PoolProtocolFees {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            token0: Primitive::from_position(slot + U256::from(0), 0),
            token1: Primitive::from_position(slot + U256::from(0), 16),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.token0.set_slots_getter(getter.clone());
        self.token1.set_slots_getter(getter.clone())
    }
}
impl Position for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl SlotsGetterSetter for UniswapV3PoolProtocolFees {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
impl TickInfo {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            liquidityGross: Primitive::from_position(slot + U256::from(0), 0),
            liquidityNet: Primitive::from_position(slot + U256::from(0), 16),
            feeGrowthOutside0X128: Primitive::from_position(slot + U256::from(1), 0),
            feeGrowthOutside1X128: Primitive::from_position(slot + U256::from(2), 0),
            tickCumulativeOutside: Primitive::from_position(slot + U256::from(3), 0),
            secondsPerLiquidityOutsideX128: Primitive::from_position(
                slot + U256::from(3),
                7,
            ),
            secondsOutside: Primitive::from_position(slot + U256::from(3), 27),
            initialized: Primitive::from_position(slot + U256::from(3), 31),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 128)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.liquidityGross.set_slots_getter(getter.clone());
        self.liquidityNet.set_slots_getter(getter.clone());
        self.feeGrowthOutside0X128.set_slots_getter(getter.clone());
        self.feeGrowthOutside1X128.set_slots_getter(getter.clone());
        self.tickCumulativeOutside.set_slots_getter(getter.clone());
        self.secondsPerLiquidityOutsideX128.set_slots_getter(getter.clone());
        self.secondsOutside.set_slots_getter(getter.clone());
        self.initialized.set_slots_getter(getter.clone())
    }
}
impl Position for TickInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        128
    }
}
impl SlotsGetterSetter for TickInfo {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
impl PositionInfo {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            liquidity: Primitive::from_position(slot + U256::from(0), 0),
            feeGrowthInside0LastX128: Primitive::from_position(slot + U256::from(1), 0),
            feeGrowthInside1LastX128: Primitive::from_position(slot + U256::from(2), 0),
            tokensOwed0: Primitive::from_position(slot + U256::from(3), 0),
            tokensOwed1: Primitive::from_position(slot + U256::from(3), 16),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 128)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.liquidity.set_slots_getter(getter.clone());
        self.feeGrowthInside0LastX128.set_slots_getter(getter.clone());
        self.feeGrowthInside1LastX128.set_slots_getter(getter.clone());
        self.tokensOwed0.set_slots_getter(getter.clone());
        self.tokensOwed1.set_slots_getter(getter.clone())
    }
}
impl Position for PositionInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        128
    }
}
impl SlotsGetterSetter for PositionInfo {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
impl OracleObservation {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            __slot_getter: None,
            blockTimestamp: Primitive::from_position(slot + U256::from(0), 0),
            tickCumulative: Primitive::from_position(slot + U256::from(0), 4),
            secondsPerLiquidityCumulativeX128: Primitive::from_position(
                slot + U256::from(0),
                11,
            ),
            initialized: Primitive::from_position(slot + U256::from(0), 31),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
    pub fn value(self) -> U256 {
        match self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let slots = getter.get_slots(self.__slot, 1);
                slots[0]
            }
        }
    }
    pub fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter.clone());
        self.blockTimestamp.set_slots_getter(getter.clone());
        self.tickCumulative.set_slots_getter(getter.clone());
        self.secondsPerLiquidityCumulativeX128.set_slots_getter(getter.clone());
        self.initialized.set_slots_getter(getter.clone())
    }
}
impl Position for OracleObservation {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl SlotsGetterSetter for OracleObservation {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
