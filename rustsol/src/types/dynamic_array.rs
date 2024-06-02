use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use derivative::Derivative;
use crate::types::{Position, Primitive};
use crate::types::{SlotsGetter, SlotsGetterSetter, Value};
use crate::utils::{bytes32_to_u256, ceil_div, index_to_position, keccak256, u256_to_bytes32, u256_to_u64, vec_u256_to_vec_bytes};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct DynamicArray<ElementType>
    where ElementType: Debug + Value + Position {
    __slot: U256,
    __marker: PhantomData<ElementType>,
    #[derivative(Debug = "ignore")]
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<ElementType: Debug + Position + Value + SlotsGetterSetter> DynamicArray<ElementType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, usize, usize) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
    }

    // Return the packing ratio: (n, d).
    // This means that packing is "n slot per d elements"
    // For dynamic array d is always one, i.e., values are not tightly packed.
    pub fn packing_ratio(&self) -> (usize, usize) {
        (ceil_div(ElementType::size(), 32), 1)
    }

    pub fn value(&self) -> Result<Vec<<ElementType as Value>::ValueType>, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let base_slot_value = getter.get_slots(U256::from(self.__slot), 1)
            .map_err(|err| format!("Failed to get slot values: {}", err))?[0];
        self.value_from_base_bytes(&u256_to_bytes32(base_slot_value))
    }
}

impl<ElementType: Debug + Position + Value> Position for DynamicArray<ElementType> {
    fn from_position(slot: U256, _: usize) -> Self {
        DynamicArray::<ElementType> { __slot: slot, __marker: PhantomData, __slot_getter: None }
    }

    fn size() -> usize {
        32
    }
}

impl<ElementType: Debug + Position + Value> DynamicArray<ElementType> {
    fn new_element(&self, slot: U256, offset: usize) -> ElementType
        where ElementType: Debug + Position + Value + SlotsGetterSetter,
    {
        let mut element = ElementType::from_position(slot, offset);
        match &self.__slot_getter {
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

    pub fn get(&self, index: usize) -> ElementType
        where ElementType: Debug + Position + Value + SlotsGetterSetter,
    {
        let (packing_n, packing_d) = self.packing_ratio(); // Currently in solidity always ratio_d == 1.
        let (index_slot, index_offset) = index_to_position(index, packing_n, packing_d);
        self.new_element(self.storage() + index_slot, index_offset)
    }
}

impl<ElementType: Debug + Position + Value> SlotsGetterSetter for DynamicArray<ElementType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}

impl<ElementType: Debug + Position + Value + SlotsGetterSetter> Value for DynamicArray<ElementType> {
    type ValueType = Vec<<ElementType as Value>::ValueType>;

    fn value_from_base_bytes(&self, bytes: &[u8]) -> Result<Self::ValueType, String> {
        let getter = self.__slot_getter.as_ref().expect("No slots getter");
        let array_len = u256_to_u64(U256::from_be_slice(bytes)); // TODO: More effective conversion?
        let (packing_n, packing_d) = self.packing_ratio();
        let array_size_slots = array_len as usize * packing_n / packing_d;
        let slot_values = getter.get_slots(self.storage(), array_size_slots as usize)
            .map_err(|err| format!("Failed to get slot values: {}", err))?;
        let mut values = Vec::new();
        // These zeros are enough because we need element only to get its value.
        let element = self.new_element(U256::ZERO, 0);
        for i in 0..array_len as usize {
            // Simple assumption (holds in Solidity) that element occupying several slots cannot have offset.
            // TODO: Write general element_size_slots estimator.
            let element_size_slots = packing_n;
            let (element_slot, element_offset) = index_to_position(i, packing_n, packing_d);
            // let element = self.new_element(storage_slot + element_slot, element_offset);
            let element_bytes = &vec_u256_to_vec_bytes(&slot_values, i, i + element_size_slots as usize)[element_offset as usize..];
            let value = element.value_from_base_bytes(element_bytes)?;
            values.push(value);
        }
        Ok(values)
    }
}
