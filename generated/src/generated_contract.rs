use rustsol::types::{
    Primitive, Bytes, Mapping, DynamicArray, StaticArray, PrimitiveKey, BytesKey,
    Position,
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
#[allow(non_snake_case)]
pub struct MyContractMyStructNested {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myStruct: MyContractMyStruct,
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStruct {
    __slot: U256,
    pub myAddress: Primitive<20>,
    pub myUint: Primitive<32>,
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStructSmall {
    __slot: U256,
    pub smallInt1: Primitive<4>,
    pub smallInt2: Primitive<4>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            plainUint112: Primitive::from_position(slot + 0u64, 0u8),
            plainUint32: Primitive::from_position(slot + 0u64, 14u8),
            plainString: Bytes::from_position(slot + 1u64, 0u8),
            myStructNested: MyContractMyStructNested::from_position(slot + 2u64, 0u8),
            staticArray: StaticArray::from_position(slot + 5u64, 0u8),
            staticArrayLarge: StaticArray::from_position(slot + 10u64, 0u8),
            staticArrayNestedSmall: StaticArray::from_position(slot + 14u64, 0u8),
            dynamicArray: DynamicArray::from_position(slot + 18u64, 0u8),
            dynamicArrayStruct: DynamicArray::from_position(slot + 19u64, 0u8),
            dynamicArraySmall: DynamicArray::from_position(slot + 20u64, 0u8),
            myMapping1: Mapping::from_position(slot + 21u64, 0u8),
            myMapping2: Mapping::from_position(slot + 22u64, 0u8),
            myNestedMapping: Mapping::from_position(slot + 23u64, 0u8),
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
        0u64
    }
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
        96u64
    }
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
        64u64
    }
}
impl MyContractMyStructSmall {
    pub fn new_from_position(slot: U256, offset: u8) -> Self {
        Self {
            __slot: slot,
            smallInt1: Primitive::from_position(slot + 0u64, 0u8),
            smallInt2: Primitive::from_position(slot + 0u64, 4u8),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
}
impl Position for MyContractMyStructSmall {
    fn from_position(slot: U256, offset: u8) -> Self {
        Self::new_from_position(slot, offset)
    }
    fn size() -> u64 {
        32u64
    }
}
