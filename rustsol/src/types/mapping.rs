use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::utils::{bytes32_to_u256, keccak256_concat, u256_to_bytes32};
use crate::types::{PrimitiveKey, BytesKey, AddressKey};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

// The value corresponding to a mapping key k is located at keccak256(h(k) . p) where . is
// concatenation and h is a function that is applied to the key depending on its type:
// - for value types, h pads the value to 32 bytes in the same way as when storing the value in memory.
// - for strings and byte arrays, h computes the keccak256 hash of the unpadded data.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Mapping<KeyType, ElementType> {
    __slot: U256,
    __marker: PhantomData<(KeyType, ElementType)>,
    #[derivative(Debug = "ignore")]
    __slots_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<KeyType, ElementType> Mapping<KeyType, ElementType> {
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

    fn at_bytes_key(&self, key: [u8; 32]) -> ElementType
        where
            ElementType: Position + SlotsGetterSetter,
    {
        let value_slot_bytes = keccak256_concat(key, u256_to_bytes32(self.__slot));
        let element_slot = bytes32_to_u256(value_slot_bytes);
        self.new_element(element_slot, 0)
    }
}

impl<KeyType, ElementType> Position for Mapping<KeyType, ElementType> {
    fn from_position(slot: U256, _: usize) -> Self {
        Mapping::<KeyType, ElementType> { __slot: slot, __marker: PhantomData, __slots_getter: None }
    }

    fn size() -> usize {
        32
    }
}


impl<ElementType> Mapping<PrimitiveKey, ElementType> {
    pub fn at<T>(&self, key: T) -> ElementType
        where
            T: Into<PrimitiveKey>,
            ElementType: Position + SlotsGetterSetter,
    {
        self.at_bytes_key(key.into().0)
    }
}

impl<ElementType> Mapping<BytesKey, ElementType> {
    pub fn at<T>(&self, key: T) -> ElementType
        where
            T: Into<BytesKey>,
            ElementType: Position + SlotsGetterSetter,
    {
        self.at_bytes_key(key.into().0)
    }
}

impl<ElementType> Mapping<AddressKey, ElementType> {
    pub fn at<T>(&self, key: T) -> ElementType
        where
            T: Into<AddressKey>,
            ElementType: Position + SlotsGetterSetter,
    {
        self.at_bytes_key(key.into().0)
    }
}

impl<KeyType: Debug, ElementType: Debug> SlotsGetterSetter for Mapping<KeyType, ElementType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slots_getter = Some(getter);
    }
}
