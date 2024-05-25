use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey};
#[derive(Default)]
#[allow(non_snake_case)]
pub struct MyContract {
    __slot: [u8; 32],
    pub plainUint112: Primitive,
    pub plainUint32: Primitive,
    pub plainString: Bytes,
    pub myStructNested: MyContract_MyStructNested,
    pub myMapping1: Mapping<PrimitiveKey, Primitive>,
    pub myMapping2: Mapping<BytesKey, Primitive>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive>>,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct MyContract_MyStructNested {
    __slot: [u8; 32],
    pub myAddress: Primitive,
    pub myStruct: MyContract_MyStruct,
}
#[derive(Default)]
#[allow(non_snake_case)]
pub struct MyContract_MyStruct {
    __slot: [u8; 32],
    pub myAddress: Primitive,
    pub myUint: Primitive,
}
