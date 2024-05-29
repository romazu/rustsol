#![allow(non_snake_case, unused, dead_code, unused_imports)]
use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
};
use primitive_types::U256;
#[derive(Debug)]
pub struct MyContract {
    __slot: U256,
    pub plainUint112: Primitive<14>,
    pub plainUint32: Primitive<4>,
    pub plainString: Bytes,
    pub myStructNested: MyContractMyStructNested,
    pub staticArray: StaticArray<160, Primitive<14>>,
    pub staticArrayLarge: StaticArray<128, MyContractMyStruct>,
    pub staticArrayNestedSmall: StaticArray<128, StaticArray<32, Primitive<1>>>,
    pub dynamicArray: DynamicArray<Primitive<32>>,
    pub dynamicArrayStruct: DynamicArray<MyContractMyStructNested>,
    pub dynamicArraySmall: DynamicArray<MyContractMyStructSmall>,
    pub myMapping1: Mapping<PrimitiveKey, Primitive<32>>,
    pub myMapping2: Mapping<BytesKey, Primitive<32>>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive<32>>>,
}
#[derive(Debug)]
pub struct MyContractMyStructNested {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myStruct: MyContractMyStruct,
}
#[derive(Debug)]
pub struct MyContractMyStruct {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myUint: Primitive<32>,
}
#[derive(Debug)]
pub struct MyContractMyStructSmall {
    __slot: U256,
    pub smallInt1: Primitive<4>,
    pub smallInt2: Primitive<4>,
}
impl MyContract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            plainUint112: Primitive::from_position(slot + 0, 0),
            plainUint32: Primitive::from_position(slot + 0, 14),
            plainString: Bytes::from_position(slot + 1, 0),
            myStructNested: MyContractMyStructNested::from_position(slot + 2, 0),
            staticArray: StaticArray::from_position(slot + 5, 0),
            staticArrayLarge: StaticArray::from_position(slot + 10, 0),
            staticArrayNestedSmall: StaticArray::from_position(slot + 14, 0),
            dynamicArray: DynamicArray::from_position(slot + 18, 0),
            dynamicArrayStruct: DynamicArray::from_position(slot + 19, 0),
            dynamicArraySmall: DynamicArray::from_position(slot + 20, 0),
            myMapping1: Mapping::from_position(slot + 21, 0),
            myMapping2: Mapping::from_position(slot + 22, 0),
            myNestedMapping: Mapping::from_position(slot + 23, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 0)
    }
}
impl Position for MyContract {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        0
    }
}
impl MyContractMyStructNested {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0, 0),
            myStruct: MyContractMyStruct::from_position(slot + 1, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 96)
    }
}
impl Position for MyContractMyStructNested {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        96
    }
}
impl MyContractMyStruct {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0, 0),
            myUint: Primitive::from_position(slot + 1, 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 64)
    }
}
impl Position for MyContractMyStruct {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        64
    }
}
impl MyContractMyStructSmall {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            smallInt1: Primitive::from_position(slot + 0, 0),
            smallInt2: Primitive::from_position(slot + 0, 4),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }
}
impl Position for MyContractMyStructSmall {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32
    }
}
