use std::fmt::Debug;
use std::sync::Arc;
use alloy_primitives::U256;
use crate::types::{Mapping, SlotsGetterSetter};
use crate::types::traits::{Position, SlotsGetter, Value};
use derivative::Derivative;
use crate::utils::vec_u256_to_vec_bytes;

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Primitive<const SIZE: usize> {
    __slot: U256,
    __offset: usize,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<const SIZE: usize> Primitive<SIZE> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn offset(&self) -> usize {
        self.__offset
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, self.__offset, SIZE)
    }

    pub fn value(self) -> Result<U256, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let slot_values = getter.get_slots(self.__slot, 1)?;
        let end = 32 - self.__offset as usize;
        let start = end - SIZE as usize;
        let element_bytes = &vec_u256_to_vec_bytes(&slot_values, 0, 1)[start..end];
        let value = self.value_from_base_bytes(element_bytes)?;
        Ok(value)
    }
}

impl<const SIZE: usize> Position for Primitive<SIZE> {
    fn from_position(slot: U256, offset: usize) -> Self {
        Primitive { __slot: slot, __offset: offset, __slot_getter: None }  // Use the conversion from U256 to u64
    }

    fn size() -> usize {
        SIZE
    }
}

impl<const SIZE: usize> SlotsGetterSetter for Primitive<SIZE> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}

impl<const SIZE: usize> Value for Primitive<SIZE> {
    // TODO: Change to concrete values type like u64 and bool.
    type ValueType = U256;

    fn value_from_base_bytes(&self, bytes: &[u8]) -> Result<Self::ValueType, String> {
        Ok(U256::from_be_slice(bytes))
    }
}
