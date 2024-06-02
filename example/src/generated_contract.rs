#![allow(unused_imports, non_snake_case, unused, dead_code)]
use std::sync::Arc;
use rustsol::types::Derivative;
use rustsol::types::{Position, SlotsGetter, SlotsGetterSetter, Value};
use rustsol::types::{Primitive, Bytes, Address, Mapping, DynamicArray, StaticArray};
use rustsol::types::{PrimitiveKey, BytesKey, AddressKey};
use alloy_primitives::U256;
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MyContract {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub plainUint112: Primitive<14>,
    pub dynamicArray: DynamicArray<Primitive<32>>,
    pub dynamicArrayNested: DynamicArray<DynamicArray<Primitive<32>>>,
    pub plainUint32: Primitive<4>,
    pub plainAddress: Address,
    pub myStructNested: MyContractMyStructNested,
    pub staticArray: StaticArray<160, Primitive<14>>,
    pub staticArrayLarge: StaticArray<128, MyContractMyStruct>,
    pub staticArrayNestedSmall: StaticArray<128, StaticArray<32, Primitive<1>>>,
    pub dynamicArrayStruct: DynamicArray<MyContractMyStructNested>,
    pub dynamicArraySmall: DynamicArray<MyContractMyStructSmall>,
    pub myMapping1: Mapping<PrimitiveKey, Primitive<32>>,
    pub myMapping2: Mapping<BytesKey, Primitive<32>>,
    pub myMappingBool: Mapping<PrimitiveKey, Primitive<1>>,
    pub myAddressMappingNested: Mapping<AddressKey, Mapping<AddressKey, Address>>,
    pub myNestedMapping: Mapping<PrimitiveKey, Mapping<PrimitiveKey, Primitive<32>>>,
    pub myEnum: Primitive<1>,
    pub ___gap___: StaticArray<1248, Primitive<32>>,
    pub plainString: Bytes,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MyContractMyStructNested {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub myAddress: Address,
    pub myStruct: MyContractMyStruct,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MyContractMyStruct {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub myAddress: Address,
    pub myUint: Primitive<32>,
}
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MyContractMyStructSmall {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
    pub smallInt1: Primitive<4>,
    pub smallInt2: Primitive<4>,
}
impl MyContract {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            plainUint112: Primitive::from_position(slot + U256::from(0), 0),
            dynamicArray: DynamicArray::from_position(slot + U256::from(1), 0),
            dynamicArrayNested: DynamicArray::from_position(slot + U256::from(2), 0),
            plainUint32: Primitive::from_position(slot + U256::from(3), 0),
            plainAddress: Address::from_position(slot + U256::from(3), 4),
            myStructNested: MyContractMyStructNested::from_position(
                slot + U256::from(4),
                0,
            ),
            staticArray: StaticArray::from_position(slot + U256::from(7), 0),
            staticArrayLarge: StaticArray::from_position(slot + U256::from(12), 0),
            staticArrayNestedSmall: StaticArray::from_position(slot + U256::from(16), 0),
            dynamicArrayStruct: DynamicArray::from_position(slot + U256::from(20), 0),
            dynamicArraySmall: DynamicArray::from_position(slot + U256::from(21), 0),
            myMapping1: Mapping::from_position(slot + U256::from(22), 0),
            myMapping2: Mapping::from_position(slot + U256::from(23), 0),
            myMappingBool: Mapping::from_position(slot + U256::from(24), 0),
            myAddressMappingNested: Mapping::from_position(slot + U256::from(25), 0),
            myNestedMapping: Mapping::from_position(slot + U256::from(26), 0),
            myEnum: Primitive::from_position(slot + U256::from(27), 0),
            ___gap___: StaticArray::from_position(slot + U256::from(28), 0),
            plainString: Bytes::from_position(slot + U256::from(67), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 0)
    }
    pub fn value(&self) -> Result<<Self as Value>::ValueType, String> {
        panic!("Not implemented")
    }
}
impl Position for MyContract {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        0
    }
}
impl SlotsGetterSetter for MyContract {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.plainUint112.set_slots_getter(getter.clone());
        self.dynamicArray.set_slots_getter(getter.clone());
        self.dynamicArrayNested.set_slots_getter(getter.clone());
        self.plainUint32.set_slots_getter(getter.clone());
        self.plainAddress.set_slots_getter(getter.clone());
        self.myStructNested.set_slots_getter(getter.clone());
        self.staticArray.set_slots_getter(getter.clone());
        self.staticArrayLarge.set_slots_getter(getter.clone());
        self.staticArrayNestedSmall.set_slots_getter(getter.clone());
        self.dynamicArrayStruct.set_slots_getter(getter.clone());
        self.dynamicArraySmall.set_slots_getter(getter.clone());
        self.myMapping1.set_slots_getter(getter.clone());
        self.myMapping2.set_slots_getter(getter.clone());
        self.myMappingBool.set_slots_getter(getter.clone());
        self.myAddressMappingNested.set_slots_getter(getter.clone());
        self.myNestedMapping.set_slots_getter(getter.clone());
        self.myEnum.set_slots_getter(getter.clone());
        self.___gap___.set_slots_getter(getter.clone());
        self.plainString.set_slots_getter(getter.clone())
    }
}
impl Value for MyContract {
    type ValueType = U256;
    fn value_from_slots(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        panic!("Not implemented")
    }
}
impl MyContractMyStructNested {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            myAddress: Address::from_position(slot + U256::from(0), 0),
            myStruct: MyContractMyStruct::from_position(slot + U256::from(1), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 96)
    }
    pub fn value(&self) -> Result<<Self as Value>::ValueType, String> {
        panic!("Not implemented")
    }
}
impl Position for MyContractMyStructNested {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        96
    }
}
impl SlotsGetterSetter for MyContractMyStructNested {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.myAddress.set_slots_getter(getter.clone());
        self.myStruct.set_slots_getter(getter.clone())
    }
}
impl Value for MyContractMyStructNested {
    type ValueType = U256;
    fn value_from_slots(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        panic!("Not implemented")
    }
}
impl MyContractMyStruct {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            myAddress: Address::from_position(slot + U256::from(0), 0),
            myUint: Primitive::from_position(slot + U256::from(1), 0),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 64)
    }
    pub fn value(&self) -> Result<<Self as Value>::ValueType, String> {
        panic!("Not implemented")
    }
}
impl Position for MyContractMyStruct {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        64
    }
}
impl SlotsGetterSetter for MyContractMyStruct {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.myAddress.set_slots_getter(getter.clone());
        self.myUint.set_slots_getter(getter.clone())
    }
}
impl Value for MyContractMyStruct {
    type ValueType = U256;
    fn value_from_slots(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        panic!("Not implemented")
    }
}
impl MyContractMyStructSmall {
    pub fn new() -> Self {
        Self::from_position(U256::ZERO, 0)
    }
    pub fn from_position(slot: U256, offset: usize) -> Self {
        Self {
            __slot: slot,
            __slots_getter: None,
            smallInt1: Primitive::from_position(slot + U256::from(0), 0),
            smallInt2: Primitive::from_position(slot + U256::from(0), 4),
        }
    }
    pub fn slot(&self) -> U256 {
        self.__slot
    }
    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }
    pub fn value(&self) -> Result<<Self as Value>::ValueType, String> {
        panic!("Not implemented")
    }
}
impl Position for MyContractMyStructSmall {
    fn from_position(slot: U256, offset: usize) -> Self {
        Self::from_position(slot, offset)
    }
    fn size() -> usize {
        32
    }
}
impl SlotsGetterSetter for MyContractMyStructSmall {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter.clone());
        self.smallInt1.set_slots_getter(getter.clone());
        self.smallInt2.set_slots_getter(getter.clone())
    }
}
impl Value for MyContractMyStructSmall {
    type ValueType = U256;
    fn value_from_slots(
        &self,
        slot_values: Vec<U256>,
    ) -> Result<Self::ValueType, String> {
        panic!("Not implemented")
    }
}
