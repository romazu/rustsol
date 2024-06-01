use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use alloy_primitives::U256;
use crate::types::{Position, Primitive, SlotsGetter, SlotsGetterSetter};
use crate::types::traits::Value;
use crate::utils::{bytes32_to_u256, ceil_div, index_to_position, keccak256, u256_to_bytes32, u256_to_u64, vec_u256_to_vec_bytes};

#[derive(Debug)]
pub struct DynamicArray<ElementType> {
    __slot: U256,
    __marker: PhantomData<ElementType>,
    __slot_getter: Option<Arc<dyn SlotsGetter>>,
}

impl<ElementType: Position+Value> DynamicArray<ElementType> {
    pub fn slot(&self) -> U256 {
        self.__slot
    }

    pub fn position(&self) -> (U256, u8, u64) {
        (self.__slot, 0, 32)
    }

    pub fn storage(&self) -> U256 {
        bytes32_to_u256(keccak256(u256_to_bytes32(self.__slot)))
    }

    // Return the packing ratio: (n, d).
    // This means that packing is "n slot per d elements"
    // For dynamic array d is always one, i.e., values are not tightly packed.
    pub fn packing_ratio(&self) -> (u64, u64) {
        (ceil_div(ElementType::size(), 32), 1)
    }

    pub fn value(&self) -> Result<Vec<<ElementType as Value>::ValueType>, String> {
        match &self.__slot_getter {
            None => panic!("No slots getter"),
            Some(getter) => {
                let base_slot_value = getter.get_slots(U256::from(self.__slot), 1)
                    .map_err(|err| format!("Failed to get slot values: {}", err))?[0];
                let array_len = u256_to_u64(base_slot_value);
                let (packing_n, packing_d) = self.packing_ratio();
                let array_size_slots = array_len * packing_n / packing_d;
                let slot_values = getter.get_slots(self.storage(), array_size_slots as usize)
                    .map_err(|err| format!("Failed to get slot values: {}", err))?;
                let mut values = Vec::new();
                for i in 0..array_len as usize {
                    // Simple assumption (holds in Solidity) that element occupying several slots cannot have offset.
                    // TODO: Write general element_size_slots estimator.
                    let element_size_slots = packing_n;
                    let (_, element_offset) = index_to_position(i, packing_n, packing_d);
                    let elements_bytes = &vec_u256_to_vec_bytes(&slot_values, i, i+element_size_slots as usize)[element_offset as usize..];
                    let value = ElementType::value_from_bytes(elements_bytes);
                    values.push(value);
                }
                Ok(values)
            },
        }
    }
}

impl<ElementType> Position for DynamicArray<ElementType> {
    fn from_position(slot: U256, _: u8) -> Self {
        DynamicArray::<ElementType> { __slot: slot, __marker: PhantomData, __slot_getter: None }
    }

    fn size() -> u64 {
        32
    }
}

impl<ElementType> DynamicArray<ElementType> {
    pub fn get(&self, index: usize) -> ElementType
        where
            ElementType: Position+Value,
    {
        let (packing_n, packing_d) = self.packing_ratio(); // Currently in solidity always ratio_d == 1.
        let (index_slot, index_offset) = index_to_position(index, packing_n, packing_d);
        ElementType::from_position(self.storage() + U256::from(index_slot), index_offset)
    }
}

impl<ElementType: Debug> SlotsGetterSetter for DynamicArray<ElementType> {
    fn set_slots_getter(&mut self, getter: Arc<dyn SlotsGetter>) {
        self.__slot_getter = Some(getter);
    }
}
