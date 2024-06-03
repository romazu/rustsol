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
pub struct UniswapV3Pool {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
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
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UniswapV3PoolSlot0 {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub sqrtPriceX96: Primitive<20>,
    pub tick: Primitive<3>,
    pub observationIndex: Primitive<2>,
    pub observationCardinality: Primitive<2>,
    pub observationCardinalityNext: Primitive<2>,
    pub feeProtocol: Primitive<1>,
    pub unlocked: Primitive<1>,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct UniswapV3PoolProtocolFees {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub token0: Primitive<16>,
    pub token1: Primitive<16>,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TickInfo {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub liquidityGross: Primitive<16>,
    pub liquidityNet: Primitive<16>,
    pub feeGrowthOutside0X128: Primitive<32>,
    pub feeGrowthOutside1X128: Primitive<32>,
    pub tickCumulativeOutside: Primitive<7>,
    pub secondsPerLiquidityOutsideX128: Primitive<20>,
    pub secondsOutside: Primitive<4>,
    pub initialized: Primitive<1>,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct PositionInfo {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub liquidity: Primitive<16>,
    pub feeGrowthInside0LastX128: Primitive<32>,
    pub feeGrowthInside1LastX128: Primitive<32>,
    pub tokensOwed0: Primitive<16>,
    pub tokensOwed1: Primitive<16>,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct OracleObservation {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub blockTimestamp: Primitive<4>,
    pub tickCumulative: Primitive<7>,
    pub secondsPerLiquidityCumulativeX128: Primitive<20>,
    pub initialized: Primitive<1>,
}
impl UniswapV3Pool {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
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
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 2097376)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 65543)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for UniswapV3Pool {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        2097376
    }
}
impl SlotsGetterSetter for UniswapV3Pool {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
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
#[derive(Debug)]
pub struct UniswapV3PoolValue {
    pub slot0: UniswapV3PoolSlot0Value,
    pub feeGrowthGlobal0X128: U256,
    pub feeGrowthGlobal1X128: U256,
    pub protocolFees: UniswapV3PoolProtocolFeesValue,
    pub liquidity: U256,
    pub ticks: Mapping<PrimitiveKey, TickInfo>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive<32>>,
    pub positions: Mapping<PrimitiveKey, PositionInfo>,
    pub observations: Vec<OracleObservationValue>,
}
impl Value for UniswapV3Pool {
    type ValueType = UniswapV3PoolValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(UniswapV3PoolValue {
            slot0: self.slot0.get_value_from_slots_content(slot_values[0..1].to_vec())?,
            feeGrowthGlobal0X128: self
                .feeGrowthGlobal0X128
                .get_value_from_slots_content(slot_values[1..2].to_vec())?,
            feeGrowthGlobal1X128: self
                .feeGrowthGlobal1X128
                .get_value_from_slots_content(slot_values[2..3].to_vec())?,
            protocolFees: self
                .protocolFees
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            liquidity: self
                .liquidity
                .get_value_from_slots_content(slot_values[4..5].to_vec())?,
            ticks: self.ticks.get_value_from_slots_content(slot_values[5..6].to_vec())?,
            tickBitmap: self
                .tickBitmap
                .get_value_from_slots_content(slot_values[6..7].to_vec())?,
            positions: self
                .positions
                .get_value_from_slots_content(slot_values[7..8].to_vec())?,
            observations: self
                .observations
                .get_value_from_slots_content(slot_values[8..65543].to_vec())?,
        })
    }
}
impl UniswapV3PoolSlot0 {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
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
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        32
    }
}
impl SlotsGetterSetter for UniswapV3PoolSlot0 {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.sqrtPriceX96.set_slots_getter(getter.clone());
        self.tick.set_slots_getter(getter.clone());
        self.observationIndex.set_slots_getter(getter.clone());
        self.observationCardinality.set_slots_getter(getter.clone());
        self.observationCardinalityNext.set_slots_getter(getter.clone());
        self.feeProtocol.set_slots_getter(getter.clone());
        self.unlocked.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct UniswapV3PoolSlot0Value {
    pub sqrtPriceX96: U256,
    pub tick: U256,
    pub observationIndex: U256,
    pub observationCardinality: U256,
    pub observationCardinalityNext: U256,
    pub feeProtocol: U256,
    pub unlocked: U256,
}
impl Value for UniswapV3PoolSlot0 {
    type ValueType = UniswapV3PoolSlot0Value;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(UniswapV3PoolSlot0Value {
            sqrtPriceX96: self
                .sqrtPriceX96
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            tick: self.tick.get_value_from_slots_content(slot_values[0..1].to_vec())?,
            observationIndex: self
                .observationIndex
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            observationCardinality: self
                .observationCardinality
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            observationCardinalityNext: self
                .observationCardinalityNext
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            feeProtocol: self
                .feeProtocol
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            unlocked: self
                .unlocked
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
        })
    }
}
impl UniswapV3PoolProtocolFees {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            token0: Primitive::from_position(slot + U256::from(0), 0),
            token1: Primitive::from_position(slot + U256::from(0), 16),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        32
    }
}
impl SlotsGetterSetter for UniswapV3PoolProtocolFees {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.token0.set_slots_getter(getter.clone());
        self.token1.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct UniswapV3PoolProtocolFeesValue {
    pub token0: U256,
    pub token1: U256,
}
impl Value for UniswapV3PoolProtocolFees {
    type ValueType = UniswapV3PoolProtocolFeesValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(UniswapV3PoolProtocolFeesValue {
            token0: self
                .token0
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            token1: self.token1.get_value_from_slots_content(slot_values[0..1].to_vec())?,
        })
    }
}
impl TickInfo {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
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
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 128)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 4)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for TickInfo {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        128
    }
}
impl SlotsGetterSetter for TickInfo {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
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
#[derive(Debug)]
pub struct TickInfoValue {
    pub liquidityGross: U256,
    pub liquidityNet: U256,
    pub feeGrowthOutside0X128: U256,
    pub feeGrowthOutside1X128: U256,
    pub tickCumulativeOutside: U256,
    pub secondsPerLiquidityOutsideX128: U256,
    pub secondsOutside: U256,
    pub initialized: U256,
}
impl Value for TickInfo {
    type ValueType = TickInfoValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(TickInfoValue {
            liquidityGross: self
                .liquidityGross
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            liquidityNet: self
                .liquidityNet
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            feeGrowthOutside0X128: self
                .feeGrowthOutside0X128
                .get_value_from_slots_content(slot_values[1..2].to_vec())?,
            feeGrowthOutside1X128: self
                .feeGrowthOutside1X128
                .get_value_from_slots_content(slot_values[2..3].to_vec())?,
            tickCumulativeOutside: self
                .tickCumulativeOutside
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            secondsPerLiquidityOutsideX128: self
                .secondsPerLiquidityOutsideX128
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            secondsOutside: self
                .secondsOutside
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            initialized: self
                .initialized
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
        })
    }
}
impl PositionInfo {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
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
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 128)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 4)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for PositionInfo {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        128
    }
}
impl SlotsGetterSetter for PositionInfo {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.liquidity.set_slots_getter(getter.clone());
        self.feeGrowthInside0LastX128.set_slots_getter(getter.clone());
        self.feeGrowthInside1LastX128.set_slots_getter(getter.clone());
        self.tokensOwed0.set_slots_getter(getter.clone());
        self.tokensOwed1.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct PositionInfoValue {
    pub liquidity: U256,
    pub feeGrowthInside0LastX128: U256,
    pub feeGrowthInside1LastX128: U256,
    pub tokensOwed0: U256,
    pub tokensOwed1: U256,
}
impl Value for PositionInfo {
    type ValueType = PositionInfoValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(PositionInfoValue {
            liquidity: self
                .liquidity
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            feeGrowthInside0LastX128: self
                .feeGrowthInside0LastX128
                .get_value_from_slots_content(slot_values[1..2].to_vec())?,
            feeGrowthInside1LastX128: self
                .feeGrowthInside1LastX128
                .get_value_from_slots_content(slot_values[2..3].to_vec())?,
            tokensOwed0: self
                .tokensOwed0
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
            tokensOwed1: self
                .tokensOwed1
                .get_value_from_slots_content(slot_values[3..4].to_vec())?,
        })
    }
}
impl OracleObservation {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
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
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }
    pub fn get_value(&self) -> Result<<Self as Value>::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let slot_values = getter
            .get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.get_value_from_slots_content(slot_values)
    }
}
impl Position for OracleObservation {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        32
    }
}
impl SlotsGetterSetter for OracleObservation {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.blockTimestamp.set_slots_getter(getter.clone());
        self.tickCumulative.set_slots_getter(getter.clone());
        self.secondsPerLiquidityCumulativeX128.set_slots_getter(getter.clone());
        self.initialized.set_slots_getter(getter.clone())
    }
}
#[derive(Debug)]
pub struct OracleObservationValue {
    pub blockTimestamp: U256,
    pub tickCumulative: U256,
    pub secondsPerLiquidityCumulativeX128: U256,
    pub initialized: U256,
}
impl Value for OracleObservation {
    type ValueType = OracleObservationValue;
    fn get_value_from_slots_content(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        Ok(OracleObservationValue {
            blockTimestamp: self
                .blockTimestamp
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            tickCumulative: self
                .tickCumulative
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            secondsPerLiquidityCumulativeX128: self
                .secondsPerLiquidityCumulativeX128
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
            initialized: self
                .initialized
                .get_value_from_slots_content(slot_values[0..1].to_vec())?,
        })
    }
}
