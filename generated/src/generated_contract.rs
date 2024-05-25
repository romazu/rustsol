use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey};
#[derive(Default)]
#[allow(non_snake_case)]
pub struct Contract {
    __slot: [u8; 32],
    pub slot0: UniswapV3PoolSlot0,
    pub feeGrowthGlobal0X128: Primitive,
    pub feeGrowthGlobal1X128: Primitive,
    pub protocolFees: UniswapV3PoolProtocolFees,
    pub liquidity: Primitive,
    pub ticks: Mapping<PrimitiveKey, TickInfo>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive>,
    pub positions: Mapping<PrimitiveKey, PositionInfo>,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct UniswapV3PoolSlot0 {
    __slot: [u8; 32],
    pub sqrtPriceX96: Primitive,
    pub tick: Primitive,
    pub observationIndex: Primitive,
    pub observationCardinality: Primitive,
    pub observationCardinalityNext: Primitive,
    pub feeProtocol: Primitive,
    pub unlocked: Primitive,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct UniswapV3PoolProtocolFees {
    __slot: [u8; 32],
    pub token0: Primitive,
    pub token1: Primitive,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct TickInfo {
    __slot: [u8; 32],
    pub liquidityGross: Primitive,
    pub liquidityNet: Primitive,
    pub feeGrowthOutside0X128: Primitive,
    pub feeGrowthOutside1X128: Primitive,
    pub tickCumulativeOutside: Primitive,
    pub secondsPerLiquidityOutsideX128: Primitive,
    pub secondsOutside: Primitive,
    pub initialized: Primitive,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct PositionInfo {
    __slot: [u8; 32],
    pub liquidity: Primitive,
    pub feeGrowthInside0LastX128: Primitive,
    pub feeGrowthInside1LastX128: Primitive,
    pub tokensOwed0: Primitive,
    pub tokensOwed1: Primitive,
}
