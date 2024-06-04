use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::utils::{bytes32_to_u256, ceil_div, index_to_position, keccak256_concat, u256_to_bytes32, u256_to_u64};
use crate::types::{Value, DynamicArray};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};
use crate::types::keys::Key;

// The value corresponding to a mapping key k is located at keccak256(h(k) . p) where . is
// concatenation and h is a function that is applied to the key depending on its type:
// - for value types, h pads the value to 32 bytes in the same way as when storing the value in memory.
// - for strings and byte arrays, h computes the keccak256 hash of the unpadded data.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Mapping<KeyNativeType, ElementType> {
    __slot: U256,
    __marker: PhantomData<(KeyNativeType, ElementType)>,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<KeyNativeType: Key, ElementType> Mapping<KeyNativeType, ElementType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }

    fn new_element(&self, slot: U256, offset: usize) -> ElementType
        where ElementType: Position + SlotsGetterSetter,
    {
        let mut element = ElementType::from_position(slot, offset);
        match &self.__slots_getter {
            None => {
                // No slots getter to pass to children.
            }
            Some(getter) => {
                // Set child's slots getter.
                element.set_slots_getter(getter.clone());
            }
        }
        element
    }
    fn storage_at_bytes(&self, key: [u8; 32]) -> U256 {
        let value_slot_bytes = keccak256_concat(key, u256_to_bytes32(self.__slot));
        bytes32_to_u256(value_slot_bytes)
    }

    pub fn get_value_at(&self, key: KeyNativeType) -> Result<<ElementType as Value>::ValueType, String>
        where
            KeyNativeType: Key,
            ElementType: Position + Value + SlotsGetterSetter,
    {
        let getter = self.__slots_getter.as_ref().expect("No slots getter");
        let element_slot = self.storage_at_bytes(key.to_bytes());
        let element_size_slots = ceil_div(ElementType::size(), 32);
        let element_slot_values = getter.get_slots(element_slot, element_size_slots)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.new_element(element_slot, 0).get_value_from_slots_content(element_slot_values)
    }
}

impl<KeyNativeType, ElementType> Position for Mapping<KeyNativeType, ElementType> {
    fn from_position(slot: U256, _: usize) -> Self {
        Mapping::<KeyNativeType, ElementType> { __slot: slot, __marker: PhantomData, __slots_getter: None }
    }

    fn size() -> usize {
        32
    }
}


impl<KeyNativeType, ElementType> Mapping<KeyNativeType, ElementType> {
    pub fn at(&self, key: KeyNativeType) -> ElementType
        where
            KeyNativeType: Key,
            ElementType: Position + SlotsGetterSetter,
    {
        let element_slot = self.storage_at_bytes(key.to_bytes());
        self.new_element(element_slot, 0)
    }
}

impl<KeyNativeType: Debug, ElementType: Debug> SlotsGetterSetter for Mapping<KeyNativeType, ElementType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter);
    }
}

impl<KeyNativeType, ElementType> Value for Mapping<KeyNativeType, ElementType> {
    type ValueType = Self;

    fn get_value_from_slots_content(&self, _: Vec<U256>) -> Result<Self::ValueType, String> {
        // Mapping base slot is always empty and is used only for element storage calculation.
        // Clone self.
        Ok(Mapping {
            __slot: self.__slot,
            __marker: self.__marker,
            __slots_getter: self.__slots_getter.clone(),
        })
    }
}
