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
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            slot0: UniswapV3PoolSlot0::from_position(slot + 0u64, U256::from(0u64)),
            feeGrowthGlobal0X128: Primitive::from_position(
                slot + 1u64,
                U256::from(0u64),
            ),
            feeGrowthGlobal1X128: Primitive::from_position(
                slot + 2u64,
                U256::from(0u64),
            ),
            protocolFees: UniswapV3PoolProtocolFees::from_position(
                slot + 3u64,
                U256::from(0u64),
            ),
            liquidity: Primitive::from_position(slot + 4u64, U256::from(0u64)),
            ticks: Mapping::from_position(slot + 5u64, U256::from(0u64)),
            tickBitmap: Mapping::from_position(slot + 6u64, U256::from(0u64)),
            positions: Mapping::from_position(slot + 7u64, U256::from(0u64)),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for Contract {
    fn from_position(slot: U256, offset: U256) -> Self {
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
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            sqrtPriceX96: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            tick: Primitive::from_position(slot + 0u64, U256::from(20u64)),
            observationIndex: Primitive::from_position(slot + 0u64, U256::from(23u64)),
            observationCardinality: Primitive::from_position(
                slot + 0u64,
                U256::from(25u64),
            ),
            observationCardinalityNext: Primitive::from_position(
                slot + 0u64,
                U256::from(27u64),
            ),
            feeProtocol: Primitive::from_position(slot + 0u64, U256::from(29u64)),
            unlocked: Primitive::from_position(slot + 0u64, U256::from(30u64)),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for UniswapV3PoolSlot0 {
    fn from_position(slot: U256, offset: U256) -> Self {
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
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            token0: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            token1: Primitive::from_position(slot + 0u64, U256::from(16u64)),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for UniswapV3PoolProtocolFees {
    fn from_position(slot: U256, offset: U256) -> Self {
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
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            liquidityGross: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            liquidityNet: Primitive::from_position(slot + 0u64, U256::from(16u64)),
            feeGrowthOutside0X128: Primitive::from_position(
                slot + 1u64,
                U256::from(0u64),
            ),
            feeGrowthOutside1X128: Primitive::from_position(
                slot + 2u64,
                U256::from(0u64),
            ),
            tickCumulativeOutside: Primitive::from_position(
                slot + 3u64,
                U256::from(0u64),
            ),
            secondsPerLiquidityOutsideX128: Primitive::from_position(
                slot + 3u64,
                U256::from(7u64),
            ),
            secondsOutside: Primitive::from_position(slot + 3u64, U256::from(27u64)),
            initialized: Primitive::from_position(slot + 3u64, U256::from(31u64)),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for TickInfo {
    fn from_position(slot: U256, offset: U256) -> Self {
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
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            liquidity: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            feeGrowthInside0LastX128: Primitive::from_position(
                slot + 1u64,
                U256::from(0u64),
            ),
            feeGrowthInside1LastX128: Primitive::from_position(
                slot + 2u64,
                U256::from(0u64),
            ),
            tokensOwed0: Primitive::from_position(slot + 3u64, U256::from(0u64)),
            tokensOwed1: Primitive::from_position(slot + 3u64, U256::from(16u64)),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl FromPosition for PositionInfo {
    fn from_position(slot: U256, offset: U256) -> Self {
        Self::new_from_position(slot, offset)
    }
}
