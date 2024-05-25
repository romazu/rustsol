use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey};
#[derive(Default)]
#[allow(non_snake_case)]
pub struct MyContract {
    __slot: [u8; 32],
    pub slot0: UniswapV3Pool_Slot0,
    pub feeGrowthGlobal0X128: Primitive,
    pub feeGrowthGlobal1X128: Primitive,
    pub protocolFees: UniswapV3Pool_ProtocolFees,
    pub liquidity: Primitive,
    pub ticks: Mapping<PrimitiveKey, Tick_Info>,
    pub tickBitmap: Mapping<PrimitiveKey, Primitive>,
    pub positions: Mapping<PrimitiveKey, Position_Info>,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct UniswapV3Pool_Slot0 {
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
pub struct UniswapV3Pool_ProtocolFees {
    __slot: [u8; 32],
    pub token0: Primitive,
    pub token1: Primitive,
}
