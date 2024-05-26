use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Contract {
    __slot: U256,
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
#[allow(non_snake_case)]
pub struct UniswapV3PoolSlot0 {
    __slot: U256,
    pub sqrtPriceX96: Primitive<20>,
    pub tick: Primitive<3>,
    pub observationIndex: Primitive<2>,
    pub observationCardinality: Primitive<2>,
    pub observationCardinalityNext: Primitive<2>,
    pub feeProtocol: Primitive<1>,
    pub unlocked: Primitive<1>,
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UniswapV3PoolProtocolFees {
    __slot: U256,
    pub token0: Primitive<16>,
    pub token1: Primitive<16>,
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct TickInfo {
    __slot: U256,
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
#[allow(non_snake_case)]
pub struct PositionInfo {
    __slot: U256,
    pub liquidity: Primitive<16>,
    pub feeGrowthInside0LastX128: Primitive<32>,
    pub feeGrowthInside1LastX128: Primitive<32>,
    pub tokensOwed0: Primitive<16>,
    pub tokensOwed1: Primitive<16>,
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OracleObservation {
    __slot: U256,
    pub blockTimestamp: Primitive<4>,
    pub tickCumulative: Primitive<7>,
    pub secondsPerLiquidityCumulativeX128: Primitive<20>,
    pub initialized: Primitive<1>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            slot0: UniswapV3PoolSlot0::from_position(slot + 0u64, 0u8),
            feeGrowthGlobal0X128: Primitive::from_position(slot + 1u64, 0u8),
            feeGrowthGlobal1X128: Primitive::from_position(slot + 2u64, 0u8),
            protocolFees: UniswapV3PoolProtocolFees::from_position(slot + 3u64, 0u8),
            liquidity: Primitive::from_position(slot + 4u64, 0u8),
            ticks: Mapping::from_position(slot + 5u64, 0u8),
            tickBitmap: Mapping::from_position(slot + 6u64, 0u8),
            positions: Mapping::from_position(slot + 7u64, 0u8),
            observations: StaticArray::from_position(slot + 8u64, 0u8),
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
impl UniswapV3PoolSlot0 {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            sqrtPriceX96: Primitive::from_position(slot + 0u64, 0u8),
            tick: Primitive::from_position(slot + 0u64, 20u8),
            observationIndex: Primitive::from_position(slot + 0u64, 23u8),
            observationCardinality: Primitive::from_position(slot + 0u64, 25u8),
            observationCardinalityNext: Primitive::from_position(slot + 0u64, 27u8),
            feeProtocol: Primitive::from_position(slot + 0u64, 29u8),
            unlocked: Primitive::from_position(slot + 0u64, 30u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        32u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32u64)
    }
}
impl Position for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
impl UniswapV3PoolProtocolFees {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            token0: Primitive::from_position(slot + 0u64, 0u8),
            token1: Primitive::from_position(slot + 0u64, 16u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        32u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32u64)
    }
}
impl Position for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
impl TickInfo {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            liquidityGross: Primitive::from_position(slot + 0u64, 0u8),
            liquidityNet: Primitive::from_position(slot + 0u64, 16u8),
            feeGrowthOutside0X128: Primitive::from_position(slot + 1u64, 0u8),
            feeGrowthOutside1X128: Primitive::from_position(slot + 2u64, 0u8),
            tickCumulativeOutside: Primitive::from_position(slot + 3u64, 0u8),
            secondsPerLiquidityOutsideX128: Primitive::from_position(slot + 3u64, 7u8),
            secondsOutside: Primitive::from_position(slot + 3u64, 27u8),
            initialized: Primitive::from_position(slot + 3u64, 31u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        128u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 128u64)
    }
}
impl Position for TickInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
impl PositionInfo {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            liquidity: Primitive::from_position(slot + 0u64, 0u8),
            feeGrowthInside0LastX128: Primitive::from_position(slot + 1u64, 0u8),
            feeGrowthInside1LastX128: Primitive::from_position(slot + 2u64, 0u8),
            tokensOwed0: Primitive::from_position(slot + 3u64, 0u8),
            tokensOwed1: Primitive::from_position(slot + 3u64, 16u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        128u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 128u64)
    }
}
impl Position for PositionInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
impl OracleObservation {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            blockTimestamp: Primitive::from_position(slot + 0u64, 0u8),
            tickCumulative: Primitive::from_position(slot + 0u64, 4u8),
            secondsPerLiquidityCumulativeX128: Primitive::from_position(
                slot + 0u64,
                11u8,
            ),
            initialized: Primitive::from_position(slot + 0u64, 31u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn size() -> u64 {
        32u64
    }
    fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32u64)
    }
}
impl Position for OracleObservation {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        Self::size()
    }
}
