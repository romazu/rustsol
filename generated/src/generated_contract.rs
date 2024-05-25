use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey, FromPosition};
use primitive_types::U256;
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Contract {
    __slot: U256,
    pub slot0: UniswapV3PoolSlot0,
    pub feeGrowthGlobal0X128: Primitive,
    pub feeGrowthGlobal1X128: Primitive,
    pub protocolFees: UniswapV3PoolProtocolFees,
    pub liquidity: Primitive,
    pub ticks: Mapping<PrimitiveKey, TickInfo>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive>,
    pub positions: Mapping<PrimitiveKey, PositionInfo>,
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
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for Contract {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UniswapV3PoolSlot0 {
    __slot: U256,
    pub sqrtPriceX96: Primitive,
    pub tick: Primitive,
    pub observationIndex: Primitive,
    pub observationCardinality: Primitive,
    pub observationCardinalityNext: Primitive,
    pub feeProtocol: Primitive,
    pub unlocked: Primitive,
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
}
impl FromPosition for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UniswapV3PoolProtocolFees {
    __slot: U256,
    pub token0: Primitive,
    pub token1: Primitive,
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
}
impl FromPosition for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct TickInfo {
    __slot: U256,
    pub liquidityGross: Primitive,
    pub liquidityNet: Primitive,
    pub feeGrowthOutside0X128: Primitive,
    pub feeGrowthOutside1X128: Primitive,
    pub tickCumulativeOutside: Primitive,
    pub secondsPerLiquidityOutsideX128: Primitive,
    pub secondsOutside: Primitive,
    pub initialized: Primitive,
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
}
impl FromPosition for TickInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PositionInfo {
    __slot: U256,
    pub liquidity: Primitive,
    pub feeGrowthInside0LastX128: Primitive,
    pub feeGrowthInside1LastX128: Primitive,
    pub tokensOwed0: Primitive,
    pub tokensOwed1: Primitive,
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
}
impl FromPosition for PositionInfo {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
}
