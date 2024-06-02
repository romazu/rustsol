use std::sync::Arc;
use alloy_primitives::{FixedBytes, U256};
use derivative::Derivative;
use crate::types::{Bytes, Primitive, Value};
use crate::types::{Position, SlotsGetter, SlotsGetterSetter};

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Address {
    __slot: U256,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl Address {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 20)
    }

    pub fn value(self) -> Result<alloy_primitives::Address, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let slot_values = getter.get_slots(self.__slot, 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        self.value_from_slots(slot_values)
    }
}

impl Position for Address {
    fn from_position(slot: U256, _: usize) -> Self {
        Address { __slot: slot, __slot_getter: None }  // Use the conversion from U256 to u64
    }

    fn size() -> usize {
        20
    }
}

impl SlotsGetterSetter for Address {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}

impl Value for Address {
    type ValueType = alloy_primitives::Address;

    fn value_from_slots(&self, slot_values: Vec<U256>) -> Result<Self::ValueType, String> {
        Ok(alloy_primitives::Address::from_word(FixedBytes::from(slot_values[0])))
    }
}
