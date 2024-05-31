#![allow(unused_imports, non_snake_case, unused, dead_code)]
use rustsol::types::Position;
use rustsol::types::{Primitive, Bytes, Address, Mapping, DynamicArray, StaticArray};
use rustsol::types::{PrimitiveKey, BytesKey, AddressKey};
use alloy_primitives::U256;
#[derive(Debug)]
pub struct UniswapV3Pool {
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
pub struct UniswapV3PoolProtocolFees {
    __slot: U256,
    pub token0: Primitive<16>,
    pub token1: Primitive<16>,
}
#[derive(Debug)]
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
pub struct PositionInfo {
    __slot: U256,
    pub liquidity: Primitive<16>,
    pub feeGrowthInside0LastX128: Primitive<32>,
    pub feeGrowthInside1LastX128: Primitive<32>,
    pub tokensOwed0: Primitive<16>,
    pub tokensOwed1: Primitive<16>,
}
#[derive(Debug)]
pub struct OracleObservation {
    __slot: U256,
    pub blockTimestamp: Primitive<4>,
    pub tickCumulative: Primitive<7>,
    pub secondsPerLiquidityCumulativeX128: Primitive<20>,
    pub initialized: Primitive<1>,
}
impl UniswapV3Pool {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
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
}
impl Position for UniswapV3Pool {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        0
    }
}
impl UniswapV3PoolSlot0 {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            sqrtPriceX96: Primitive::from_position(slot, 0),
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
}
impl Position for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl UniswapV3PoolProtocolFees {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
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
}
impl Position for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
impl TickInfo {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
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
}
impl Position for TickInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        128
    }
}
impl PositionInfo {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
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
}
impl Position for PositionInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        128
    }
}
impl OracleObservation {
    pub fn new() -> Self {
        Self::new_from_position(U256::ZERO, 0)
    }
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
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
}
impl Position for OracleObservation {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
