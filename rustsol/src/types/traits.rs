use alloy_primitives::U256;

pub trait Position {
    fn from_position(slot: U256, offset: u8) -> Self;
    fn size() -> u64;
}
