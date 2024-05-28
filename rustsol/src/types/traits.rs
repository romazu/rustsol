use primitive_types::U256;

pub trait Position {
    fn from_position(slot: U256, offset: u8) -> Self;
    fn size() -> u64;
}
