use primitive_types::U256;

trait Location {
    fn slot(&self) -> U256;
    fn offset(&self) -> U256;
    fn storage(&self) -> U256;
}

#[derive(Debug)]
pub struct Primitive {
    __slot: U256,
    __offset: U256,
}

impl Location for Primitive {
    fn slot(&self) -> U256 {
        self.__slot
    }

    fn offset(&self) -> U256 {
        self.__offset
    }

    fn storage(&self) -> U256 {
        self.__slot
    }
}

impl From<U256> for Primitive {
    fn from(u: U256) -> Self {
        Primitive{__slot: u, __offset: U256::zero()}  // Use the conversion from U256 to u64
    }
}
