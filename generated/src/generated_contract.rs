use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, PrimitiveKey, BytesKey, Position,
};
use primitive_types::U256;
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Contract {
    __slot: U256,
    pub plainUint112: Primitive<14>,
    pub plainUint32: Primitive<4>,
    pub plainString: Bytes,
    pub myStructNested: MyContractMyStructNested,
    pub dynamicArray: DynamicArray<Primitive<32>>,
    pub dynamicArrayStruct: DynamicArray<MyContractMyStructNested>,
    pub myMapping1: Mapping<PrimitiveKey, Primitive<32>>,
    pub myMapping2: Mapping<BytesKey, Primitive<32>>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive<32>>>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            plainUint112: Primitive::from_position(slot + 0u64, 0u8),
            plainUint32: Primitive::from_position(slot + 0u64, 14u8),
            plainString: Bytes::from_position(slot + 1u64, 0u8),
            myStructNested: MyContractMyStructNested::from_position(slot + 2u64, 0u8),
            dynamicArray: DynamicArray::from_position(slot + 10u64, 0u8),
            dynamicArrayStruct: DynamicArray::from_position(slot + 11u64, 0u8),
            myMapping1: Mapping::from_position(slot + 12u64, 0u8),
            myMapping2: Mapping::from_position(slot + 13u64, 0u8),
            myNestedMapping: Mapping::from_position(slot + 14u64, 0u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl Position for Contract {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        todo!()
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStructNested {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myStruct: MyContractMyStruct,
}
impl MyContractMyStructNested {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0u64, 0u8),
            myStruct: MyContractMyStruct::from_position(slot + 1u64, 0u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl Position for MyContractMyStructNested {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        todo!()
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStruct {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myUint: Primitive<32>,
}
impl MyContractMyStruct {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0u64, 0u8),
            myUint: Primitive::from_position(slot + 1u64, 0u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl Position for MyContractMyStruct {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        todo!()
    }
}
