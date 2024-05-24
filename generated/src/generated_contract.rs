use rustsol::types::{Primitive, Bytes, Mapping, PrimitiveKey, BytesKey};
#[derive(Default)]
#[allow(non_snake_case)]
pub struct MyContract {
    pub plainUint112: Primitive,
    pub plainUint32: Primitive,
    pub plainString: Bytes,
    pub myMapping1: Mapping<PrimitiveKey, Primitive>,
    pub myMapping2: Mapping<BytesKey, Primitive>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive>>,
}
