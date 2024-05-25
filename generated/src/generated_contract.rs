use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey, FromPosition};
use primitive_types::U256;
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Contract {
    __slot: U256,
    pub plainUint112: Primitive,
    pub plainUint32: Primitive,
    pub plainString: Bytes,
    pub myStructNested: MyContractMyStructNested,
    pub myMapping1: Mapping<PrimitiveKey, Primitive>,
    pub myMapping2: Mapping<BytesKey, Primitive>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive>>,
}
impl Contract {
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            plainUint112: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            plainUint32: Primitive::from_position(slot + 0u64, U256::from(14u64)),
            plainString: Bytes::from_position(slot + 1u64, U256::from(0u64)),
            myStructNested: MyContractMyStructNested::from_position(
                slot + 2u64,
                U256::from(0u64),
            ),
            myMapping1: Mapping::from_position(slot + 11u64, U256::from(0u64)),
            myMapping2: Mapping::from_position(slot + 12u64, U256::from(0u64)),
            myNestedMapping: Mapping::from_position(slot + 13u64, U256::from(0u64)),
        }
    }
}
impl FromPosition for Contract {
    fn from_position(slot: U256, offset: U256) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStructNested {
    __slot: U256,
    pub myAddress: Primitive,
    pub myStruct: MyContractMyStruct,
}
impl MyContractMyStructNested {
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            myStruct: MyContractMyStruct::from_position(slot + 1u64, U256::from(0u64)),
        }
    }
}
impl FromPosition for MyContractMyStructNested {
    fn from_position(slot: U256, offset: U256) -> Self {
        Self::new_from_position(slot, offset)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MyContractMyStruct {
    __slot: U256,
    pub myAddress: Primitive,
    pub myUint: Primitive,
}
impl MyContractMyStruct {
    pub fn new_from_position(slot: U256, offset: U256) -> Self {
        Self {
            __slot: slot,
            myAddress: Primitive::from_position(slot + 0u64, U256::from(0u64)),
            myUint: Primitive::from_position(slot + 1u64, U256::from(0u64)),
        }
    }
}
impl FromPosition for MyContractMyStruct {
    fn from_position(slot: U256, offset: U256) -> Self {
        Self::new_from_position(slot, offset)
    }
}
